use iced_x86::code_asm::*;
use iced_x86::{Decoder, DecoderOptions, Formatter, IntelFormatter, Instruction};
use std::collections::HashMap;
use std::error::Error;
use std::{env, fmt, fs};
use std::fmt::{format, Formatter as FmtFormatter};
use goblin::elf::Elf;
use std::time::Instant;
use clap::Parser;

mod log {
    use std::fmt;

    const COLOR_RESET: &str = "\x1b[0m";
    const COLOR_RED: &str = "\x1b[0;31m";
    const COLOR_GREEN: &str = "\x1b[0;32m";
    const COLOR_YELLOW: &str = "\x1b[0;33m";
    const COLOR_CYAN: &str = "\x1b[0;35m";
    const COLOR_BLUE: &str = "\x1b[0;34m";

    pub fn info<T: fmt::Display>(msg: T) {
        println!("{}[*] {}{}", COLOR_CYAN, msg, COLOR_RESET);
    }
    pub fn infoNL<T: fmt::Display>(msg: T) {
        print!("{}[*] {}{}", COLOR_CYAN, msg, COLOR_RESET);
    }

    pub fn success<T: fmt::Display>(msg: T) {
        println!("{}[+] {}{}", COLOR_GREEN, msg, COLOR_RESET);
    }
    pub fn successNL<T: fmt::Display>(msg: T) {
        print!("{}[+] {}{}", COLOR_GREEN, msg, COLOR_RESET);
    }
    pub fn error<T: fmt::Display>(msg: T) {
        println!("{}[-] {}{}", COLOR_RED, msg, COLOR_RESET);
    }
    pub fn errorNL<T: fmt::Display>(msg: T) {
        print!("{}[-] {}{}", COLOR_RED, msg, COLOR_RESET);
    }

    pub fn warning<T: fmt::Display>(msg: T) {
        println!("{}[!] {}{}", COLOR_YELLOW, msg, COLOR_RESET);
    }
    pub fn warningNL<T: fmt::Display>(msg: T) {
        print!("{}[!] {}{}", COLOR_YELLOW, msg, COLOR_RESET);
    }

    pub fn code<T: fmt::Display>(msg: T) {
        println!("{}    {}{}", COLOR_BLUE, msg, COLOR_RESET);
    }
}

type AsmFnPtr = fn(&mut CodeAssembler) -> Result<(), iced_x86::IcedError>;
type AsmFnPtr1Op = fn(&mut CodeAssembler, AsmRegister64) -> Result<(), iced_x86::IcedError>;
type AsmFnPtr2Op = fn(&mut CodeAssembler, AsmRegister64, AsmRegister64) -> Result<(), iced_x86::IcedError>;
type AsmFnPtrImm = fn(&mut CodeAssembler, i32) -> Result<(), iced_x86::IcedError>;
type AsmFnPtrRegImm = fn(&mut CodeAssembler, AsmRegister64, i32) -> Result<(), iced_x86::IcedError>;
type AsmFnPtrRegImm64 = fn(&mut CodeAssembler, AsmRegister64, u64) -> Result<(), iced_x86::IcedError>;

enum InstructionType {
    NoOp(AsmFnPtr),
    OneReg(AsmFnPtr1Op), TwoReg(AsmFnPtr2Op), OneImm(AsmFnPtrImm), RegImm(AsmFnPtrRegImm),
    RegImm64(AsmFnPtrRegImm64),
}

