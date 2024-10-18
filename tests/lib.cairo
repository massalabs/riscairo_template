use riscairo::riscv_call;

use utils::traits::bytes::{ToBytes, FromBytes};

use host_cairo::guest_rs_bytecode::BYTECODE;
use host_cairo::AddArgs;
use riscairo::bytes::U8FromBytes;
use riscairo::bytes::ArrayU8ToBytes;
use riscairo::bytes::ArrayU8FromBytes;
use riscairo::bytes::ByteArrayToBytes;
use riscairo::bytes::ByteArrayFromBytes;


#[test]
fn test_add() {
    let x = 1;
    let y = 2;
    let args = AddArgs { operand1: x, operand2: y };
    let res: u8 = riscairo::riscv_call(BYTECODE.span(), @"add", args,);
    assert_eq!(res, 3);
}

#[test]
fn test_prepend_hello() {
    let text: ByteArray = "world";
    let res: ByteArray = riscairo::riscv_call(BYTECODE.span(), @"prepend_hello", text,);
    println!("string: {}", res);
    assert_eq!(res, "hello world");
}

#[test]
fn test_compute_hash() {
    let data: Array<u8> = array![1, 2, 3, 4, 5];
    let res: Array<u8> = riscairo::riscv_call(BYTECODE.span(), @"compute_hash", data,);
    println!("hash: {:?}", res);
    assert_eq!(res, array![79, 161, 32, 2, 141, 62, 240, 181, 1, 170, 136, 165, 249, 76, 58, 243, 89, 111, 85, 63, 236, 236, 211, 219, 19, 218, 167, 3, 113, 102, 73, 16]);
}

