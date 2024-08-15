mod guest_rs_bytecode;

#[starknet::interface]
trait IRiscairoExample<TContractState> {
    fn compute_hash(self: @TContractState, data: Array<u8>) -> Array<u8>;
    fn add(self: @TContractState, x: u8, y: u8) -> u8;
    fn prepend_hello(self: @TContractState, text: Array<u8>) -> Array<u8>;
}

#[starknet::contract]
mod RiscairoExample {
    use super::guest_rs_bytecode::get_bytecode;

    #[storage]
    struct Storage {}

    #[abi(embed_v0)]
    impl RiscairoExample of super::IRiscairoExample<ContractState> {
        /// Compute the blake2s256 hash of the given data using the blake2 rust crate 
        fn compute_hash(self: @ContractState, data: Array<u8>) -> Array<u8> {
            riscairo::riscv_call(get_bytecode(), @"compute_hash", @data,)
        }

        /// Add two numbers to demonstrate arithmetics.
        /// Note that if the sum overflows u8, it cause sa guest panic.
        fn add(self: @ContractState, x: u8, y: u8) -> u8 {
            let mut args = ArrayTrait::<u8>::new();
            args.append(x);
            args.append(y);
            let res = riscairo::riscv_call(get_bytecode(), @"add", @args,);
            *res.at(0)
        }

        /// Prepend "hello" to the given text to demonstrate dynamic allocation.
        fn prepend_hello(self: @ContractState, text: Array<u8>) -> Array<u8> {
            riscairo::riscv_call(get_bytecode(), @"prepend_hello", @text,)
        }
    }
}