pub fn assemble(asm_code: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut imap = HashMap::new();

    imap.insert("ret", InstructionType::NoOp(CodeAssembler::ret));
    imap.insert("nop", InstructionType::NoOp(CodeAssembler::nop));
    imap.insert("syscall", InstructionType::NoOp(CodeAssembler::syscall));
    imap.insert("swapgs", InstructionType::NoOp(CodeAssembler::swapgs));
    imap.insert("iretq", InstructionType::NoOp(CodeAssembler::iretq));
    imap.insert("cpuid", InstructionType::NoOp(CodeAssembler::cpuid));
    imap.insert("rdtsc", InstructionType::NoOp(CodeAssembler::rdtsc));
    imap.insert("leave", InstructionType::NoOp(CodeAssembler::leave));

    imap.insert("pop", InstructionType::OneReg(CodeAssembler::pop));
    imap.insert("push", InstructionType::OneReg(CodeAssembler::push));
    imap.insert("inc", InstructionType::OneReg(CodeAssembler::inc));
    imap.insert("dec", InstructionType::OneReg(CodeAssembler::dec));
    imap.insert("neg", InstructionType::OneReg(CodeAssembler::neg));
    imap.insert("not", InstructionType::OneReg(CodeAssembler::not));
    imap.insert("jmp", InstructionType::OneReg(CodeAssembler::jmp));
    imap.insert("call", InstructionType::OneReg(CodeAssembler::call));

    imap.insert("mov", InstructionType::TwoReg(CodeAssembler::mov));
    imap.insert("xchg", InstructionType::TwoReg(CodeAssembler::xchg));
    imap.insert("add", InstructionType::TwoReg(CodeAssembler::add));
    imap.insert("sub", InstructionType::TwoReg(CodeAssembler::sub));
    imap.insert("xor", InstructionType::TwoReg(CodeAssembler::xor));
    imap.insert("and", InstructionType::TwoReg(CodeAssembler::and));
    imap.insert("or", InstructionType::TwoReg(CodeAssembler::or));
    imap.insert("cmp", InstructionType::TwoReg(CodeAssembler::cmp));
    imap.insert("test", InstructionType::TwoReg(CodeAssembler::test));

    let register_map = HashMap::from([
        ("rax", rax), ("rcx", rcx), ("rdx", rdx), ("rbx", rbx),
        ("rsp", rsp), ("rbp", rbp), ("rsi", rsi), ("rdi", rdi),
        ("r8", r8), ("r9", r9), ("r10", r10), ("r11", r11),
        ("r12", r12), ("r13", r13), ("r14", r14), ("r15", r15),
    ]);

    let mut assembler = CodeAssembler::new(64)?;

    let instructions = asm_code.split(';');

    for instr in instructions {
        let instr = instr.trim();
        if instr.is_empty() {
            continue;
        }

        let parts: Vec<&str> = instr.splitn(2, ' ').collect();
        let mnemonic = parts[0].trim();
        let operands = if parts.len() > 1 { parts[1].trim() } else { "" };

        if let Some(instr_type) = imap.get(mnemonic) {
            match instr_type {
                InstructionType::NoOp(func) => {
                    func(&mut assembler)?;
                },

                InstructionType::OneReg(func) => {
                    if let Some(reg) = register_map.get(operands) {
                        func(&mut assembler, *reg)?;
                    } else {
                        eprintln!("Unknown register: {}", operands);
                        continue;
                    }
                },

                InstructionType::TwoReg(func) => {
                    let op_parts: Vec<&str> = operands.split(',').map(|s| s.trim()).collect();
                    if op_parts.len() != 2 {
                        eprintln!("Expected 2 operands for {}", mnemonic);
                        continue;
                    }

                    if let (Some(reg1), Some(reg2)) = (register_map.get(op_parts[0]), register_map.get(op_parts[1])) {
                        func(&mut assembler, *reg1, *reg2)?;
                    } else {
                        eprintln!("Unknown register(s) in: {}", operands);
                        continue;
                    }
                },

                InstructionType::OneImm(func) => {
                    let imm = if operands.starts_with("0x") {
                        i32::from_str_radix(&operands[2..], 16)?
                    } else {
                        operands.parse::<i32>()?
                    };
                    func(&mut assembler, imm)?;
                },

                InstructionType::RegImm(func) => {
                    let op_parts: Vec<&str> = operands.split(',').map(|s| s.trim()).collect();
                    if op_parts.len() != 2 {
                        eprintln!("Expected 2 operands for {}", mnemonic);
                        continue;
                    }

                    if let Some(reg) = register_map.get(op_parts[0]) {
                        let imm = if op_parts[1].starts_with("0x") {
                            i32::from_str_radix(&op_parts[1][2..], 16)?
                        } else {
                            op_parts[1].parse::<i32>()?
                        };
                        func(&mut assembler, *reg, imm)?;
                    } else {
                        eprintln!("Unknown register: {}", op_parts[0]);
                        continue;
                    }
                },

                InstructionType::RegImm64(func) => {
                    let op_parts: Vec<&str> = operands.split(',').map(|s| s.trim()).collect();
                    if op_parts.len() != 2 {
                        eprintln!("Expected 2 operands for {}", mnemonic);
                        continue;
                    }

                    if let Some(reg) = register_map.get(op_parts[0]) {
                        let imm = if op_parts[1].starts_with("0x") {
                            u64::from_str_radix(&op_parts[1][2..], 16)?
                        } else {
                            op_parts[1].parse::<u64>()?
                        };
                        func(&mut assembler, *reg, imm)?;
                    } else {
                        eprintln!("Unknown register: {}", op_parts[0]);
                        continue;
                    }
                },
            }
        } else {
            eprintln!("Unknown instruction: {}", mnemonic);
            continue;
        }

        log::success(format!("{}", instr));
    }

    let machine_code = assembler.assemble(0x0)?;

    Ok(machine_code)
}

