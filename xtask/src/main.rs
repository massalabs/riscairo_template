use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

type DynError = Box<dyn std::error::Error>;

mod cairo;
mod config;
mod rust;

lazy_static::lazy_static! {
    static ref CONFIG: config::Config = config::Config::new(
        // get the project root, CARGO_MANIFEST_DIR is a xtask dir, so go up 1
        Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap().into(),
        PathBuf::from("guest_rs"),
        PathBuf::from("host_cairo"),
    );
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<(), DynError> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("init_rs") => rust::init(),
        Some("init_cairo") => cairo::init(),
        Some("build") => build_all(),
        Some("clean") => clean_all(),
        Some("build_rs") => {
            rust::build();
            gen_bytecode()
        }
        Some("build_cairo") => cairo::build(),
        Some("clean_rs") => rust::clean(),
        Some("clean_cairo") => cairo::clean(),
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "Tasks:

init_rs             initializes the rust project
init_cairo          initializes the cairo project

build               builds the whole project
clean               cleans the whole project

build_rs            builds only the rust project
build_cairo         builds only the cairo project
clean_rs            cleans only the rust project
clean_cairo         cleans only the cairo project
"
    )
}

fn gen_bytecode() {
    elf_to_bytecode(CONFIG.riscv_binary_path(), CONFIG.bytecode_path());
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

fn clean_all() {
    cairo::clean();
    rust::clean();
    fs::remove_file(CONFIG.bytecode_path()).unwrap_or_else(|e| {
        eprintln!(
            "Warning: failed to remove {:?} {:?}",
            CONFIG.bytecode_path(),
            e.kind()
        )
    });
}

fn build_all() {
    clean_all();
    rust::build();
    gen_bytecode();
    cairo::build();

    println!("Build successful.");
}
