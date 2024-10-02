pub(super) const SCARB_TOML: &str = r#"[package]
name = "host_cairo"
version = "0.1.0"
edition = "2024_07"

# See more keys and their definitions at https://docs.swmansion.com/scarb/docs/reference/manifest.html

[dependencies]
riscairo = { git = "https://github.com/massalabs/riscairo.git" }
starknet = ">=2.3.0"

[[target.starknet-contract]]
sierra = true        # Enable Sierra codegen.
casm = true          # Enable Casm codegen.
"#;

pub(super) const LIB_CAIRO: &str = r#"mod guest_rs_bytecode;

#[starknet::interface]
trait IRiscairoExample<TContractState> {
    fn compute_hash(self: @TContractState, data: Array<u8>) -> Array<u8>;
    fn add(self: @TContractState, x: u8, y: u8) -> u8;
    fn prepend_hello(self: @TContractState, text: Array<u8>) -> Array<u8>;
}

#[starknet::contract]
mod RiscairoExample {
    use super::guest_rs_bytecode::BYTECODE;

    #[storage]
    struct Storage {}

    #[abi(embed_v0)]
    impl RiscairoExample of super::IRiscairoExample<ContractState> {
        /// Compute the blake2s256 hash of the given data using the blake2 rust crate
        fn compute_hash(self: @ContractState, data: Array<u8>) -> Array<u8> {
            riscairo::riscv_call(BYTECODE.span(), @"compute_hash", @data,)
        }

        /// Add two numbers to demonstrate arithmetics.
        /// Note that if the sum overflows u8, it causes a guest panic.
        fn add(self: @ContractState, x: u8, y: u8) -> u8 {
            let mut args = ArrayTrait::<u8>::new();
            args.append(x);
            args.append(y);
            let res = riscairo::riscv_call(BYTECODE.span(), @"add", @args,);
            *res.at(0)
        }

        /// Prepend "hello" to the given text to demonstrate dynamic allocation.
        fn prepend_hello(self: @ContractState, text: Array<u8>) -> Array<u8> {
            riscairo::riscv_call(BYTECODE.span(), @"prepend_hello", @text,)
        }
    }
}
"#;