struct XRegion {
    offset_s: u64,
    offset_e: u64,
    size: u64,
    vaddr: u64,
    data: Vec<u8>,
}

struct DisasmContext<'a> {
    regions: &'a [XRegion],
}

struct PatternMatch {
    file_offset: u64,
    vaddr: u64,
    region_index: usize,
    following_bytes: Vec<u8>,
}

fn x_regions(fpath: &str) -> Result<Vec<XRegion>, Box<dyn Error>> {
    let buffer = fs::read(fpath)?;

    let elf = Elf::parse(&buffer)?;

    let mut regions = Vec::new();

    for ph in elf.program_headers {
        if ph.p_type == goblin::elf::program_header::PT_LOAD && (ph.p_flags & goblin::elf::program_header::PF_X) != 0 {
            let start = ph.p_offset as usize;
            let end = (ph.p_offset + ph.p_filesz) as usize;

            if end <= buffer.len() {
                regions.push(XRegion {
                    offset_s: ph.p_offset,
                    offset_e: ph.p_offset + ph.p_filesz,
                    size: ph.p_filesz,
                    vaddr: ph.p_vaddr,
                    data: buffer[start..end].to_vec(),
                });
            }
        }
    }

    Ok(regions)
}

fn bm_search(haystack: &[u8], needle: &[u8]) -> Vec<usize> {
    let mut matches = Vec::new();

    if needle.is_empty() || needle.len() > haystack.len() {
        return matches;
    }

    let mut bad_char = vec![needle.len(); 256];
    for (i, &c) in needle[..needle.len() - 1].iter().enumerate() {
        bad_char[c as usize] = needle.len() - 1 - i;
    }

    let mut i = needle.len() - 1;
    while i < haystack.len() {
        let mut j = needle.len() - 1;
        let mut k = i;

        let mut matched = true;
        while j < needle.len() {
            if haystack[k] != needle[j] {
                matched = false;
                break;
            }

            if j == 0 {
                break;
            }

            j -= 1;
            k -= 1;
        }

        if matched {
            matches.push(k);
        }
        let skip = bad_char[haystack[i] as usize];
        i += skip;
    }

    matches
}

fn f_pat(regions: &[XRegion], pattern: &[u8]) -> Vec<PatternMatch> {
    let mut matches = Vec::new();
    const FOLLOW_BYTES: usize = 64;

    for (region_index, region) in regions.iter().enumerate() {
        let region_matches = bm_search(&region.data, pattern);

        for offset in region_matches {
            let start = offset;
            let end = std::cmp::min(offset + FOLLOW_BYTES, region.data.len());

            matches.push(PatternMatch {
                file_offset: region.offset_s + offset as u64,
                vaddr: region.vaddr + offset as u64,
                region_index,
                following_bytes: region.data[start..end].to_vec(),
            });
        }
    }

    matches
}

