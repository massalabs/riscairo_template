use std::path::{Path, PathBuf};

pub struct Config {
    _project_root: PathBuf,
    rust_dir: PathBuf,
    cairo_dir: PathBuf,
    riscv_binary_path: PathBuf,
    bytecode_path: PathBuf,
}

impl Config {
    pub fn new(project_root: PathBuf, rust_dir: PathBuf, cairo_dir: PathBuf) -> Self {
        Self {
            riscv_binary_path: project_root
                .join(rust_dir.clone())
                .join("target")
                .join("riscv32i-unknown-none-elf")
                .join("release")
                .join("riscairo_guest_rs"),
            bytecode_path: project_root
                .join(cairo_dir.clone())
                .join("src")
                .join("guest_rs_bytecode.cairo"),
            rust_dir: project_root.join(rust_dir),
            cairo_dir: project_root.join(cairo_dir),
            _project_root: project_root.to_owned(),
        }
    }

    pub fn _project_root(&self) -> &Path {
        &self._project_root
    }

    pub fn rust_dir(&self) -> &Path {
        &self.rust_dir
    }

    pub fn cairo_dir(&self) -> &Path {
        &self.cairo_dir
    }

    pub fn riscv_binary_path(&self) -> &Path {
        &self.riscv_binary_path
    }

    pub fn bytecode_path(&self) -> &Path {
        &self.bytecode_path
    }
}
