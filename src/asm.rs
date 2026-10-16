use crate::isa::OPCODES;
use crate::miner::mine;
use std::collections::HashMap;

enum Instr {
    Simple(u8),
    Imm(u8, u64),       // push / load / store
    Jump(u8, String),   // jmp / jz / jnz / call
}

fn parse_imm(tok: Option<&&str>) -> Result<u64, String> {
    tok.ok_or_else(|| "не указан аргумент".to_string())?
        .parse::<u64>().map_err(|_| "аргумент должен быть числом".to_string())
}

/// Ассемблер: мнемоники -> майнинг каждой строки -> готовая программа.
pub fn assemble(src: &str) -> Result<String, String> {
    let mut instrs: Vec<Instr> = Vec::new();
    let mut labels: HashMap<String, usize> = HashMap::new();

    for raw in src.lines() {
        let line = raw.split(';').next().unwrap_or("").trim().to_string();
        if line.is_empty() { continue; }
        if line.ends_with(':') {
            labels.insert(line[..line.len()-1].trim().to_string(), instrs.len());
            continue;
        }
        let toks: Vec<&str> = line.split_whitespace().collect();
        let mn = toks[0].to_lowercase();
        let op = OPCODES.iter().position(|x| *x == mn.as_str())
            .ok_or_else(|| format!("неизвестная мнемоника: {}", mn))? as u8;
        instrs.push(match mn.as_str() {
            "push" | "load" | "store" => Instr::Imm(op, parse_imm(toks.get(1))?),
            "jmp" | "jz" | "jnz" | "call" =>
                Instr::Jump(op, toks.get(1).ok_or_else(|| "метка не указана".to_string())?.to_string()),
            _ => Instr::Simple(op),
        });
    }

    let n = instrs.len();
    let mut out = Vec::with_capacity(n);
    let mut total: u64 = 0;
    for (i, ins) in instrs.iter().enumerate() {
        let prefix = format!("r{}", i);
        let (line, iters) = match ins {
            Instr::Simple(op) => mine(&prefix, *op, 0, 0),
            Instr::Imm(op, v) => {
                let modulo = if *op == 1 { 65536 } else { 1024 };
                mine(&prefix, *op, modulo, *v)
            }
            Instr::Jump(op, label) => {
                let target = *labels.get(label).ok_or_else(|| format!("метка не найдена: {}", label))?;
                mine(&prefix, *op, n as u64, target as u64)
            }
        };
        total += iters;
        out.push(line);
    }
    eprintln!("замайнено строк: {}, итераций: {}", n, total);
    Ok(out.join("\n"))
}
