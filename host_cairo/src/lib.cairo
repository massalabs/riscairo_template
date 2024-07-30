mod guest_rs_bytecode;

fn main() {
    println!("=====================================");

    println!("Let's do a simple addition on the RISCairo VM:");
    let mut args: ByteArray = "";
    args.append_byte(5);
    args.append_byte(2);
    let res = riscairo::riscv_call(
        @guest_rs_bytecode::get_bytecode(),
        @"add",
        @args,
    );
    println!("    {} + {} = {}", args[0], args[1], res[0]);
    println!("");

    println!("Now let's do a simple subtraction on the RISCairo VM:");
    let res = riscairo::riscv_call(
        @guest_rs_bytecode::get_bytecode(),
        @"sub",
        @args,
    );
    println!("    {} - {} = {}", args[0], args[1], res[0]);
    println!("");

    println!("Now let's prepend text using the RISCairo VM (this involves dynamic memory allocation):");
    let text: ByteArray = "world";
    let res = riscairo::riscv_call(
        @guest_rs_bytecode::get_bytecode(),
        @"prepend_hello",
        @text,
    );
    println!("    concat('hello ', '{}') = '{}'", text, res);
    println!("");
}



