# `scarb-riscv` project tool

This is part of the `riscairo` project (<https://github.com/massalabs/riscairo>).

This tool extend `scarb` (the cairo build tool) to ease the use of the
`riscairo` virtual machine to provably run Rust code.

## Installation

Clone this repository then run `cargo install --path riscv` to extend your
standard `scarb` toolkit.

## Usage

(For developers working on the `scarb-riscv` itself, all commands below can be
ran without installing, simply replace the invocation of `scarb riscv` by `cargo
riscv`).

Thus, a `riscairo` project is composed of two parts (see below), both having
their own build system and some linking constraints. For the sake of simplicity
(and portability), these build systems have been wrapped into the `scarb-riscv`.

```cmd
$ scarb riscv
Tasks:

# All in one commands:
init                initializes a brand new project
build               builds the whole project
clean               cleans the whole project

# Single task commands:
init_rs             initializes the rust project
init_cairo          initializes the cairo project

build_rs            builds only the rust project (and exports the binary to cairo)
build_cairo         builds only the cairo project

clean_rs            cleans only the rust project
clean_cairo         cleans only the cairo project
```

 `scarb riscv clean` cleans all the generated filles, while `scarb riscv build`
 rebuilds everything from the `rust` code up to the `cairo casm`.

# Overview of a `riscairo` project

## General description

The guest program written in Rust is place in the (`guest_rs`) directory.

The default example exposes a couple of functions:

* `compute_hash` computes a `blake2s256` hash of the provided data using the
  <https://crates.io/crates/blake2> crate.
* `add` adds the first two `u8` of the argument to return the sum result `u8`.
  Panics on overflow with a message.
* `prepend_hello` prepends the text "hello " in ASCII to the passed bytes and
  returns the concatenated byte array. This uses dynamic memory allocation in
  Rust.

The exposed guest functions are then called from a host Starknet contract
located in the `src` directory and written in Cairo, which also exports them
back.

Compiling the `guest_rs` sub-project can be done with the classical `cargo
build`, as long as the command is run inside the directory. This is useful while
developing. Lastly, in order to export this project to the cairo host, the
`guest_rs` has be compiled with the `scard riscv build_rs`.

Compiling the cairo code can be done either with the classical `scarb build` or
the `scarb riscv build_cairo` command.

Ultimately `scarb riscv build` will take care of build the Rust code, exporting
it to cairo then build the cairo code.

To try it on on a contract already deployed on Sepolia, use `try_me.py` (don't
forget to setup your API key inside of `try_me.py`).

## Features and limitations

### Embedded constraints

The rust code is meant to run on a bare metal RISC-V CPU without an operating
system. This means that only `no_std` Rust code will compile.

### Dynamic memory allocation

This template enables dynamic memory allocation through a bump allocator defined
in `guest_rs/src/rv.rs` that never frees. This was chosen in order to minimize
computing and binary size overhead, but feel free to adjust it to your needs.

### Panics

The template supports panic unwinding and forwards the panic message to the
`riscairo` VM that decodes and handles it.