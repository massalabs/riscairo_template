pub mod guest_rs_bytecode;

#[starknet::interface]
trait IRiscairoExample<TContractState> {
    fn compute_hash(self: @TContractState, data: Array<u8>) -> Array<u8>;
    fn add(self: @TContractState, x: u8, y: u8) -> u8;
    fn prepend_hello(self: @TContractState, text: ByteArray) -> ByteArray;
}

use utils::traits::bytes::{ToBytes, U8ToBytes};

#[derive(Drop)]
pub struct AddArgs {
    pub operand1: u8,
    pub operand2: u8,
}

impl AddArgsToBytes of ToBytes<AddArgs> {
    fn to_le_bytes(self: AddArgs) -> Span<u8> {
        [self.operand1, self.operand2].span()
    }

    fn to_be_bytes(self: AddArgs) -> Span<u8> {
        panic!("unsupported")
    }

    fn to_le_bytes_padded(self: AddArgs) -> Span<u8> {
        panic!("unsupported")
    }

    fn to_be_bytes_padded(self: AddArgs) -> Span<u8> {
        panic!("unsupported")
    }
}


#[starknet::contract]
mod RiscairoExample {
    use super::guest_rs_bytecode::BYTECODE;
    use super::AddArgs;
    use super::U8ToBytes;
    use riscairo::bytes::U8FromBytes;
    use riscairo::bytes::ArrayU8ToBytes;
    use riscairo::bytes::ArrayU8FromBytes;
    use riscairo::bytes::ByteArrayToBytes;
    use riscairo::bytes::ByteArrayFromBytes;

    #[storage]
    struct Storage {}

    #[abi(embed_v0)]
    impl RiscairoExample of super::IRiscairoExample<ContractState> {
        /// Compute the blake2s256 hash of the given data using the blake2 rust crate
        fn compute_hash(self: @ContractState, data: Array<u8>) -> Array<u8> {
            riscairo::riscv_call(BYTECODE.span(), @"compute_hash", data,)
        }

        /// Add two numbers to demonstrate arithmetics.
        /// Note that if the sum overflows u8, it causes a guest panic.
        fn add(self: @ContractState, x: u8, y: u8) -> u8 {
            let args = AddArgs { operand1: x, operand2: y };
            riscairo::riscv_call(BYTECODE.span(), @"add", args,)
        }

        /// Prepend "hello" to the given text to demonstrate dynamic allocation.
        fn prepend_hello(self: @ContractState, text: ByteArray) -> ByteArray {
            riscairo::riscv_call(BYTECODE.span(), @"prepend_hello", text,)
        }
    }
}
