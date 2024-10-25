#![allow(warnings)]
use crate::{
    instruction_memory::InstructionMemory,
    instructions::{BType, IType, JType, RType, SType, UType},
    memory::Memory,
    state::State,
};
pub struct Processor<'a> {
    state: &'a mut State<'a>,
    instruction_memory: &'a InstructionMemory,
    memory: &'a mut Memory,
    increment_pc: bool,
}

impl<'a> Processor<'a> {
    pub fn new_processor(
        state: &'a mut State<'a>,
        instruction_memory: &'a InstructionMemory,
        memory: &'a mut Memory,
    ) -> Processor<'a> {
        Processor {
            state,
            instruction_memory,
            memory,
            increment_pc: false,
        }
    }

    pub fn set_state(&mut self, new_state: &'a mut State<'a>) {
        self.state = new_state;
    }

    pub fn get_state(&mut self) -> &mut State<'a> {
        return self.state;
    }

    pub fn display_state(&mut self) {
        // display pc and registers for basic debugging
        println!("Current PC = {:#04x}", self.state.get_pc());
        for i in 0..self.state.get_regfile().get_num_registers() {
            println!(
                "r{i} = {value}",
                value = self.state.get_regfile().read(i) as i64
            );
        }
    }

    pub fn step(&mut self) {
        // TODO - read instruction, identify its type, execute the instruction, and increment the PC
        let instruction_word: u32 = self
            .instruction_memory
            .read(self.get_state().get_pc() as usize);
        let opcode: u8 = (instruction_word & 0x7f) as u8;

        self.increment_pc = true;
        if instruction_word & 0xfe00707f == 0x33 {
            let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
            self.add(instruction);
            return;
        }

        if instruction_word & 0x707f == 0x13 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.addi(instruction);
            return;
        }

        if instruction_word & 0x707f == 0x1b {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.addiw(instruction);
            return;
        }

        if instruction_word & 0xfe00707f == 0x3b {
            let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
            self.addw(instruction);
            return;
        }

        if instruction_word & 0xfe00707f == 0x7033 {
            let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
            self.and(instruction);
            return;
        }

        if instruction_word & 0x707f == 0x7013 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.andi(instruction);
            return;
        }

        if instruction_word & 0x7f == 0x17 {
            let instruction: UType = UType::from_bytes(instruction_word.to_le_bytes());
            self.auipc(instruction);
            return;
        }

        if instruction_word & 0x707f == 0x63 {
            let instruction: BType = BType::from_bytes(instruction_word.to_le_bytes());
            self.beq(instruction);
            return;
        }

        if instruction_word & 0x707f == 0x5063 {
            let instruction: BType = BType::from_bytes(instruction_word.to_le_bytes());
            self.bge(instruction);
            return;
        }

        if instruction_word & 0x707f == 0x7063 {
            let instruction: BType = BType::from_bytes(instruction_word.to_le_bytes());
            self.bgeu(instruction);
            return;
        }

        if instruction_word & 0x707f == 0x4063 {
            let instruction: BType = BType::from_bytes(instruction_word.to_le_bytes());
            self.blt(instruction);
            return;
        }

        if instruction_word & 0x707f == 0x6063 {
            let instruction: BType = BType::from_bytes(instruction_word.to_le_bytes());
            self.bltu(instruction);
            return;
        }
        if instruction_word & 0x707f == 0x1063 {
            let instruction: BType = BType::from_bytes(instruction_word.to_le_bytes());
            self.bne(instruction);
            return;
        }

        if instruction_word & 0x7f == 0x6f {
            let instruction: JType = JType::from_bytes(instruction_word.to_le_bytes());
            self.jal(instruction);
            return;
        }

        if instruction_word & 0x707f == 0x67 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.jalr(instruction);
            return;
        }

        if instruction_word & 0x707f == 0x3 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.lb(instruction);
            return;
        }

        if instruction_word & 0x707f == 0x4003 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.lbu(instruction);
            return;
        }

        if instruction_word & 0x707f == 0x3003 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.ld(instruction);
            return;
        }

        if instruction_word & 0x707f == 0x1003 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.lh(instruction);
            return;
        }

        if instruction_word & 0x707f == 0x5003 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.lhu(instruction);
            return;
        }

        if instruction_word & 0x7f == 0x37 {
            let instruction: UType = UType::from_bytes(instruction_word.to_le_bytes());
            self.lui(instruction);
            return;
        }

        if instruction_word & 0x707f == 0x2003 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.lw(instruction);
            return;
        }

        if instruction_word & 0x707f == 0x6003 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.lwu(instruction);
            return;
        }
        
        if instruction_word & 0xfe00707f == 0x6033 {
            let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
            self.or(instruction);
            return;
        }

        if instruction_word & 0x707f == 0x6013 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.ori(instruction);
            return;
        }

        if instruction_word & 0x707f == 0x23 {
            let instruction: SType = SType::from_bytes(instruction_word.to_le_bytes());
            self.sb(instruction);
            return;
        }

        if instruction_word & 0x707f == 0x3023 {
            let instruction: SType = SType::from_bytes(instruction_word.to_le_bytes());
            self.sd(instruction);
            return;
        }

        if instruction_word & 0x707f == 0x1023 {
            let instruction: SType = SType::from_bytes(instruction_word.to_le_bytes());
            self.sh(instruction);
            return;
        }

        if instruction_word & 0xfe00707f == 0x1033 {
            let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
            self.sll(instruction);
            return;
        }

        if instruction_word & 0xfc00707f == 0x1013 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.slli(instruction);
            return;
        }

        if instruction_word & 0xfe00707f == 0x101b {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.slliw(instruction);
            return;
        }

        if instruction_word & 0xfe00707f == 0x103b {
            let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
            self.sllw(instruction);
            return;
        }

        if instruction_word & 0xfe00707f == 0x2033 {
            let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
            self.slt(instruction);
            return;
        }

        if instruction_word & 0x707f == 0x2013 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.slti(instruction);
            return;
        }

        if instruction_word & 0x707f == 0x3013 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.sltiu(instruction);
            return;
        }

        if instruction_word & 0xfe00707f == 0x3033 {
            let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
            self.sltu(instruction);
            return;
        }

        if instruction_word & 0xfe00707f == 0x40005033 {
            let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
            self.sra(instruction);
            return;
        }

        if instruction_word & 0xfc00707f == 0x40005013 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.srai(instruction);
            return;
        }

        if instruction_word & 0xfe00707f == 0x4000501b {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.sraiw(instruction);
            return;
        }

        if instruction_word & 0xfe00707f == 0x4000503b {
            let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
            self.sraw(instruction);
            return;
        }

        if instruction_word & 0xfe00707f == 0x5033 {
            let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
            self.srl(instruction);
            return;
        }

        if instruction_word & 0xfc00707f == 0x5013 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.srli(instruction);
            return;
        }

        if instruction_word & 0xfe00707f == 0x501b {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.srliw(instruction);
            return;
        }

        if instruction_word & 0xfe00707f == 0x503b {
            let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
            self.srlw(instruction);
            return;
        }

        if instruction_word & 0xfe00707f == 0x40000033 {
            let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
            self.sub(instruction);
            return;
        }

        if instruction_word & 0xfe00707f == 0x4000003b {
            let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
            self.subw(instruction);
            return;
        }

        if instruction_word & 0x707f == 0x2023 {
            let instruction: SType = SType::from_bytes(instruction_word.to_le_bytes());
            self.sw(instruction);
            return;
        }

        if instruction_word & 0xfe00707f == 0x4033 {
            let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
            self.xor(instruction);
            return;
        }

        if instruction_word & 0x707f == 0x4013 {
            let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
            self.xori(instruction);
            return;
        }

        else {
            panic!("unknown instruction!")
        }

        if self.increment_pc {
            self.get_state().increment_pc();
        }
    }

    /* IMMEDIATE OPERATIONS */
    fn addi(&mut self, instruction: IType) {
        // rd = rs1 + imm
        let rs1_value: u64 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        self.get_state().get_regfile().write(
            instruction.rd() as usize,
            ((rs1_value)
                .wrapping_add(Self::sign_extend(instruction.imm() as u64, 12) as i64 as u64)),
        );
    }

    fn slli(&mut self, instruction: IType) {
        // rd = rs1 << imm[0:4]
        let result: u64 = ((self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize))
            << ((instruction.imm() as u32) & 0x3f)) as u64;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn slti(&mut self, instruction: IType) {
        // rd = (rs1 < imm) ? 1 : 0
        let rs1_value: i64 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize) as i64;
        let result: u64 = if rs1_value < Self::sign_extend(instruction.imm() as u64, 12) as i64 {
            1
        } else {
            0
        };
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result as u64);
    }

    fn sltiu(&mut self, instruction: IType) {
        // rd = (rs1 < imm) ? 1 : 0
        let rs1_value: u64 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let result: u64 = if rs1_value < (Self::sign_extend(instruction.imm() as u64, 12) as u64) {
            1
        } else {
            0
        };
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn xori(&mut self, instruction: IType) {
        let rs1_value: u64 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        self.get_state().get_regfile().write(
            instruction.rd() as usize,
            (rs1_value ^ (Self::sign_extend(instruction.imm() as u64, 12) as u64)),
        );
    }

    fn srli(&mut self, instruction: IType) {
        // rd = rs1_value >> imm[0:4]
        let rs1_value: u64 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let result: u64 = (rs1_value) >> ((instruction.imm() as u32) & 0x3f) as u64;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn srai(&mut self, instruction: IType) {
        let rs1_value: u64 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let mut result: u64 = ((rs1_value as i64) >> ((instruction.imm() as u32) & 0x3f)) as u64;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn ori(&mut self, instruction: IType) {
        let rs1_value: u64 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        self.get_state().get_regfile().write(
            instruction.rd() as usize,
            (rs1_value | (Self::sign_extend(instruction.imm() as u64, 12) as u64)),
        );
    }

    fn andi(&mut self, instruction: IType) {
        let rs1_value: u64 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        self.get_state().get_regfile().write(
            instruction.rd() as usize,
            (rs1_value & (Self::sign_extend(instruction.imm() as u64, 12) as u64)),
        );
    }

    fn addiw(&mut self, instruction: IType) {
        let rs1_value: u64 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let imm12_sign_extended = Self::sign_extend(instruction.imm() as u64, 12);
        let result = (rs1_value as u32).wrapping_add(imm12_sign_extended as u32) as u64;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn slliw(&mut self, instruction: IType) {
        // rd = rs1 << imm[0:4]
        let rs1_value = (self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize)) as u32;
        let result: u64 = Self::sign_extend(
            (rs1_value << ((instruction.imm() as u32) & 0x32)) as u64, 32) as u64;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn srliw(&mut self, instruction: IType) {
        // rd = rs1_value >> imm[0:4]
        let rs1_value = (self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize)) as u32;
        let result: u64 = Self::sign_extend(
            (rs1_value >> ((instruction.imm() as u32) & 0x32)) as u64, 32) as u64;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn sraiw(&mut self, instruction: IType) {
        let rs1_value: u64 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let mut result: u64 = ((rs1_value as i32) >> (instruction.imm() as u32) as i64) as u64;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    /* LOAD OPERATIONS */
    fn lb(&mut self, instruction: IType) {
        let address: usize = (instruction.rs1() as i64)
            .wrapping_add(Self::sign_extend(instruction.imm() as u64, 12) as i64)
            as usize;
        let value = Self::sign_extend(self.memory.read(address) as u64, 8) as u64;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, value);
    }

    fn lh(&mut self, instruction: IType) {
        let address: usize = (instruction.rs1() as i64)
            .wrapping_add(Self::sign_extend(instruction.imm() as u64, 12) as i64)
            as usize;
        let value = Self::sign_extend(
            ((self.memory.read(address + 1) << 8) | self.memory.read(address)) as u64,
            16,
        ) as u64;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, value);
    }

    fn lw(&mut self, instruction: IType) {
        let address: usize = (instruction.rs1() as i64)
            .wrapping_add(Self::sign_extend(instruction.imm() as u64, 12) as i64)
            as usize;
        let value = Self::sign_extend(
            ((self.memory.read(address + 3) << 24) as u32
                | (self.memory.read(address + 2) << 16) as u32
                | (self.memory.read(address + 1) << 8) as u32
                | self.memory.read(address) as u32) as u64,
            32,
        ) as u64;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, value);
    }

    fn lwu(&mut self, instruction: IType) {
        let address: usize = (instruction.rs1() as i64)
            .wrapping_add(Self::sign_extend(instruction.imm() as u64, 12) as i64)
            as usize;
        let value = ((self.memory.read(address + 3) << 24) as u32
            | (self.memory.read(address + 2) << 16) as u32
            | (self.memory.read(address + 1) << 8) as u32
            | self.memory.read(address) as u32) as u64;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, value);
    }

    fn lbu(&mut self, instruction: IType) {
        let address: usize = (instruction.rs1() as i64)
            .wrapping_add(Self::sign_extend(instruction.imm() as u64, 12) as i64)
            as usize;
        let value = self.memory.read(address) as u64;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, value);
    }

    fn lhu(&mut self, instruction: IType) {
        let address: usize = (instruction.rs1() as i64)
            .wrapping_add(Self::sign_extend(instruction.imm() as u64, 12) as i64)
            as usize;
        let value =
            ((self.memory.read(address.wrapping_add(1)) << 8) | self.memory.read(address)) as u64;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, value);
    }

    fn ld(&mut self, instruction: IType) {
        let address: usize = (instruction.rs1() as i64)
            .wrapping_add(Self::sign_extend(instruction.imm() as u64, 12) as i64)
            as usize;
        let value = ((self.memory.read(address + 7) << 56) as u64
            | (self.memory.read(address + 6) << 48) as u64
            | (self.memory.read(address + 5) << 40) as u64
            | (self.memory.read(address + 4) << 32) as u64
            | (self.memory.read(address + 3) << 24) as u64
            | (self.memory.read(address + 2) << 16) as u64
            | (self.memory.read(address + 1) << 8) as u64
            | self.memory.read(address) as u64) as u64;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, value);
    }

    /* REGISTER TYPE OPERATIONS */
    fn add(&mut self, instruction: RType) {
        let rs1_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        let result = rs1_value.wrapping_add(rs2_value);
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn sub(&mut self, instruction: RType) {
        let rs1_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        let result = rs1_value.wrapping_sub(rs2_value);
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn xor(&mut self, instruction: RType) {
        let rs1_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        let result = rs1_value ^ rs2_value;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn or(&mut self, instruction: RType) {
        let rs1_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        let result = rs1_value | rs2_value;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn and(&mut self, instruction: RType) {
        let rs1_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        let result = rs1_value & rs2_value;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn sll(&mut self, instruction: RType) {
        let rs1_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        // println!("SHIFT AMOUNT {}", rs2_value);
        let result = rs1_value.wrapping_shl((rs2_value % 32) as u32);
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn srl(&mut self, instruction: RType) {
        let rs1_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        let result = rs1_value.wrapping_shr((rs2_value % 32) as u32);
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn sra(&mut self, instruction: RType) {
        let rs1_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        let result = (rs1_value as i64).wrapping_shr((rs2_value % 32) as u32) as u64;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn slt(&mut self, instruction: RType) {
        let rs1_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize) as i64;
        let rs2_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize) as i64;
        let result: u64 = if rs1_value < rs2_value { 1 } else { 0 };
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn sltu(&mut self, instruction: RType) {
        let rs1_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        let result: u64 = if rs1_value < rs2_value { 1 } else { 0 };
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn addw(&mut self, instruction: RType) {
        let rs1_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        let result = (rs1_value as u32).wrapping_add(rs2_value as u32) as i64 as u64;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn subw(&mut self, instruction: RType) {
        let rs1_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        let result = Self::sign_extend((rs1_value as u32).wrapping_sub(rs2_value as u32) as u64, 32) as u64;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn srlw(&mut self, instruction: RType) {
        let rs1_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        let result = Self::sign_extend(rs1_value.wrapping_shr((rs2_value & 0x31) as u32) as u64, 32) as u64;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn sraw(&mut self, instruction: RType) {
        let rs1_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        let result = Self::sign_extend((rs1_value as i32).wrapping_shr((rs2_value & 0x31) as u32) as u64, 32) as u64;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn sllw(&mut self, instruction: RType) {
        let rs1_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        // println!("SHIFT AMOUNT {}", rs2_value);
        let result = Self::sign_extend((rs1_value as u32).wrapping_shl((rs2_value & 0x31) as u32) as u64, 32) as u64;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    /* STORE OPERATIONS */
    fn sb(&mut self, instruction: SType) {
        let rs2_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        let address: usize = (instruction.rs1() as u64).wrapping_add(Self::sign_extend(
            ((instruction.imm_upper() as u32) << 5 | instruction.imm_lower() as u32) as u64,
            12,
        ) as u64) as usize;
        self.memory.write(address, rs2_value as u8);
    }

    fn sh(&mut self, instruction: SType) {
        let rs2_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        let address: usize = (instruction.rs1() as u64).wrapping_add(Self::sign_extend(
            ((instruction.imm_upper() as u32) << 5 | instruction.imm_lower() as u32) as u64,
            12,
        ) as u64) as usize;
        self.memory
            .write(address + 1, (rs2_value >> 8 as u64) as u8);
        self.memory.write(address, rs2_value as u8);
    }

    fn sw(&mut self, instruction: SType) {
        let rs2_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        let address: usize = (instruction.rs1() as u64)
            .wrapping_add(((instruction.imm_upper() << 5) | instruction.imm_lower()) as u64)
            as usize;
        for i in 0..4 {
            self.memory
                .write(address + i, (rs2_value >> (8 * i as u8)) as u8)
        }
    }

    fn sd(&mut self, instruction: SType) {
        let rs2_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        let address: usize = (instruction.rs1() as u64)
            .wrapping_add(((instruction.imm_upper() << 5) | instruction.imm_lower()) as u64)
            as usize;

        for i in 0..8 {
            self.memory
                .write(address + i, (rs2_value >> (8 * i as u8)) as u8)
        }
    }

    /* BRANCH OPERATIONS */
    fn beq(&mut self, instruction: BType) {
        let rs1_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        if (rs1_value as i64) == (rs2_value as i64) {
            let imm: i32 = Self::sign_extend(
                ((((instruction.imm_upper() as u32) & 0x7f) << 5)
                    | ((instruction.imm_lower() as u32) & 0x1 << 10)
                    | (((instruction.imm_upper() as u32) & 0x3f) << 5)
                    | (instruction.imm_lower() as u32) & 0x1e) as u64,
                12,
            ) as i32;

            let result = (self.get_state().get_pc()) as i64 + (imm as i64);
            self.get_state().set_pc(result as u64);
        }
    }

    fn bne(&mut self, instruction: BType) {
        let rs1_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        if (rs1_value as i64) != (rs2_value as i64) {
            let imm: i32 = Self::sign_extend(
                ((((instruction.imm_upper() as u32) & 0x7f) << 5)
                    | ((instruction.imm_lower() as u32) & 0x1 << 10)
                    | (((instruction.imm_upper() as u32) & 0x3f) << 5)
                    | (instruction.imm_lower() as u32) & 0x1e) as u64,
                12,
            ) as i32;

            let result = (self.get_state().get_pc()) as i64 + (imm as i64);
            self.get_state().set_pc(result as u64);
            self.increment_pc = false;
        }
    }

    fn blt(&mut self, instruction: BType) {
        let rs1_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        if (rs1_value as i64) < (rs2_value as i64) {
            let imm: i32 = Self::sign_extend(
                ((((instruction.imm_upper() as u32) & 0x7f) << 5)
                    | ((instruction.imm_lower() as u32) & 0x1 << 10)
                    | (((instruction.imm_upper() as u32) & 0x3f) << 5)
                    | (instruction.imm_lower() as u32) & 0x1e) as u64,
                12,
            ) as i32;

            let result = (self.get_state().get_pc()) as i64 + (imm as i64);
            self.get_state().set_pc((result as u64));
            self.increment_pc = false;
        }
    }

    fn bge(&mut self, instruction: BType) {
        let rs1_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        if (rs1_value as i64) >= (rs2_value as i64) {
            let imm: i32 = Self::sign_extend(
                ((((instruction.imm_upper() as u32) & 0x7f) << 5)
                    | ((instruction.imm_lower() as u32) & 0x1 << 10)
                    | (((instruction.imm_upper() as u32) & 0x3f) << 5)
                    | (instruction.imm_lower() as u32) & 0x1e) as u64,
                12,
            ) as i32;

            let result = (self.get_state().get_pc()) as i64 + (imm as i64);
            self.get_state().set_pc((result as u64));
            self.increment_pc = false;
        }
    }

    fn bltu(&mut self, instruction: BType) {
        let rs1_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        if rs1_value >= rs2_value {
            let imm: i32 = Self::sign_extend(
                ((((instruction.imm_upper() as u32) & 0x7f) << 5)
                    | ((instruction.imm_lower() as u32) & 0x1 << 10)
                    | (((instruction.imm_upper() as u32) & 0x3f) << 5)
                    | (instruction.imm_lower() as u32) & 0x1e) as u64,
                12,
            ) as i32;
            let result = (self.get_state().get_pc()) as i64 + (imm as i64);
            self.get_state().set_pc((result as u64));
            self.increment_pc = false;
        }
    }

    fn bgeu(&mut self, instruction: BType) {
        let rs1_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2_value = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        if rs1_value >= rs2_value {
            let imm: i32 = Self::sign_extend(
                ((((instruction.imm_upper() as u32) & 0x7f) << 5)
                    | ((instruction.imm_lower() as u32) & 0x1 << 10)
                    | (((instruction.imm_upper() as u32) & 0x3f) << 5)
                    | (instruction.imm_lower() as u32) & 0x1e) as u64,
                12,
            ) as i32;

            let result = (self.get_state().get_pc()) as i64 + (imm as i64);
            self.get_state().set_pc((result as u64));
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

        let stored_pc: u64 = self.get_state().get_pc() + 4;
        let new_pc: u64 = self.get_state().get_pc() + imm as u64;

        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, stored_pc);

        self.get_state().set_pc(new_pc);
        self.increment_pc = false;
    }

    fn jalr(&mut self, instruction: IType) {
        let imm = Self::sign_extend(instruction.imm() as u64, 12);

        let stored_pc: u64 = self.get_state().get_pc() + 4;
        let new_pc: u64 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize)
            + imm as u64;

        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, stored_pc);

        self.get_state().set_pc(new_pc);
        self.increment_pc = false;
    }

    fn lui(&mut self, instruction: UType) {
        let imm: u32 = instruction.imm() as u32;
        let result = Self::sign_extend((imm << 12) as u64, 32);
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result as u64);
    }

    fn auipc(&mut self, instruction: UType) {
        let imm: u64 = instruction.imm() as u64;
        let result = self.get_state().get_pc() + Self::sign_extend(imm << 12, 32) as u64;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    pub fn sign_extend(value: u64, length: u8) -> i64 {
        let sign_bit = 1u64 << (length - 1);
        if value & sign_bit != 0 {
            (value as i64) | !((1 << length) - 1) as i64
        } else {
            value as i64
        }
    }
}
