// --- Day 23: Safe Cracking ---
//
// This is one of the top floors of the nicest tower in EBHQ. The Easter Bunny's private office
// is here, complete with a safe hidden behind a painting, and who wouldn't hide a star in a
// safe behind a painting?
//
// The safe has a digital screen and keypad for code entry. A sticky note attached to the safe
// has a password hint on it: "eggs". The painting is of a large rabbit coloring some eggs.
// You see 7.
//
// When you go to type the code, though, nothing appears on the display; instead, the keypad
// comes apart in your hands, apparently having been smashed. Behind it is some kind of
// socket - one that matches a connector in your prototype computer! You pull apart the smashed
// keypad and extract the logic circuit, plug it into your computer, and plug your computer
// into the safe.
//
// Now, you just need to figure out what output the keypad would have sent to the safe.
// You extract the assembunny code from the logic chip (your puzzle input).
// The code looks like it uses almost the same architecture and instruction set that
// the monorail computer used! You should be able to use the same assembunny interpreter
// for this as you did there, but with one new instruction:
//
// tgl x toggles the instruction x away (pointing at instructions like jnz does: positive means
// forward; negative means backward):
//
//  - For one-argument instructions, inc becomes dec, and all other one-argument instructions
//    become inc.
//  - For two-argument instructions, jnz becomes cpy, and all other two-instructions become jnz.
//  - The arguments of a toggled instruction are not affected.
//  - If an attempt is made to toggle an instruction outside the program, nothing happens.
//  - If toggling produces an invalid instruction (like cpy 1 2) and an attempt is later made to
//    execute that instruction, skip it instead.
//  - If tgl toggles itself (for example, if a is 0, tgl a would target itself and become inc a),
//    the resulting instruction is not executed until the next time it is reached.
//
// For example, given this program:
//
// cpy 2 a
// tgl a
// tgl a
// tgl a
// cpy 1 a
// dec a
// dec a
//
// cpy 2 a initializes register a to 2.
// The first tgl a toggles an instruction a (2) away from it, which changes the third
// tgl a into inc a.
// The second tgl a also modifies an instruction 2 away from it, which changes the cpy 1 a
// into jnz 1 a.
// The fourth line, which is now inc a, increments a to 3.
// Finally, the fifth line, which is now jnz 1 a, jumps a (3) instructions ahead, skipping
// the dec a instructions.
// In this example, the final value in register a is 3.
//
// The rest of the electronics seem to place the keypad entry (the number of eggs, 7) in
// register a, run the code, and then send the value left in register a to the safe.
//
// What value should be sent to the safe?
//
// --- Part Two ---
//
// The safe doesn't open, but it does make several angry noises to express its frustration.
//
// You're quite sure your logic is working correctly, so the only other thing is... you check
// the painting again. As it turns out, colored eggs are still eggs. Now you count 12.
//
// As you run the program with this new input, the prototype computer begins to overheat. You
// wonder what's taking so long, and whether the lack of any instruction more powerful
// than "add one" has anything to do with it. Don't bunnies usually multiply?
//
// Anyway, what value should actually be sent to the safe?

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
    memory: &'a mut [String],
}

impl<'a> CPU<'a> {
    fn tick(&mut self) {
        // Halt if PC points to illegal address.
        if self.pc < 0 || self.pc >= (self.memory.len()) as i32 {
            self.halt = true;
            return;
        }

        // Fetch
        let instruction = self.memory[self.pc as usize].clone();
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
            "jnz" => {
                let op_one = tokens[1];
                let op_two = tokens[2];
                self.inst_jnz(op_one, op_two);
            }
            "inc" => {
                let reg = tokens[1].chars().next().unwrap();
                self.inst_inc(reg);
            }
            "dec" => {
                let reg = tokens[1].chars().next().unwrap();
                self.inst_dec(reg);
            }
            "tgl" => {
                let src = tokens[1].chars().next().unwrap();
                self.inst_tgl_x(src);
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

    fn inst_jnz(&mut self, op_one: &str, op_two: &str) {
        let cond = match op_one.parse::<i32>() {
            Ok(imm) => imm,
            Err(_) => {
                let reg = op_one.chars().next().unwrap();
                *self.chr_to_reg(reg)
            }
        };
        if cond == 0 {
            return;
        }

        let offset = match op_two.parse::<i32>() {
            Ok(imm) => imm,
            Err(_) => {
                let reg = op_two.chars().next().unwrap();
                *self.chr_to_reg(reg)
            }
        };

        self.pc += offset - 1;
    }

    fn inst_tgl_x(&mut self, reg: char) {
        let offset = *self.chr_to_reg(reg);
        let address = self.pc + offset - 1;
        if address < 0 || address >= self.memory.len() as i32 {
            return;
        }

        let instruction = self.memory[address as usize].clone();
        let tokens: Vec<&str> = instruction.split_whitespace().collect();
        let opcode = tokens[0];

        let new_instruction = match opcode {
            "inc" => "dec ".to_string() + tokens[1],
            "dec" | "tgl" => "inc ".to_string() + tokens[1],
            "jnz" => "cpy ".to_string() + tokens[1] + " " + tokens[2],
            "cpy" => "jnz ".to_string() + tokens[1] + " " + tokens[2],
            _ => {
                panic!("Unknown instruction: {}", instruction);
            }
        };
        self.memory[address as usize] = new_instruction;
    }

    fn new(memory: &mut [String]) -> CPU {
        CPU {
            a: 7, // Init to 7
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
    let mut memory: Vec<String> = buf.lines()
        .map(|l| l.expect("Could not read data."))
        .collect();

    let mut cpu = CPU::new(&mut memory);
    while !cpu.halt {
        // println!("{:?}", cpu);
        cpu.tick();
    }
    println!("CPU register a := {}", cpu.a);
}
