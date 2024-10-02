use std::{fs::{self, File}, io::Write, process::Command};

use crate::{run_command, CONFIG};

mod constants;

fn scarb() -> String {
    let scarb_program = "scarb";

    // check if the scarb command is present on the system
    match Command::new(scarb_program).output() {
        Ok(_) => scarb_program.into(),
        Err(e) => {
            panic!(
                "{}, check that `{}` is installed and accessible in your path.",
                e, scarb_program
            )
        }
    }
}

pub fn clean() {
    run_command(&scarb(), &["clean"], CONFIG.cairo_dir())
}

pub fn build() {
    run_command(&scarb(), &["--release", "build"], CONFIG.cairo_dir())
}

pub fn init() {
    // if the cairo directory already exists, do nothing
    if CONFIG.cairo_dir().exists() {
        eprintln!(
            "cairo directory already exists: {:?}, skipping",
            CONFIG.cairo_dir()
        );
        return;
    }

    // create cairo directory
    fs::create_dir_all(CONFIG.cairo_dir()).unwrap_or_else(|e| {
        panic!(
            "failed to create directory: {:?} with error {}",
            CONFIG.cairo_dir(),
            e
        )
    });

    // create src/ directory
    let src_dir = CONFIG.cairo_dir().join("src");
    fs::create_dir_all(src_dir.clone()).unwrap_or_else(|e| {
        panic!(
            "failed to create directory: {:?} with error {}",
            src_dir, e
        )
    });

    // create lib.cairo
    let lib_cairo_path = src_dir.join("lib.cairo");
    let mut lib_cairo = File::create(lib_cairo_path.clone())
        .unwrap_or_else(|e| panic!("failed to create file: {:?} with error {}", lib_cairo_path, e));
    lib_cairo
        .write_all(constants::LIB_CAIRO.as_bytes())
        .unwrap_or_else(|e| {
            panic!(
                "failed to write to file: {:?} with error {}",
                lib_cairo_path, e
            )
        });

    // create Scarb.toml
    let scarb_toml_path = CONFIG.cairo_dir().join("Scarb.toml");
    let mut scarb_toml = File::create(scarb_toml_path.clone())
        .unwrap_or_else(|e| panic!("failed to create file: {:?} with error {}", scarb_toml_path, e));
    scarb_toml
        .write_all(constants::SCARB_TOML.as_bytes())
        .unwrap_or_else(|e| {
            panic!(
                "failed to write to file: {:?} with error {}",
                scarb_toml_path, e
            )
        });
}