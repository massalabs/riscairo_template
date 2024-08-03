mod guest_rs_bytecode;

#[starknet::interface]
trait IRiscairoExample<TContractState> {
    fn compute_hash(self: @TContractState, data: Array<u8>) -> Array<u8>;
    fn add(self: @TContractState, x: u8, y: u8) -> u8;
    fn prepend_hello(self: @TContractState, text: ByteArray) -> ByteArray;
}

#[starknet::contract]
mod RiscairoExample {
    use super::guest_rs_bytecode::get_bytecode;

    #[storage]
    struct Storage {}

    #[abi(embed_v0)]
    impl RiscairoExample of super::IRiscairoExample<ContractState> {
        fn compute_hash(self: @ContractState, data: Array<u8>) -> Array<u8> {
            riscairo::riscv_call(get_bytecode(), @"compute_hash", @data,)
        }

        fn add(self: @ContractState, x: u8, y: u8) -> u8 {
            let mut args = ArrayTrait::<u8>::new();
            args.append(x);
            args.append(y);
            let res = riscairo::riscv_call(get_bytecode(), @"add", @args,);
            *res.at(0)
        }

        fn prepend_hello(self: @ContractState, text: ByteArray) -> ByteArray {
            let mut args = ArrayTrait::<u8>::new();
            let mut i: u32 = 0;
            while i < text.len() {
                args.append(text[i]);
                i += 1;
            };
            let res = riscairo::riscv_call(get_bytecode(), @"prepend_hello", @args,);
            let mut j: u32 = 0;
            let mut res_txt: ByteArray = "";
            while j < res.len() {
                res_txt.append_byte(*res.at(j));
                j += 1;
            };
            res_txt
        }
    }
}

