use std::{
    fs::{self, File},
    io::Write,
};

use crate::CONFIG;
mod constants;

pub fn init() {
    // if the proto directory already exists, do nothing
    if CONFIG.proto_dir().exists() {
        eprintln!(
            "proto directory already exists: {:?}, skipping",
            CONFIG.proto_dir()
        );
        return;
    }

    // create proto directory
    fs::create_dir_all(CONFIG.proto_dir()).unwrap_or_else(|e| {
        panic!(
            "failed to create directory: {:?} with error {}",
            CONFIG.proto_dir(),
            e
        )
    });

    // create interface.proto
    let interface_proto_path = CONFIG.proto_dir().join("interface.proto");
    let mut interface_proto = File::create(interface_proto_path.clone()).unwrap_or_else(|e| {
        panic!(
            "failed to create file: {:?} with error {}",
            interface_proto_path, e
        )
    });
    interface_proto
        .write_all(constants::INTERFACE_PROTO.as_bytes())
        .unwrap_or_else(|e| {
            panic!(
                "failed to write to file: {:?} with error {}",
                interface_proto_path, e
            )
        });
}

pub(crate) fn gen_interfaces() {
    let interface_proto_path = CONFIG.proto_dir().join("interface.proto");

    micropb_gen::Generator::new()
        .use_container_alloc()
        .add_protoc_arg("-Iproto")
        .compile_protos(
            &["interface.proto"],
            CONFIG.rust_dir().join("src").join("interface.rs"),
        )
        .unwrap();

    cairo_proto_build::Config::new()
        .out_dir(CONFIG.cairo_dir().join("src"))
        .oracle_lock("oracle.lock") // mandatory but useless in our case
        .compile_protos(&[&interface_proto_path], &[CONFIG.proto_dir()])
        .unwrap();
}
