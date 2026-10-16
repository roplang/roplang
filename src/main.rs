mod sha256;
mod isa;
mod miner;
mod vm;
mod asm;

use std::env;
use std::fs;
use std::io::Read;
use std::process;

fn usage() {
    eprintln!("roplang — низкоуровневый язык, исполняемый через SHA-256");
    eprintln!("  roplang run <file.rop>                исполнить программу");
    eprintln!("  roplang asm <file.ropasm> [-o out]    собрать (Proof-of-Work компиляция)");
    eprintln!("  roplang dis <file.rop>                дизассемблировать");
    eprintln!("  roplang mine <mnemonic> [imm]         намайнить одну строку");
    eprintln!("  roplang hash <text>                   SHA-256 (отладка)");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { usage(); process::exit(2); }
    match args[1].as_str() {
        "run" => cmd_run(&args),
        "asm" => cmd_asm(&args),
        "dis" => cmd_dis(&args),
        "mine" => cmd_mine(&args),
        "hash" => {
            let d = sha256::sha256(args[2].as_bytes());
            for b in d { print!("{:02x}", b); }
            println!();
        }
        _ => { usage(); process::exit(2); }
    }
}

fn cmd_run(args: &[String]) {
    if args.len() < 3 { usage(); process::exit(2); }
    let src = fs::read_to_string(&args[2]).expect("не могу прочитать файл");
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).expect("stdin");
    match vm::run(&src, &input) {
        Ok(out) => print!("{}", out),
        Err(e) => { eprintln!("{}", e); process::exit(1); }
    }
}

fn cmd_asm(args: &[String]) {
    if args.len() < 3 { usage(); process::exit(2); }
    let src = fs::read_to_string(&args[2]).expect("не могу прочитать файл");
    let out = args.iter().position(|a| a == "-o").and_then(|i| args.get(i + 1)).cloned()
        .unwrap_or_else(|| args[2].replace(".ropasm", ".rop"));
    match asm::assemble(&src) {
        Ok(prog) => {
            fs::write(&out, &prog).expect("запись файла");
            println!("собрано: {} ({} строк)", out, prog.lines().count());
        }
        Err(e) => { eprintln!("ошибка сборки: {}", e); process::exit(1); }
    }
}

fn cmd_dis(args: &[String]) {
    if args.len() < 3 { usage(); process::exit(2); }
    let src = fs::read_to_string(&args[2]).expect("не могу прочитать файл");
    for (i, line) in src.lines().enumerate() {
        let t = line.trim();
        if t.is_empty() || t.starts_with(';') { continue; }
        let (op, arg) = isa::compile_line(t);
        println!("{:4} {:8} {:>20}  {}", i, isa::op_name(op), arg, t);
    }
}

fn cmd_mine(args: &[String]) {
    if args.len() < 3 { usage(); process::exit(2); }
    let name = args[2].to_lowercase();
    let op = match isa::OPCODES.iter().position(|n| *n == name) {
        Some(p) => p as u8,
        None => { eprintln!("неизвестная мнемоника: {}", name); process::exit(1); }
    };
    let (modulo, value) = match name.as_str() {
        "push" => (65536, args.get(3).and_then(|s| s.parse().ok()).unwrap_or(0)),
        "load" | "store" => (1024, args.get(3).and_then(|s| s.parse().ok()).unwrap_or(0)),
        _ => (0, 0),
    };
    let t0 = std::time::Instant::now();
    let (line, iters) = miner::mine("m", op, modulo, value);
    println!("{}", line);
    eprintln!("итераций: {}, {:.3} с", iters, t0.elapsed().as_secs_f64());
}
