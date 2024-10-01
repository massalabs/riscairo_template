use std::process::Command;

use crate::{run_command, CONFIG};

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
