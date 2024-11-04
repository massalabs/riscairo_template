use std::{
    env,
    fs::{self, File},
    io::Write,
};

use crate::{
    config::{Config, ProjectType},
    run_command,
};

mod constants;

fn cargo() -> String {
    env::var("CARGO").unwrap_or_else(|_| "cargo".to_string())
}

pub fn clean(cfg: &Config) {
    run_command(&cargo(), &["clean"], cfg.rust_dir())
}

pub fn build(cfg: &Config) {
    run_command(&cargo(), &["build", "--release"], cfg.rust_dir())
}

pub fn init(cfg: &Config) {
    // if the rust directory already exists, do nothing
    if cfg.rust_dir().exists() {
        eprintln!(
            "rust directory already exists: {:?}, skipping",
            cfg.rust_dir()
        );
        return;
    }

    // create rust directory
    fs::create_dir_all(cfg.rust_dir()).unwrap_or_else(|e| {
        panic!(
            "failed to create directory: {:?} with error {}",
            cfg.rust_dir(),
            e
        )
    });

    // create Cargo.toml
    let cargo_toml_path = cfg.rust_dir().join("Cargo.toml");
    let mut cargo_toml = File::create(cargo_toml_path.clone()).unwrap_or_else(|e| {
        panic!(
            "failed to create file: {:?} with error {}",
            cargo_toml_path, e
        )
    });

    let (cargo_toml_content, main_rs_content) = match cfg.project_type() {
        ProjectType::New => (constants::CARGO_TOML_NEW, constants::MOD_MAIN_RS_NEW),
        ProjectType::Template => (
            constants::CARGO_TOML_TEMPLATE,
            constants::MOD_MAIN_RS_TEMPLATE,
        ),
    };

    cargo_toml
        .write_all(cargo_toml_content.as_bytes())
        .unwrap_or_else(|e| {
            panic!(
                "failed to write to file: {:?} with error {}",
                cargo_toml_path, e
            )
        });

    // create .cargo/ directory
    let cango_config_dir = cfg.rust_dir().join(".cargo");
    fs::create_dir_all(cango_config_dir.clone()).unwrap_or_else(|e| {
        panic!(
            "failed to create directory: {:?} with error {}",
            cango_config_dir, e
        )
    });
    // create .cargo/config.toml
    let cargo_config_toml_path = cango_config_dir.join("config.toml");
    let mut cargo_config_toml = File::create(cargo_config_toml_path.clone()).unwrap_or_else(|e| {
        panic!(
            "failed to create file: {:?} with error {}",
            cargo_config_toml_path, e
        )
    });
    cargo_config_toml
        .write_all(constants::CARGO_CONFIG_TOML.as_bytes())
        .unwrap_or_else(|e| {
            panic!(
                "failed to write to file: {:?} with error {}",
                cargo_config_toml_path, e
            )
        });

    // create link.ld
    let link_ld_path = cfg.rust_dir().join("link.ld");
    let mut link_ld = File::create(link_ld_path.clone())
        .unwrap_or_else(|e| panic!("failed to create file: {:?} with error {}", link_ld_path, e));
    link_ld
        .write_all(constants::LINK_LD.as_bytes())
        .unwrap_or_else(|e| {
            panic!(
                "failed to write to file: {:?} with error {}",
                link_ld_path, e
            )
        });

    // create src/ directory
    let src_dir = cfg.rust_dir().join("src");
    fs::create_dir_all(src_dir.clone())
        .unwrap_or_else(|e| panic!("failed to create directory: {:?} with error {}", src_dir, e));

    // create rv.rs module
    let rv_rs_path = src_dir.join("rv.rs");
    let mut rv_rs = File::create(rv_rs_path.clone())
        .unwrap_or_else(|e| panic!("failed to create file: {:?} with error {}", rv_rs_path, e));
    rv_rs
        .write_all(constants::MOD_RV_RS.as_bytes())
        .unwrap_or_else(|e| panic!("failed to write to file: {:?} with error {}", rv_rs_path, e));

    // create main.rs
    let main_rs_path = src_dir.join("main.rs");
    let mut main_rs = File::create(main_rs_path.clone())
        .unwrap_or_else(|e| panic!("failed to create file: {:?} with error {}", main_rs_path, e));
    main_rs
        .write_all(main_rs_content.as_bytes())
        .unwrap_or_else(|e| {
            panic!(
                "failed to write to file: {:?} with error {}",
                main_rs_path, e
            )
        });
}
