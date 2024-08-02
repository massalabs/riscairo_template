mod guest_rs_bytecode;

#[starknet::interface]
trait IRiscairoExample<TContractState> {
    fn add(self: @TContractState, x: u8, y: u8) -> u8;
    fn sub(self: @TContractState, x: u8, y: u8) -> u8;
    fn prepend_hello(self: @TContractState, text: ByteArray) -> ByteArray;
}

#[starknet::contract]
mod RiscairoExample {
    use super::guest_rs_bytecode::get_bytecode;

    #[storage]
    struct Storage {}

    #[abi(embed_v0)]
    impl RiscairoExample of super::IRiscairoExample<ContractState> {
        fn add(self: @ContractState, x: u8, y: u8) -> u8 {
            let mut args: ByteArray = "";
            args.append_byte(x);
            args.append_byte(y);
            let res = riscairo::riscv_call(@get_bytecode(), @"add", @args,);
            res[0]
        }

        fn sub(self: @ContractState, x: u8, y: u8) -> u8 {
            let mut args: ByteArray = "";
            args.append_byte(x);
            args.append_byte(y);
            let res = riscairo::riscv_call(@get_bytecode(), @"sub", @args,);
            res[0]
        }

        fn prepend_hello(self: @ContractState, text: ByteArray) -> ByteArray {
            riscairo::riscv_call(@get_bytecode(), @"prepend_hello", @text,)
        }
    }
}

