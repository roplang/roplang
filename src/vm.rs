use crate::isa::compile_line;

const MEM_SIZE: usize = 1024;

pub fn run(program: &str, input: &[u8]) -> Result<String, String> {
    let lines: Vec<&str> = program.lines().collect();
    let n = lines.len();
    let mut code: Vec<(u8, u64)> = Vec::with_capacity(n);
    for raw in &lines {
        let t = raw.trim();
        if t.is_empty() || t.starts_with(';') {
            code.push((0, 0)); // заглушка, не исполняется
        } else {
            code.push(compile_line(t));
        }
    }

    let mut stack: Vec<u64> = Vec::new();
    let mut calls: Vec<usize> = Vec::new();
    let mut mem = [0u64; MEM_SIZE];
    let mut out = String::new();
    let mut in_pos = 0usize;
    let mut ip: usize = 0;

    macro_rules! err { ($($a:tt)*) => { return Err(format!($($a)*)) } }
    macro_rules! pop { () => { match stack.pop() { Some(v) => v, None => err!("строка {}: underflow", ip + 1) } } }
    macro_rules! bin { ($f:expr) => {{ let b = pop!(); let a = pop!(); stack.push(($f)(a, b)); }} }

    while ip < n {
        let (op, arg) = code[ip];
        match op % 32 {
            0 => {}
            1 => stack.push(arg % 65536),
            2 => { pop!(); }
            3 => { let v = *stack.last().unwrap_or(&0); stack.push(v); }
            4 => { let b = pop!(); let a = pop!(); stack.push(b); stack.push(a); }
            5 => bin!(|a: u64, b: u64| a.wrapping_add(b)),
            6 => bin!(|a: u64, b: u64| a.wrapping_sub(b)),
            7 => bin!(|a: u64, b: u64| a.wrapping_mul(b)),
            8 => bin!(|a: u64, b: u64| if b == 0 { 0 } else { a / b }),
            9 => bin!(|a: u64, b: u64| if b == 0 { 0 } else { a % b }),
            10 => bin!(|a: u64, b: u64| a.wrapping_shl(b as u32)),
            11 => bin!(|a: u64, b: u64| a.wrapping_shr(b as u32)),
            12 => bin!(|a: u64, b: u64| a & b),
            13 => bin!(|a: u64, b: u64| a | b),
            14 => bin!(|a: u64, b: u64| a ^ b),
            15 => { let a = pop!(); stack.push(!a); }
            16 => bin!(|a: u64, b: u64| if a == b { 1 } else { 0 }),
            17 => bin!(|a: u64, b: u64| if a != b { 1 } else { 0 }),
            18 => bin!(|a: u64, b: u64| if a < b { 1 } else { 0 }),
            19 => bin!(|a: u64, b: u64| if a > b { 1 } else { 0 }),
            20 => bin!(|a: u64, b: u64| if a <= b { 1 } else { 0 }),
            21 => bin!(|a: u64, b: u64| if a >= b { 1 } else { 0 }),
            22 => { ip = (arg as usize) % n; continue; }
            23 => { let v = pop!(); if v == 0 { ip = (arg as usize) % n; continue; } }
            24 => { let v = pop!(); if v != 0 { ip = (arg as usize) % n; continue; } }
            25 => stack.push(mem[(arg as usize) % MEM_SIZE]),
            26 => { mem[(arg as usize) % MEM_SIZE] = pop!(); }
            27 => { let v = pop!(); out.push((v % 128) as u8 as char); }
            28 => { let v = pop!(); out.push_str(&v.to_string()); }
            29 => {
                let v = if in_pos < input.len() { let c = input[in_pos]; in_pos += 1; c as u64 } else { 0 };
                stack.push(v);
            }
            30 => { calls.push(ip + 1); ip = (arg as usize) % n; continue; }
            31 => match calls.pop() { Some(ret) => { ip = ret; continue; } None => break },
            _ => unreachable!(),
        }
        ip += 1;
    }
    Ok(out)
}
