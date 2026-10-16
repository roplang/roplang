use crate::isa::compile_line;

/// Proof-of-Work programming: find a line whose SHA-256 compiles to `op`
/// and (optionally) whose argument satisfies arg % modulo == value.
pub fn mine(prefix: &str, op: u8, modulo: u64, value: u64) -> (String, u64) {
    let mut i: u64 = 0;
    loop {
        let line = format!("{} {}", prefix, i);
        let (o, a) = compile_line(&line);
        if o == op % 32 && (modulo == 0 || a % modulo == value % modulo) {
            return (line, i);
        }
        i += 1;
    }
}
