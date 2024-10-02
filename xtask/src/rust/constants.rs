pub(super) const CARGO_TOML: &str = r#"[package]
name = "riscairo_guest_rs"
version = "0.1.0"
authors = []
edition = "2021"

[dependencies]
blake2 = {version = "0.10", default-features = false}

# Do not edit after this line
[profile.dev]
panic = "abort"

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = "symbols"
"#;

pub(super) const CARGO_CONFIG_TOML: &str = r#"[build]
target = "riscv32i-unknown-none-elf"
rustflags = [
    "-C", "link-arg=-Tlink.ld",
    "-C", "link-arg=--gc-sections",
    "-C", "link-arg=--strip-all",
    "-C", "link-arg=--nostdlib",
    "-C", "link-arg=--strip-debug",
    "-C", "link-arg=--build-id=none",
    "-C", "link-arg=--no-warn-mismatch",
    "-C", "link-arg=-n",
]
"#;

pub(super) const LINK_LD: &str = r#"/* Linker script for riscairo guests. Do not modify. */

/* Define the memory regions */
MEMORY
{
    RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 10M
    IN (rwx) : ORIGIN = 0x30000000, LENGTH = 1M
}

/* Define the sections */
SECTIONS
{
    . = ORIGIN(RAM);

    /* Code and read-only data */
    .text : ALIGN(4)
    {
        *(.text.init)
        *(.text*)
        *(.rodata*)
    } > RAM

    /* Initialized data */
    .data : ALIGN(4)
    {
        *(.data*)
    } > RAM

    /* Uninitialized data */
    .bss : ALIGN(4)
    {
        *(.bss)
        *(COMMON)
    } > RAM

    /* Small uninitialized data */
    .sbss : ALIGN(4)
    {
        *(.sbss*)
    } > RAM

    /* Stack section */
    .stack (NOLOAD) : ALIGN(4)
    {
        __sp = . + 0x10000;  /* Adjust stack size as needed */
        . = .;               /* Keep location counter at the end of stack */
    } > RAM

    /* Heap section */
    .heap (NOLOAD) : ALIGN(4)
    {
        HEAP = .;
        . = . + 1048576;  /* Heap size */
    } > RAM

    /* Custom sections */
    .in_name_len (NOLOAD) : ALIGN(1)
    {
        . = ORIGIN(IN);
        IN_FUNC_NAME_LEN = .;
        . = . + 4;
    } > IN

    .in_name (NOLOAD) : ALIGN(1)
    {
        IN_FUNC_NAME = .;
        . = . + 255;
    } > IN

    .in_args_len (NOLOAD) : ALIGN(1)
    {
        IN_FUNC_ARGS_LEN = .;
        . = . + 4;
    } > IN

    .in_args (NOLOAD) : ALIGN(1)
    {
        IN_FUNC_ARGS = .;
        . = . + 10240;  /* Set args max length */
    } > IN

    /* End of the data section */
    _end = .;

    /* Discard sections to minimize binary size */
    /DISCARD/ : {
        *(.eh_frame)
        *(.comment)
        *(.debug*)
        *(.note*)
        *(.symtab)
        *(.strtab)
        *(.riscv.attributes)
    }
}
"#;

pub(super) const MOD_RV_RS: &str = r#"//! This file contains the runtime setup for the `riscario`` VM.
//! Do not modify it.

use core::arch::{asm, global_asm};

#[macro_export]
macro_rules! export_fn {
    ($($name:expr => $func:ident),* $(,)?) => {
        extern crate alloc;

        #[no_mangle]
        pub extern "C" fn _main() {
            $(
                rv::_register_function($name, $func);
            )*
            rv::_run();
        }
    };

}

// initialize the stack pointer and call the user entry point
global_asm!(
    "
    .section .text.init
    .global _start
    _start:
        la   sp, __sp
        call _main
    "
);

// fixed memory area for input data
extern "C" {
    static IN_FUNC_NAME_LEN: usize;
    static IN_FUNC_NAME: [u8; 255];
    static IN_FUNC_ARGS_LEN: usize;
    static IN_FUNC_ARGS: [u8; 10240];
    static HEAP: [u8; 1048576];
}

const ECALL_CATEGORY_PANIC: u32 = 1;
const ECALL_CATEGORY_RETURN: u32 = 2;

extern crate alloc;
use alloc::vec::Vec;
use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use core::ptr::null_mut;

// a simple allocator that never frees
struct BumpAllocator {
    offset: UnsafeCell<usize>,
}

