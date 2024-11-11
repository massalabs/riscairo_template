use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

mod cairo;
mod config;
mod rust;

use config::{Config, ProjectType};

type DynError = Box<dyn std::error::Error>;

fn main() {
    println!(
        "Working directory: {:?}",
        std::env::current_dir().unwrap()
    );

    let cfg = Config::new(
        std::env::current_dir().unwrap(),
        PathBuf::from("guest_rs"),
        PathBuf::from("."),
        ProjectType::New,
    );

    if let Err(e) = try_main(cfg) {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main(mut cfg: Config) -> Result<(), DynError> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("init") => init_all(&cfg),
        Some("init_template") => {
            cfg.set_project_type(ProjectType::Template);
            init_all(&cfg)
        }
        Some("init_rs") => rust::init(&cfg),
        Some("init_cairo") => cairo::init(&cfg),
        Some("build") => build_all(&cfg),
        Some("clean") => clean_all(&cfg),
        Some("build_rs") => {
            rust::build(&cfg);
            gen_bytecode(&cfg)
        }
        Some("build_cairo") => cairo::build(&cfg),
        Some("clean_rs") => rust::clean(&cfg),
        Some("clean_cairo") => cairo::clean(&cfg),
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "Tasks:

# All in one commands:
init                initializes a brand new empty project
init_template       initializes a brand new project based on a template
build               builds the whole project
clean               cleans the whole project

# Single task commands:
init_rs             initializes the rust project
init_cairo          initializes the cairo project

build_rs            builds only the rust project (and exports the binary to cairo)
build_cairo         builds only the cairo project

clean_rs            cleans only the rust project
clean_cairo         cleans only the cairo project
"
    )
}

fn gen_bytecode(cfg: &Config) {
    elf_to_bytecode(cfg.riscv_binary_path(), cfg.bytecode_path());
}

fn run_command(command: &str, args: &[&str], dir: &Path) {
    let status = Command::new(command)
        .current_dir(dir)
        .args(args)
        .status()
        .unwrap_or_else(|e| {
            panic!(
                "{} {} in {:?} failed with {}",
                command,
                args.join(" "),
                dir,
                e
            )
        });

    if !status.success() {
        panic!(
            "failed to execute {} {} in {:?}",
            command,
            args.join(" "),
            dir
        );
    }

    println!("{} {} in {:?} succeeded", command, args.join(" "), dir);
}
fn elf_to_bytecode(in_file_name: &Path, out_file_name: &Path) {
    fn write_bytecode(mut output_file: File, buffer: Vec<u8>) -> Result<(), DynError> {
        writeln!(
            output_file,
            "// This file is auto-generated. Do not modify."
        )?;
        write!(
            output_file,
            "pub const BYTECODE: [u8; {}] = [",
            buffer.len()
        )?;

        for &byte in &buffer {
            write!(output_file, "0x{:02x},", byte)?;
        }

        writeln!(output_file, "];")?;
        Ok(())
    }

    let buffer = fs::read(in_file_name)
        .unwrap_or_else(|e| panic!("failed to read file: {:?} with error {}", in_file_name, e));

    let output_file = File::create(out_file_name).unwrap_or_else(|e| {
        panic!(
            "failed to create file: {:?} with error {}",
            out_file_name, e
        )
    });

    write_bytecode(output_file, buffer).unwrap_or_else(|e| {
        panic!(
            "failed to write to file: {:?} with error {}",
            out_file_name, e
        )
    });

    println!("Converted\n\t{:?}\nto\n\t{:?}", in_file_name, out_file_name);
}

fn init_all(cfg: &Config) {
    rust::init(cfg);
    cairo::init(cfg);
}

fn clean_all(cfg: &Config) {
    cairo::clean(cfg);
    rust::clean(cfg);
    fs::remove_file(cfg.bytecode_path()).unwrap_or_else(|e| {
        eprintln!(
            "Warning: failed to remove {:?} {:?}",
            cfg.bytecode_path(),
            e.kind()
        )
    });
}

fn build_all(cfg: &Config) {
    clean_all(cfg);
    rust::build(cfg);
    gen_bytecode(cfg);
    cairo::build(cfg);

    println!("Build successful.");
}

#[cfg(test)]
mod test {
    use std::io;

    use config::Config;
    use rustc_hex::ToHex;
    use sha2::{Digest, Sha256};
    use tempfile::tempdir;

    use super::*;

    fn init_all_test() -> Config {
        let tmp_dir = tempdir().unwrap().into_path();
        println!("testing init_all in tmp_dir: {:?}", tmp_dir);
        let cfg = Config::new(tmp_dir, "guest_rs".into(), ".".into(), ProjectType::Template);

        init_all(&cfg);
        cfg
    }

    #[test]
    fn test_init_all() {
        let cfg = init_all_test();

        assert!(cfg.rust_dir().exists());
        assert!(cfg.cairo_dir().exists());
    }

    #[test]
    fn test_build_all() {
        let cfg = init_all_test();
        build_all(&cfg);

        assert!(cfg.riscv_binary_path().exists());
        println!("riscv binary path: {:?}", cfg.riscv_binary_path());

        let mut sha = Sha256::new();
        let mut riscv_bin = fs::File::open(cfg.riscv_binary_path()).unwrap();
        let _ = io::copy(&mut riscv_bin, &mut sha).unwrap();

        let digest = sha.finalize();

        println!("riscv binary digest: {}", digest.to_hex::<String>());
        assert_eq!(
            digest.to_hex::<String>(),
            "1d5bcf87de2fc9de05782e766c218dc97f4b6447a65a8d82ca20cc67fbd575a7"
        );

        assert!(cfg.bytecode_path().exists());
        println!("bytecode_path: {:?}", cfg.bytecode_path());

        let cairo_class_path = cfg
            .cairo_dir()
            .join("target")
            .join("release")
            .join("host_cairo_RiscairoExample.contract_class.json");

        assert!(cairo_class_path.exists());
        println!("cairo class path: {:?}", cairo_class_path);

        let mut sha = Sha256::new();
        let mut cairo_class = fs::File::open(cairo_class_path).unwrap();
        let _ = io::copy(&mut cairo_class, &mut sha).unwrap();

        let digest = sha.finalize();

        println!("cairo class digest: {}", digest.to_hex::<String>());
        assert_eq!(
            digest.to_hex::<String>(),
            "ef32b0d0a87a6904faa31ae5844ce67e5e746fce0b47826940e9a451fb6efb5b"
        );
    }
}
