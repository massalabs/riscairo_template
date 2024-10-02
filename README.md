# `riscairo` project template

This is part of the `riscairo` project (<https://github.com/massalabs/riscairo>).

Clone this template to use the `riscairo` virtual machine to provably run Rust code.

## General description

This repo contains a sample `riscairo` guest program written in Rust
(`guest_rs`) that exposes a couple of functions:

* `compute_hash` computes a `blake2s256` hash of the provided data using the
  <https://crates.io/crates/blake2> crate.
* `add` adds the first two `u8` of the argument to return the sum result `u8`.
  Panics on overflow with a message.
* `prepend_hello` prepends the text "hello " in ASCII to the passed bytes and
  returns the concatenated byte array. This uses dynamic memory allocation in
  Rust.

The exposed guest functions are then called from a host Starknet contract
`host_cairo` written in Cairo, which also exports them back.

When you compile the `guest_rs`, make sure to use a tool like the provided
`convert.py` to inline the ELF file into a `.cairo` source file within the
`host_cairo` source.

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

## Project building

Thus, a `riscairo` project is composed of two parts, both having their own build
system. For the sake of simplicity (and portability), these build systems have
been wrapped using the `cargo xtask` workflow.

```cmd
$ cargo xtask
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/xtask`
Tasks:

init_rs             initialize a template rust project
init_cairo          initialize a template cairo project

build               builds the whole project
clean               cleans the whole project

build_rs            builds only the rust project
build_cairo         builds only the cairo project
clean_rs            cleans only the rust project
clean_cairo         cleans only the cairo project
```

`cargo xtask clean` cleans all the generated filles, while `cargo xtask build`
rebuilds everything from the `rust` code up to the `cairo casm`.

