// --- Day 12: Leonardo's Monorail ---
//
// You finally reach the top floor of this building: a garden with a slanted glass ceiling.
// Looks like there are no more stars to be had.
//
// While sitting on a nearby bench amidst some tiger lilies, you manage to decrypt
// some of the files you extracted from the servers downstairs.
//
// According to these documents, Easter Bunny HQ isn't just this building - it's a collection of
// buildings in the nearby area. They're all connected by a local monorail, and there's another
// building not far from here! Unfortunately, being night, the monorail is currently not operating.
//
// You remotely connect to the monorail control systems and discover that the boot sequence
// expects a password. The password-checking logic (your puzzle input) is easy to extract,
// but the code it uses is strange: it's assembunny code designed for the new computer you
// just assembled. You'll have to execute the code and get the password.
//
// The assembunny code you've extracted operates on four registers (a, b, c, and d) that
// start at 0 and can hold any integer. However, it seems to make use of only a few instructions:
//
//  - cpy x y copies x (either an integer or the value of a register) into register y.
//  - inc x increases the value of register x by one.
//  - dec x decreases the value of register x by one.
//  - jnz x y jumps to an instruction y away (positive means forward; negative means backward),
//    but only if x is not zero.
//
// The jnz instruction moves relative to itself: an offset of -1 would continue at
// the previous instruction, while an offset of 2 would skip over the next instruction.
//
// For example:
//
// cpy 41 a
// inc a
// inc a
// dec a
// jnz a 2
// dec a
//
// The above code would set register a to 41, increase its value by 2, decrease its value by 1,
// and then skip the last dec a (because a is not zero, so the jnz a 2 skips it),
// leaving register a at 42. When you move past the last instruction, the program halts.
//
// After executing the assembunny code in your puzzle input, what value is left in register a?
//
// --- Part Two ---
//
// As you head down the fire escape to the monorail, you notice it didn't start; register c needs
// to be initialized to the position of the ignition key.
//
// If you instead initialize register c to be 1, what value is now left in register a?

use std::env;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
struct CPU<'a> {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    pc: i32,
    halt: bool,
    memory: &'a [String],
}

impl<'a> CPU<'a> {
    fn tick(&mut self) {
        if self.pc < 0 || self.pc >= self.memory.len() as i32 {
            self.halt = true;
            return;
        }

        // Fetch
        let ref instruction = self.memory[self.pc as usize];
        self.pc += 1;

        // Decode and execute
        let tokens: Vec<&str> = instruction.split_whitespace().collect();
        let opcode = tokens[0];
        match opcode {
            "cpy" => {
                let op = tokens[1];
                let dst = tokens[2].chars().next().unwrap();
                match op.parse::<i32>() {
                    Ok(imm) => {
                        self.inst_cpy_imm(dst, imm);
                    }
                    Err(_) => {
                        self.inst_cpy(op.chars().next().unwrap(), dst);
                    }
                }
            }
            "inc" => {
                let dst = tokens[1].chars().next().unwrap();
                self.inst_inc(dst);
            }
            "dec" => {
                let dst = tokens[1].chars().next().unwrap();
                self.inst_dec(dst);
            }
            "jnz" => {
                let op = tokens[1];
                let mut offset: i32 = tokens[2].parse().unwrap();
                // PC is incremented after fetch
                offset -= 1;
                match op.parse::<i32>() {
                    Ok(imm) => {
                        self.inst_jnz_imm(imm, offset);
                    }
                    Err(_) => {
                        self.inst_jnz(op.chars().next().unwrap(), offset);
                    }
                }
            }
            _ => panic!("Illegal instruction: {}", opcode),
        }
    }

    fn chr_to_reg(&mut self, reg: char) -> &mut i32 {
        let rv = match reg {
            'a' => &mut self.a,
            'b' => &mut self.b,
            'c' => &mut self.c,
            'd' => &mut self.d,
            _ => panic!("Illegal register: `{}`", reg),
        };
        rv
    }

    fn inst_inc(&mut self, reg: char) {
        *self.chr_to_reg(reg) += 1;
    }

    fn inst_dec(&mut self, reg: char) {
        *self.chr_to_reg(reg) -= 1;
    }

    fn inst_cpy(&mut self, src: char, dst: char) {
        *self.chr_to_reg(dst) = *self.chr_to_reg(src);
    }

    fn inst_cpy_imm(&mut self, dst: char, imm: i32) {
        *self.chr_to_reg(dst) = imm;
    }

    fn inst_jnz(&mut self, reg: char, offset: i32) {
        if *self.chr_to_reg(reg) == 0 {
            return;
        }
        self.pc += offset;
    }

    fn inst_jnz_imm(&mut self, imm: i32, offset: i32) {
        if imm == 0 {
            return;
        }
        self.pc += offset;
    }

    fn new(memory: &[String]) -> CPU {
        CPU {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            pc: 0,
            halt: false,
            memory: memory,
        }
    }
}

fn main() {
    let prog_name: String = env::args().nth(0).unwrap();
    if env::args().len() < 2 {
        println!("{} INPUT", prog_name);
        return;
    }
    let file_name: String = env::args().nth(1).unwrap();
    let path = Path::new(&file_name);
    let file = File::open(&path).expect("Couldn't open file.");
    let buf = BufReader::new(file);
    let memory: Vec<String> = buf.lines()
        .map(|l| l.expect("Could not read data."))
        .collect();

    let mut cpu = CPU::new(&memory);
    while !cpu.halt {
        cpu.tick();
    }
    println!("CPU register a := {}", cpu.a);
}
