#![allow(warnings)]
use crate::{
    instructions::{BType, IType, JType, RType, SType, UType},
    csr::{CSR, Field, Privilege, Access},
    rom::Rom,
};

use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::io::prelude::*;
use std::{
    fs::{self, File},
    os::macos::raw::stat,
};

#[derive(serde::Serialize, Deserialize)]
pub struct Processor {
    pc: u64,
    regfile: Vec<u64>,
    instruction_memory: Vec<u8>,
    memory: Vec<u8>,
    csrs: Vec<CSR>,
    increment_pc: bool,
}

impl Processor {
    pub fn new_processor(memory_size: usize, rom: &Rom) -> Processor {
        let regfile: Vec<u64> = vec![0; 32];
        let mut csrs: Vec<CSR> = Vec::new();
        for i in 0..4096 {
            csrs.push(CSR::new_csr(String::from(""), Privilege::M));
        }

        let mut instruction_memory: Vec<u8> = (0..0).collect();
        for address in 0..rom.get_length() {
            instruction_memory.push(rom.read_byte(address));             
        }

        let mut memory: Vec<u8> = vec![0; memory_size];

        Processor {
            pc: 0,
            regfile: regfile,
            instruction_memory: instruction_memory,
            memory: memory,
            csrs: csrs,
            increment_pc: true,
        }
    }

    /* ATTRIBUTE ACCESS */
    pub fn get_pc(&self) -> u64 {
        return self.pc;
    }

    pub fn get_instruction_memory(&self) -> Vec<u8> {
        return self.instruction_memory.clone();
    }

    /* DISPLAY STATE */
    pub fn display_state(&self) {
        println!("PC = {}", self.pc);
        for register in 0..self.regfile.len() as usize {
            println!("  x{}: {}", register, self.regfile[register] as i64);
        }
    }

    pub fn add_csr(&mut self, csr: CSR, address: usize) {
        self.csrs[address] = csr;
    }