fn disass(bytes: &[u8], base_address: u64, num_instructions: usize) -> Vec<(u64, String, Option<u64>)> {
    let mut decoder = Decoder::with_ip(64, bytes, base_address, DecoderOptions::NONE);
    let mut formatter = IntelFormatter::new();
    let mut instructions = Vec::new();
    let mut instruction = Instruction::default();
    let mut output = String::new();

    for _ in 0..num_instructions {
        if !decoder.can_decode() {
            break;
        }

        decoder.decode_out(&mut instruction);
        output.clear();
        formatter.format(&instruction, &mut output);

        let jump_target = if instruction.is_jmp_short_or_near() ||
                             instruction.is_jcc_short_or_near() ||
                             instruction.is_call_near() {
            Some(instruction.near_branch64())
        } else {
            None
        };

        instructions.push((instruction.ip(), output.clone(), jump_target));
    }

    instructions
}

fn vaddr2offset(regions: &[XRegion], vaddr: u64) -> Option<(usize, u64)> {
    for (index, region) in regions.iter().enumerate() {
        if vaddr >= region.vaddr && vaddr < region.vaddr + region.size {
            let offset = vaddr - region.vaddr;
            return Some((index, offset));
        }
    }
    None
}

fn disass_vaddr(regions: &[XRegion], vaddr: u64, num_instructions: usize, indent_level: usize) {
    if let Some((region_index, offset)) = vaddr2offset(regions, vaddr) {
        let region = &regions[region_index];
        if offset < region.data.len() as u64 {
            let start = offset as usize;
            let end = std::cmp::min(start + 64, region.data.len());
            let bytes = &region.data[start..end];

            let instructions = disass(bytes, vaddr, num_instructions);
            let indent = "    ".repeat(indent_level);
            let mut flag = false;
            for (addr, asm, jump_target) in instructions {
                if !flag && indent_level >= 1 {
                    flag = true;
                    let first = "    ".repeat(indent_level-1);
                    log::code(format!("{}└-->0x{:016x}: {}", first, addr, asm));
                } else {
                    log::code(format!("{}0x{:016x}: {}", indent, addr, asm));
                }

                if let Some(target) = jump_target {
                    if indent_level < 2 {
                        disass_vaddr(regions, target, 3, indent_level + 1);
                    }
                }
            }
        }
    } else {
        let indent = "    ".repeat(indent_level);
        log::warning(format!("{}invalid address 0x{:x}", indent, vaddr));
    }
}

fn finder(f_path: &str, target: &[u8],is_disass: bool) -> Result<(), Box<dyn Error>> {
    let st = Instant::now();
    match x_regions(f_path) {
        Ok(xs) => {
            let fp = f_pat(&xs, target);
            let ela = st.elapsed();
            log::info(format!("Finish process in {:.2?}", ela));
            if fp.is_empty() {
                log::warning("Nothing..");
            } else {
                for (i, m) in fp.iter().enumerate() {
                    log::successNL(format!("Instr #{}/{}", i + 1, fp.len()));
                    print!(" Offset: 0x{:x}", m.file_offset);
                    print!(" Vaddr: 0x{:x}", m.vaddr);
                    println!();

                    if is_disass {
                        disass_vaddr(&xs, m.vaddr, 7, 0);
                        println!();
                    }
                }
            }
            Ok(())
        },
        Err(e) => {
            Err(e)
        }
    }
}

#[derive(Parser)]
#[command(version)]
struct Opt {
    #[arg(short = 'f', long = "file")]
    file: String,

    #[arg(short = 'a', long = "asm")]
    asm: String,

    #[arg(short = 'n', long = "no-disass")]
    disass: bool,

}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::parse();
    let asm = opt.asm;
    let elf = opt.file;
    let mc = assemble(&asm)?;
    log::infoNL("Generated Machine Code: ");
    for byte in &mc {
        print!("{:02x}", byte);
    }
    println!("");

    finder(&elf, &mc, !opt.disass)?;

    Ok(())
}