//! This is the main file of the program. It contains the user-defined functions that need to be made available to the VM.
//! The `export_fn!` macro is used to register the exported functions with the VM. The string literal is the exported name of the function as visible from outside the VM.
//! Everything runs in a `no_std` environment but dynamic memory allocation is available.

#![no_std]
#![no_main]

mod rv;
// Do not edit above this line

// custom alloc here
use alloc::vec::Vec;

// Add your code below
use blake2::{Blake2s256, Digest};

fn compute_hash(args: &[u8]) -> Vec<u8> {
    Blake2s256::digest(args).to_vec()
}

fn add(args: &[u8]) -> Vec<u8> {
    if args.len() != 2 {
        panic!("Invalid arguments");
    }
    match args[0].checked_add(args[1]) {
        Some(v) => [v].to_vec(),
        None => panic!("The sum of the operands overflows u8"),
    }
}

fn prepend_hello(args: &[u8]) -> Vec<u8> {
    let mut res = Vec::new();
    res.extend_from_slice(b"hello ");
    res.extend_from_slice(args);
    res
}

// export your functions here
export_fn!(
    "compute_hash" => compute_hash,
    "add" => add,
    "prepend_hello" => prepend_hello
);
