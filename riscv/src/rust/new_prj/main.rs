//! This is the main file of the program. It contains the user-defined functions that need to be made available to the VM.
//! The `export_fn!` macro is used to register the exported functions with the VM. The string literal is the exported name of the function as visible from outside the VM.
//! Everything runs in a `no_std` environment but dynamic memory allocation is available.

#![no_std]
#![no_main]

mod rv;
// Do not edit above this line

// custom alloc here
use alloc::vec::Vec;

// Add your dependencies here
// fn fn_name(args: &[u8]) -> Vec<u8> {
//     ...
// }

// export your functions here
export_fn!(
    //"exported_rust_fn" => fn_name,
);

// Add your code below