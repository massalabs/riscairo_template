use std::env;

use crate::{run_command, CONFIG};

fn cargo() -> String {
    env::var("CARGO").unwrap_or_else(|_| "cargo".to_string())
}

pub fn clean() {
    run_command(&cargo(), &["clean"], CONFIG.rust_dir())
}

pub fn build() {
    run_command(&cargo(), &["build", "--release"], CONFIG.rust_dir())
}