impl BumpAllocator {
    const fn new() -> Self {
        BumpAllocator {
            offset: UnsafeCell::new(0),
        }
    }

    unsafe fn bump_alloc(&self, layout: Layout) -> *mut u8 {
        let heap_start = HEAP.as_ptr() as usize;
        let offset = self.offset.get();
        let alloc_start = heap_start + *offset;
        let alloc_size = layout.size();
        let Some(alloc_end) = alloc_start.checked_add(alloc_size) else {
            return null_mut();
        };
        if alloc_end > heap_start + HEAP.len() {
            return null_mut();
        }
        *offset += alloc_size;
        alloc_start as *mut u8
    }
}

// Implementing Sync for BumpAllocator
unsafe impl Sync for BumpAllocator {}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.bump_alloc(layout)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocator does not deallocate
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: BumpAllocator = BumpAllocator::new();

// function registry
const MAX_FUNCTIONS: usize = 128;
static mut FUNCTION_NAMES: [Option<&'static [u8]>; MAX_FUNCTIONS] = [None; MAX_FUNCTIONS];
static mut FUNCTION_PTRS: [Option<fn(&[u8]) -> Vec<u8>>; MAX_FUNCTIONS] = [None; MAX_FUNCTIONS];
static mut FUNCTION_COUNT: usize = 0;

// use assembly to send the memory address of the data, and call "ecall" to signal upstream that there is data to be read
fn ecall(data: &[u8], category: u32) {
    unsafe {
        let data_len: u32 = data.len() as u32;
        let data_ptr: u32 = data.as_ptr() as u32;
        asm!(
            "mv a0, {0}",   // x10 registry: category
            "mv a1, {1}",   // x11 registry: data_len
            "mv a2, {2}",   // x12 registry: data_ptr
            "ecall",
            in(reg) category,
            in(reg) data_len,
            in(reg) data_ptr,
            out("a0") _,
            out("a1") _,
            out("a2") _,
        );
    };
}

// retuns a tuple of the function name and the function arguments
fn einput() -> (&'static [u8], &'static [u8]) {
    unsafe {
        (
            core::slice::from_raw_parts(IN_FUNC_NAME.as_ptr(), IN_FUNC_NAME_LEN),
            core::slice::from_raw_parts(IN_FUNC_ARGS.as_ptr(), IN_FUNC_ARGS_LEN),
        )
    }
}

// handle panic by sending the panic message to the upstream and quit
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let msg = info.message().as_str().unwrap_or_default().as_bytes();
    ecall(msg, ECALL_CATEGORY_PANIC);
    // assume we never return from this ecall
    unsafe {
        asm!("", options(noreturn));
    };
}

// send result data to the upstream and quit
fn ereturn(data: &[u8]) -> ! {
    ecall(data, ECALL_CATEGORY_RETURN);
    // assume we never return from this ecall
    unsafe {
        asm!("", options(noreturn));
    };
}

pub fn _register_function(name: &'static str, func: fn(&[u8]) -> Vec<u8>) {
    unsafe {
        if FUNCTION_COUNT >= MAX_FUNCTIONS {
            panic!("Too many functions");
        }
        FUNCTION_NAMES[FUNCTION_COUNT] = Some(name.as_bytes());
        FUNCTION_PTRS[FUNCTION_COUNT] = Some(func);
        FUNCTION_COUNT += 1;
    }
}

// main entry point of the program
#[no_mangle]
pub fn _run() -> ! {
    // Read input data
    let (func_name, args) = einput();

    // find the right function to call
    for i in 0..unsafe { FUNCTION_COUNT } {
        if let Some(name) = unsafe { FUNCTION_NAMES[i] } {
            if name == func_name {
                let func = unsafe { FUNCTION_PTRS[i] }.unwrap();
                let res = func(args);
                ereturn(&res);
            }
        }
    }

    // function not found
    panic!("Function not found");
}
"#;

pub(super) const MOD_MAIN_RS: &str = r#"//! This is the main file of the program. It contains the user-defined functions that need to be made available to the VM.
//! The `export_fn!` macro is used to register the exported functions with the VM. The string literal is the exported name of the function as visible from outside the VM.
//! Everything runs in a `no_std` environment but dynamic memory allocation is available.

#![no_std]
#![no_main]

mod rv;

use alloc::vec::Vec;
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

export_fn!(
    "compute_hash" => compute_hash,
    "add" => add,
    "prepend_hello" => prepend_hello
);
"#;
