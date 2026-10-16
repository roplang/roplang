use crate::sha256;

pub const OPCODES: [&str; 32] = [
    "nop", "push", "pop", "dup", "swap", "add", "sub", "mul",
    "div", "mod", "shl", "shr", "and", "or", "xor", "not",
    "eq", "ne", "lt", "gt", "le", "ge", "jmp", "jz",
    "jnz", "load", "store", "putc", "putn", "getc", "call", "ret",
];

/// The compiler: SHA-256 digest -> (opcode, argument). Text never executes.
pub fn compile_line(line: &str) -> (u8, u64) {
    let d = sha256::sha256(line.as_bytes());
    let mut b = [0u8; 8];
    b.copy_from_slice(&d[1..9]);
    (d[0] % 32, u64::from_be_bytes(b))
}

pub fn op_name(op: u8) -> &'static str { OPCODES[(op % 32) as usize] }