    /* EXECUTE */
    pub fn step(&mut self) {
        // TODO - read instruction, identify its type, execute the instruction, and increment the PC
        let instruction_word: u32 = (self.instruction_memory[self.pc as usize + 3] as u32) << 24
            | (self.instruction_memory[self.pc as usize + 2] as u32) << 16
            | (self.instruction_memory[self.pc as usize + 1] as u32) << 8
            | (self.instruction_memory[self.pc as usize] as u32);
        self.increment_pc = true;

        if instruction_word & 0xfe00707f == 0x33 {
            let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
            self.add(instruction);
        } else if instruction_word & 0x707f == 0x13 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.addi(instruction);
        } else if instruction_word & 0x707f == 0x1b {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.addiw(instruction);
        } else if instruction_word & 0xfe00707f == 0x3b {
            let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
            self.addw(instruction);
        } else if instruction_word & 0xfe00707f == 0x7033 {
            let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
            self.and(instruction);
        } else if instruction_word & 0x707f == 0x7013 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.andi(instruction);
        } else if instruction_word & 0x7f == 0x17 {
            let instruction: UType = UType::from_bytes(instruction_word.to_le_bytes());
            self.auipc(instruction);
        } else if instruction_word & 0x707f == 0x63 {
            let instruction: BType = BType::from_bytes(instruction_word.to_le_bytes());
            self.beq(instruction);
        } else if instruction_word & 0x707f == 0x5063 {
            let instruction: BType = BType::from_bytes(instruction_word.to_le_bytes());
            self.bge(instruction);
        } else if instruction_word & 0x707f == 0x7063 {
            let instruction: BType = BType::from_bytes(instruction_word.to_le_bytes());
            self.bgeu(instruction);
        } else if instruction_word & 0x707f == 0x4063 {
            let instruction: BType = BType::from_bytes(instruction_word.to_le_bytes());
            self.blt(instruction);
        } else if instruction_word & 0x707f == 0x6063 {
            let instruction: BType = BType::from_bytes(instruction_word.to_le_bytes());
            self.bltu(instruction);
        } else if instruction_word & 0x707f == 0x1063 {
            let instruction: BType = BType::from_bytes(instruction_word.to_le_bytes());
            self.bne(instruction);
        } else if instruction_word & 0x7f == 0x6f {
            let instruction: JType = JType::from_bytes(instruction_word.to_le_bytes());
            self.jal(instruction);
        } else if instruction_word & 0x707f == 0x67 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.jalr(instruction);
        } else if instruction_word & 0x707f == 0x3 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.lb(instruction);
        } else if instruction_word & 0x707f == 0x4003 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.lbu(instruction);
        } else if instruction_word & 0x707f == 0x3003 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.ld(instruction);
        } else if instruction_word & 0x707f == 0x1003 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.lh(instruction);
        } else if instruction_word & 0x707f == 0x5003 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.lhu(instruction);
        } else if instruction_word & 0x7f == 0x37 {
            let instruction: UType = UType::from_bytes(instruction_word.to_le_bytes());
            self.lui(instruction);
        } else if instruction_word & 0x707f == 0x2003 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.lw(instruction);
        } else if instruction_word & 0x707f == 0x6003 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.lwu(instruction);
        } else if instruction_word & 0xfe00707f == 0x6033 {
            let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
            self.or(instruction);
        } else if instruction_word & 0x707f == 0x6013 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.ori(instruction);
        } else if instruction_word & 0x707f == 0x23 {
            let instruction: SType = SType::from_bytes(instruction_word.to_le_bytes());
            self.sb(instruction);
        } else if instruction_word & 0x707f == 0x3023 {
            let instruction: SType = SType::from_bytes(instruction_word.to_le_bytes());
            self.sd(instruction);
        } else if instruction_word & 0x707f == 0x1023 {
            let instruction: SType = SType::from_bytes(instruction_word.to_le_bytes());
            self.sh(instruction);
        } else if instruction_word & 0xfe00707f == 0x1033 {
            let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
            self.sll(instruction);
        } else if instruction_word & 0xfc00707f == 0x1013 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.slli(instruction);
        } else if instruction_word & 0xfe00707f == 0x101b {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.slliw(instruction);
        } else if instruction_word & 0xfe00707f == 0x103b {
            let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
            self.sllw(instruction);
        } else if instruction_word & 0xfe00707f == 0x2033 {
            let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
            self.slt(instruction);
        } else if instruction_word & 0x707f == 0x2013 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.slti(instruction);
        } else if instruction_word & 0x707f == 0x3013 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.sltiu(instruction);
        } else if instruction_word & 0xfe00707f == 0x3033 {
            let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
            self.sltu(instruction);
        } else if instruction_word & 0xfe00707f == 0x40005033 {
            let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
            self.sra(instruction);
        } else if instruction_word & 0xfc00707f == 0x40005013 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.srai(instruction);
        } else if instruction_word & 0xfe00707f == 0x4000501b {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.sraiw(instruction);
        } else if instruction_word & 0xfe00707f == 0x4000503b {
            let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
            self.sraw(instruction);
        } else if instruction_word & 0xfe00707f == 0x5033 {
            let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
            self.srl(instruction);
        } else if instruction_word & 0xfc00707f == 0x5013 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.srli(instruction);
        } else if instruction_word & 0xfe00707f == 0x501b {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.srliw(instruction);
        } else if instruction_word & 0xfe00707f == 0x503b {
            let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
            self.srlw(instruction);
        } else if instruction_word & 0xfe00707f == 0x40000033 {
            let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
            self.sub(instruction);
        } else if instruction_word & 0xfe00707f == 0x4000003b {
            let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
            self.subw(instruction);
        } else if instruction_word & 0x707f == 0x2023 {
            let instruction: SType = SType::from_bytes(instruction_word.to_le_bytes());
            self.sw(instruction);
        } else if instruction_word & 0xfe00707f == 0x4033 {
            let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
            self.xor(instruction);
        } else if instruction_word & 0x707f == 0x4013 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.xori(instruction);
        }

        if self.increment_pc {
            self.pc += 4;
        }
    }

    /* IMMEDIATE OPERATIONS */
    fn addi(&mut self, instruction: IType) {
        // rd = rs1 + imm
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        self.regfile[instruction.rd() as usize] = rs1_value.wrapping_add(instruction.imm() as u64);
    }

    fn slli(&mut self, instruction: IType) {
        // rd = rs1 << imm[0:4]
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let result = rs1_value << ((instruction.imm() as u32) & 0x3f) as u64;
        self.regfile[instruction.rd() as usize] = result;
    }

    fn slti(&mut self, instruction: IType) {
        // rd = (rs1 < imm) ? 1 : 0
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let result: u64 =
            if (rs1_value as i64) < Self::sign_extend(instruction.imm() as u64, 12) as i64 {
                1
            } else {
                0
            };
        self.regfile[instruction.rd() as usize] = result;
    }

    fn sltiu(&mut self, instruction: IType) {
        // rd = (rs1 < imm) ? 1 : 0
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let result: u64 = if rs1_value < (Self::sign_extend(instruction.imm() as u64, 12) as u64) {
            1
        } else {
            0
        };
        self.regfile[instruction.rd() as usize] = result;
    }

    fn xori(&mut self, instruction: IType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        self.regfile[instruction.rd() as usize] = rs1_value ^ (instruction.imm() as u64);
    }

    fn srli(&mut self, instruction: IType) {
        // rd = rs1_value >> imm[0:4]
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let result: u64 = (rs1_value) >> ((instruction.imm() as u32) & 0x3f) as u64;
        self.regfile[instruction.rd() as usize] = result;
    }

    fn srai(&mut self, instruction: IType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let mut result: u64 = ((rs1_value as i64) >> ((instruction.imm() as u32) & 0x3f)) as u64;
        self.regfile[instruction.rd() as usize] = result;
    }

    fn ori(&mut self, instruction: IType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        self.regfile[instruction.rd() as usize] = rs1_value | (instruction.imm() as u64);
    }

    fn andi(&mut self, instruction: IType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        self.regfile[instruction.rd() as usize] = rs1_value & (instruction.imm() as u64);
    }

    fn addiw(&mut self, instruction: IType) {
        let rs1_value_half: u32 = self.regfile[instruction.rs1() as usize] as u32;
        let imm12_sign_extended = Self::sign_extend(instruction.imm() as u64, 12);
        let result = (rs1_value_half).wrapping_add(imm12_sign_extended as u32) as u64;
        self.regfile[instruction.rd() as usize] = result;
    }

    fn slliw(&mut self, instruction: IType) {
        // rd = rs1 << imm[0:4]
        let rs1_value_half: u32 = self.regfile[instruction.rs1() as usize] as u32;
        let result: u64 = Self::sign_extend(
            (rs1_value_half << ((instruction.imm() as u32) & 0x32)) as u64,
            32,
        ) as u64;
        self.regfile[instruction.rd() as usize] = result;
    }

    fn srliw(&mut self, instruction: IType) {
        // rd = rs1_value >> imm[0:4]
        let rs1_value_half: u32 = self.regfile[instruction.rs1() as usize] as u32;
        let result: u64 = Self::sign_extend(
            (rs1_value_half >> ((instruction.imm() as u32) & 0x32)) as u64,
            32,
        ) as u64;
        self.regfile[instruction.rd() as usize] = result;
    }

    fn sraiw(&mut self, instruction: IType) {
        let rs1_value_half: u32 = self.regfile[instruction.rs1() as usize] as u32;
        let mut result: u64 = ((rs1_value_half as i32) >> (instruction.imm() as u32) as i64) as u64;
        self.regfile[instruction.rd() as usize] = result;
    }

    /* LOAD OPERATIONS */
    fn lb(&mut self, instruction: IType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let address: usize = (rs1_value as i64)
            .wrapping_add(Self::sign_extend(instruction.imm() as u64, 12) as i64)
            as usize;
        let value = Self::sign_extend(self.memory[address] as u64, 8) as u64;
        self.regfile[instruction.rd() as usize] = value;
    }

    fn lh(&mut self, instruction: IType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let address: usize = (rs1_value as i64)
            .wrapping_add(Self::sign_extend(instruction.imm() as u64, 12) as i64)
            as usize;
        let value = Self::sign_extend(
            (self.memory[address + 1] as u64) << 8 | self.memory[address] as u64,
            16,
        ) as u64;
        self.regfile[instruction.rd() as usize] = value;
    }

    fn lw(&mut self, instruction: IType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let address: usize = (rs1_value as i64)
            .wrapping_add(Self::sign_extend(instruction.imm() as u64, 12) as i64)
            as usize;
        let value = Self::sign_extend(
            (self.memory[address + 3] as u64) << 24
                | (self.memory[address + 2] as u64) << 16 as u64
                | (self.memory[address + 1] as u64) << 8
                | self.memory[address] as u64,
            32,
        ) as u64;
        self.regfile[instruction.rd() as usize] = value;
    }

    fn lwu(&mut self, instruction: IType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let address: usize = (rs1_value as i64)
            .wrapping_add(Self::sign_extend(instruction.imm() as u64, 12) as i64)
            as usize;
        let value = (self.memory[address + 3] as u64) << 24
            | (self.memory[address + 2] as u64) << 16 as u64
            | (self.memory[address + 1] as u64) << 8
            | self.memory[address] as u64;
        self.regfile[instruction.rd() as usize] = value;
    }

    fn lbu(&mut self, instruction: IType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let address: usize = (rs1_value as i64)
            .wrapping_add(Self::sign_extend(instruction.imm() as u64, 12) as i64)
            as usize;
        let value = (self.memory[address]) as u64;
        self.regfile[instruction.rd() as usize] = value;
    }

    fn lhu(&mut self, instruction: IType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let address: usize = (rs1_value as i64)
            .wrapping_add(Self::sign_extend(instruction.imm() as u64, 12) as i64)
            as usize;
        let value = (self.memory[address + 1] as u64) << 8 | self.memory[address] as u64;
        self.regfile[instruction.rd() as usize] = value;
    }

    fn ld(&mut self, instruction: IType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let address: usize = (rs1_value as i64)
            .wrapping_add(Self::sign_extend(instruction.imm() as u64, 12) as i64)
            as usize;
        let value = (self.memory[address + 7] as u64) << 56
            | (self.memory[address + 6] as u64) << 48
            | (self.memory[address + 5] as u64) << 40
            | (self.memory[address + 4] as u64) << 32
            | (self.memory[address + 3] as u64) << 24
            | (self.memory[address + 2] as u64) << 16
            | (self.memory[address + 1] as u64) << 8
            | self.memory[address] as u64;
        self.regfile[instruction.rd() as usize] = value;
    }

    /* REGISTER TYPE OPERATIONS */
    fn add(&mut self, instruction: RType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let rs2_value: u64 = self.regfile[instruction.rs2() as usize];
        let result = rs1_value.wrapping_add(rs2_value);
        self.regfile[instruction.rd() as usize] = rs1_value.wrapping_add(rs2_value);
    }

    fn sub(&mut self, instruction: RType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let rs2_value: u64 = self.regfile[instruction.rs2() as usize];
        let result = rs1_value.wrapping_add(rs2_value);
        self.regfile[instruction.rd() as usize] = rs1_value.wrapping_sub(rs2_value);
    }

    fn xor(&mut self, instruction: RType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let rs2_value: u64 = self.regfile[instruction.rs2() as usize];
        let result = rs1_value.wrapping_add(rs2_value);
        self.regfile[instruction.rd() as usize] = rs1_value ^ rs2_value;
    }

    fn or(&mut self, instruction: RType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let rs2_value: u64 = self.regfile[instruction.rs2() as usize];
        let result = rs1_value.wrapping_add(rs2_value);
        self.regfile[instruction.rd() as usize] = rs1_value | rs2_value;
    }

    fn and(&mut self, instruction: RType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let rs2_value: u64 = self.regfile[instruction.rs2() as usize];
        let result = rs1_value.wrapping_add(rs2_value);
        self.regfile[instruction.rd() as usize] = rs1_value & rs2_value;
    }

    fn sll(&mut self, instruction: RType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let rs2_value: u64 = self.regfile[instruction.rs2() as usize];
        let result = rs1_value.wrapping_shl((rs2_value % 32) as u32);
        self.regfile[instruction.rd() as usize] = result;
    }

    fn srl(&mut self, instruction: RType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let rs2_value: u64 = self.regfile[instruction.rs2() as usize];
        let result = rs1_value.wrapping_shr((rs2_value % 32) as u32);
        self.regfile[instruction.rd() as usize] = result;
    }

    fn sra(&mut self, instruction: RType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let rs2_value: u64 = self.regfile[instruction.rs2() as usize];
        let result = (rs1_value as i64).wrapping_shr((rs2_value % 32) as u32) as u64;
        self.regfile[instruction.rd() as usize] = result;
    }

    fn slt(&mut self, instruction: RType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let rs2_value: u64 = self.regfile[instruction.rs2() as usize];
        let result: u64 = if rs1_value < rs2_value { 1 } else { 0 };
        self.regfile[instruction.rd() as usize] = result;
    }

    fn sltu(&mut self, instruction: RType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let rs2_value: u64 = self.regfile[instruction.rs2() as usize];
        let result: u64 = if rs1_value < rs2_value { 1 } else { 0 };
        self.regfile[instruction.rd() as usize] = result;
    }

    fn addw(&mut self, instruction: RType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let rs2_value: u64 = self.regfile[instruction.rs2() as usize];
        let result = (rs1_value as u32).wrapping_add(rs2_value as u32) as i64 as u64;
        self.regfile[instruction.rd() as usize] = result;
    }

    fn subw(&mut self, instruction: RType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let rs2_value: u64 = self.regfile[instruction.rs2() as usize];
        let result =
            Self::sign_extend((rs1_value as u32).wrapping_sub(rs2_value as u32) as u64, 32) as u64;
        self.regfile[instruction.rd() as usize] = result;
    }

    fn srlw(&mut self, instruction: RType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let rs2_value: u64 = self.regfile[instruction.rs2() as usize];
        let result =
            Self::sign_extend(rs1_value.wrapping_shr((rs2_value & 0x31) as u32) as u64, 32) as u64;
        self.regfile[instruction.rd() as usize] = result;
    }

    fn sraw(&mut self, instruction: RType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let rs2_value: u64 = self.regfile[instruction.rs2() as usize];
        let result = Self::sign_extend(
            (rs1_value as i32).wrapping_shr((rs2_value & 0x31) as u32) as u64,
            32,
        ) as u64;
        self.regfile[instruction.rd() as usize] = result;
    }

    fn sllw(&mut self, instruction: RType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let rs2_value: u64 = self.regfile[instruction.rs2() as usize];
        let result = Self::sign_extend(
            (rs1_value as u32).wrapping_shl((rs2_value & 0x31) as u32) as u64,
            32,
        ) as u64;
        self.regfile[instruction.rd() as usize] = result;
    }

    /* STORE OPERATIONS */
    fn sb(&mut self, instruction: SType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let rs2_value: u64 = self.regfile[instruction.rs2() as usize];
        let address: usize = (rs1_value as u64).wrapping_add(Self::sign_extend(
            ((instruction.imm_upper() as u32) << 5 | instruction.imm_lower() as u32) as u64,
            12,
        ) as u64) as usize;
        self.memory[address] = rs2_value as u8;
    }

    fn sh(&mut self, instruction: SType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let rs2_value: u64 = self.regfile[instruction.rs2() as usize];
        let address: usize = (rs1_value as u64).wrapping_add(Self::sign_extend(
            ((instruction.imm_upper() as u32) << 5 | instruction.imm_lower() as u32) as u64,
            12,
        ) as u64) as usize;
        self.memory[address + 1] = (rs2_value >> 8) as u8;
        self.memory[address] = rs2_value as u8;
    }

    fn sw(&mut self, instruction: SType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let rs2_value: u64 = self.regfile[instruction.rs2() as usize];
        let address: usize = (rs1_value as u64).wrapping_add(Self::sign_extend(
            ((instruction.imm_upper() as u32) << 5 | instruction.imm_lower() as u32) as u64,
            12,
        ) as u64) as usize;

        self.memory[address + 3] = (rs2_value >> 24) as u8;
        self.memory[address + 2] = (rs2_value >> 16) as u8;
        self.memory[address + 1] = (rs2_value >> 8) as u8;
        self.memory[address] = rs2_value as u8;
    }

    fn sd(&mut self, instruction: SType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let rs2_value: u64 = self.regfile[instruction.rs2() as usize];
        let address: usize = (rs1_value as u64).wrapping_add(Self::sign_extend(
            ((instruction.imm_upper() as u32) << 5 | instruction.imm_lower() as u32) as u64,
            12,
        ) as u64) as usize;

        self.memory[address + 7] = (rs2_value >> 56) as u8;
        self.memory[address + 6] = (rs2_value >> 48) as u8;
        self.memory[address + 5] = (rs2_value >> 40) as u8;
        self.memory[address + 4] = (rs2_value >> 32) as u8;
        self.memory[address + 3] = (rs2_value >> 24) as u8;
        self.memory[address + 2] = (rs2_value >> 16) as u8;
        self.memory[address + 1] = (rs2_value >> 8) as u8;
        self.memory[address] = rs2_value as u8;
    }

    /* BRANCH OPERATIONS */
    fn beq(&mut self, instruction: BType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let rs2_value: u64 = self.regfile[instruction.rs2() as usize];
        if (rs1_value as i64) == (rs2_value as i64) {
            let imm: i32 = Self::sign_extend(
                ((((instruction.imm_upper() as u32) & 0x7f) << 5)
                    | ((instruction.imm_lower() as u32) & 0x1 << 10)
                    | (((instruction.imm_upper() as u32) & 0x3f) << 5)
                    | (instruction.imm_lower() as u32) & 0x1e) as u64,
                12,
            ) as i32;

            let result = (self.pc) as i64 + (imm as i64);
            self.pc = result as u64;
            self.increment_pc = false;
        }
    }

    fn bne(&mut self, instruction: BType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let rs2_value: u64 = self.regfile[instruction.rs2() as usize];
        if (rs1_value as i64) != (rs2_value as i64) {
            let imm: i32 = Self::sign_extend(
                ((((instruction.imm_upper() as u32) & 0x7f) << 5)
                    | ((instruction.imm_lower() as u32) & 0x1 << 10)
                    | (((instruction.imm_upper() as u32) & 0x3f) << 5)
                    | (instruction.imm_lower() as u32) & 0x1e) as u64,
                12,
            ) as i32;

            let result = (self.pc) as i64 + (imm as i64);
            self.pc = result as u64;
            self.increment_pc = false;
        }
    }

    fn blt(&mut self, instruction: BType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let rs2_value: u64 = self.regfile[instruction.rs2() as usize];
        if (rs1_value as i64) < (rs2_value as i64) {
            let imm: i32 = Self::sign_extend(
                ((((instruction.imm_upper() as u32) & 0x7f) << 5)
                    | ((instruction.imm_lower() as u32) & 0x1 << 10)
                    | (((instruction.imm_upper() as u32) & 0x3f) << 5)
                    | (instruction.imm_lower() as u32) & 0x1e) as u64,
                12,
            ) as i32;

            let result = (self.pc) as i64 + (imm as i64);
            self.pc = result as u64;
            self.increment_pc = false;
        }
    }

    fn bge(&mut self, instruction: BType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let rs2_value: u64 = self.regfile[instruction.rs2() as usize];
        if (rs1_value as i64) >= (rs2_value as i64) {
            let imm: i32 = Self::sign_extend(
                ((((instruction.imm_upper() as u32) & 0x7f) << 5)
                    | ((instruction.imm_lower() as u32) & 0x1 << 10)
                    | (((instruction.imm_upper() as u32) & 0x3f) << 5)
                    | (instruction.imm_lower() as u32) & 0x1e) as u64,
                12,
            ) as i32;

            let result = (self.pc) as i64 + (imm as i64);
            self.pc = result as u64;
            self.increment_pc = false;
        }
    }

    fn bltu(&mut self, instruction: BType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let rs2_value: u64 = self.regfile[instruction.rs2() as usize];
        if rs1_value >= rs2_value {
            let imm: i32 = Self::sign_extend(
                ((((instruction.imm_upper() as u32) & 0x7f) << 5)
                    | ((instruction.imm_lower() as u32) & 0x1 << 10)
                    | (((instruction.imm_upper() as u32) & 0x3f) << 5)
                    | (instruction.imm_lower() as u32) & 0x1e) as u64,
                12,
            ) as i32;

            let result = (self.pc) as i64 + (imm as i64);
            self.pc = result as u64;
            self.increment_pc = false;
        }
    }

    fn bgeu(&mut self, instruction: BType) {
        let rs1_value: u64 = self.regfile[instruction.rs1() as usize];
        let rs2_value: u64 = self.regfile[instruction.rs2() as usize];
        if rs1_value >= rs2_value {
            let imm: i32 = Self::sign_extend(
                ((((instruction.imm_upper() as u32) & 0x7f) << 5)
                    | ((instruction.imm_lower() as u32) & 0x1 << 10)
                    | (((instruction.imm_upper() as u32) & 0x3f) << 5)
                    | (instruction.imm_lower() as u32) & 0x1e) as u64,
                12,
            ) as i32;

            let result = (self.pc) as i64 + (imm as i64);
            self.pc = result as u64;
            self.increment_pc = false;
        }
    }

    /* JUMP OPERATIONS */
    fn jal(&mut self, instruction: JType) {
        let imm: i64 = Self::sign_extend(
            (((instruction.imm_20() as u32) << 20)
                | ((instruction.imm_12_19() as u32) << 12)
                | ((instruction.imm_11() as u32) << 11)
                | ((instruction.imm_1_10() as u32) << 1)) as u64,
            20,
        ) as i64;

        let stored_pc: u64 = self.pc + 4;
        let new_pc: u64 = self.pc + imm as u64;

        self.regfile[instruction.rd() as usize] = stored_pc;
        self.pc = new_pc;
        self.increment_pc = false;
    }

    fn jalr(&mut self, instruction: IType) {
        let imm = Self::sign_extend(instruction.imm() as u64, 12);

        let stored_pc: u64 = self.pc + 4;
        let new_pc = self.regfile[instruction.rs1() as usize] + imm as u64;

        self.regfile[instruction.rd() as usize] = stored_pc;
        self.pc = new_pc;
        self.increment_pc = false;
    }

    fn lui(&mut self, instruction: UType) {
        let imm: u32 = instruction.imm() as u32;
        let result = Self::sign_extend((imm << 12) as u64, 32);
        self.regfile[instruction.rd() as usize] = result as u64;
    }

    fn auipc(&mut self, instruction: UType) {
        let imm: u64 = instruction.imm() as u64;
        let result = self.pc + Self::sign_extend(imm << 12, 32) as u64;
        self.regfile[instruction.rd() as usize] = result as u64;
    }

    pub fn sign_extend(value: u64, length: u8) -> i64 {
        let sign_bit = 1u64 << (length - 1);
        if value & sign_bit != 0 {
            (value as i64) | !((1 << length) - 1) as i64
        } else {
            value as i64
        }
    }

    /* SERIALIZE/DESERIALIZE */
    pub fn store_state(&mut self, filename: &str) {
        let mut file = File::create("processor_state.json");
        let j = serde_json::to_string(self).unwrap();
        fs::write(filename, j);
    }

    pub fn load_state(&mut self, filename: &str) {
        let file = fs::read_to_string(filename).unwrap();
        let json: serde_json::Value = serde_json::from_str(&file).unwrap();
        
        self.pc = json["pc"].as_u64().unwrap();

        for i in 0..json["regfile"].as_array().unwrap().len() {
            self.regfile[i] = json["regfile"].as_array().unwrap()[i].as_u64().unwrap();
        }
        for i in 0..json["instruction_memory"].as_array().unwrap().len() {
            self.instruction_memory[i] = json["instruction_memory"].as_array().unwrap()[i].as_u64().unwrap() as u8;
        }
        for i in 0..json["memory"].as_array().unwrap().len() {
            self.memory[i] = json["memory"].as_array().unwrap()[i].as_u64().unwrap() as u8;
        }
    }
}
