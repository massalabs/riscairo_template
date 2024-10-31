mod guest_rs_bytecode;

#[starknet::interface]
trait IRiscairoExample<TContractState> {
    // Args have to be serialized to bytes and deserialized back to the original type
    fn rust_fn_wrapper(self: @TContractState, args: Array<u8>) -> Array<u8>;
}

#[starknet::contract]
mod RiscairoExample {
    use super::guest_rs_bytecode::BYTECODE;

    #[storage]
    struct Storage {}

    #[abi(embed_v0)]
    impl RiscairoExample of super::IRiscairoExample<ContractState> {
        fn rust_fn_wrapper(self: @ContractState, args: Array<u8>) -> Array<u8> {
            riscairo::riscv_call(BYTECODE.span(), @"exported_rust_fn", @args,)
        }
    }
}
