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
            println!("r{i} = {value}", value = self.state.get_regfile().read(i));
        }
    }

    pub fn step(&mut self) {
        // TODO - read instruction, identify its type, execute the instruction, and increment the PC
        let instruction_word: u32 = self
            .instruction_memory
            .read(self.get_state().get_pc() as usize);
        let opcode: u8 = (instruction_word & 0x7f) as u8;
        self.increment_pc = true;
        match opcode {
            /* IMMEDIATE ARITHMETIC */
            0x13 => {
                let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
                match instruction.funct3() {
                    0x0 => self.addi(instruction),
                    0x1 => self.slli(instruction),
                    0x2 => self.slti(instruction),
                    0x3 => self.sltiu(instruction),
                    0x4 => self.xori(instruction),
                    0x5 => {
                        if ((instruction.imm() >> 5) & 0x20) > 0 {
                            self.srai(instruction);
                        } else {
                            self.srli(instruction);
                        }
                    }
                    0x6 => self.ori(instruction),
                    0x7 => self.andi(instruction),

                    _ => println!("ILLEGAL INSTRUCTION"),
                }
            }

            /* LOADS */
            0x3 => {
                let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
                match instruction.funct3() {
                    0x0 => self.lb(instruction),
                    0x1 => self.lh(instruction),
                    0x2 => self.lw(instruction),
                    0x4 => self.lbu(instruction),
                    0x5 => self.lhu(instruction),
                    _ => println!("ILLEGAL INSTRUCTION"),
                }
            }

            /* ARITHMETIC */
            0x33 => {
                let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
                match instruction_word & 0xfe007000 {
                    0x0 => self.add(instruction),
                    0x40000000 => self.sub(instruction),
                    0x4000 => self.xor(instruction),
                    0x6000 => self.or(instruction),
                    0x7000 => self.and(instruction),
                    0x1000 => self.sll(instruction),
                    0x5000 => self.srl(instruction),
                    0x4005000 => self.sra(instruction),
                    0x4000000 => self.slt(instruction),
                    0x6000000 => self.sltu(instruction),
                    _ => println!("ILLEGAL INSTRUCTION"),
                }
            }

            /* STORES */
            0x23 => {
                let instruction: SType = SType::from_bytes(instruction_word.to_le_bytes());
                match instruction.funct3() {
                    0x0 => self.sb(instruction),
                    0x1 => self.sh(instruction),
                    0x2 => self.sw(instruction),
                    _ => println!("ILLEGAL INSTRUCTION"),
                }
            }

            /* BRANCHES */
            0x63 => {
                let instruction: BType = BType::from_bytes(instruction_word.to_le_bytes());
                match instruction.funct3() {
                    0x0 => self.beq(instruction),
                    0x1 => self.bne(instruction),
                    0x4 => self.blt(instruction),
                    0x5 => self.bge(instruction),
                    0x6 => self.bltu(instruction),
                    0x7 => self.bgeu(instruction),
                    _ => println!("ILLEGAL INSTRUCTION"),
                }
            }

            /* JUMPS */
            0x6F => {
                let instruction: JType = JType::from_bytes(instruction_word.to_le_bytes());
                self.jal(instruction);
            }

            0x67 => {
                let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
                self.jalr(instruction);
            }

            /* UPPER IMMEDIATES */
            0x37 => {
                let instruction: UType = UType::from_bytes(instruction_word.to_le_bytes());
                self.lui(instruction);
            }

            0x17 => {
                let instruction: UType = UType::from_bytes(instruction_word.to_le_bytes());
                self.auipc(instruction);
            }

            _ => println!("ILLEGAL INSTRUCTION"),
        }

        if self.increment_pc {
            self.get_state().increment_pc();
        }
    }

    /* IMMEDIATE OPERATIONS */
    fn addi(&mut self, instruction: IType) {
        // rd = rs1 + imm
        let rs1: u64 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        self.get_state().get_regfile().write(
            instruction.rd() as usize,
            (rs1 as i64 + Self::sign_extend(instruction.imm() as u32) as i64) as u64,
        );
    }

    fn slli(&mut self, instruction: IType) {
        // rd = rs1 << imm[0:4]
        let result: u64 = ((self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize))
            << ((instruction.imm() as u32) & 0x1f)) as u64;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn slti(&mut self, instruction: IType) {
        // rd = (rs1 < imm) ? 1 : 0
        let rs1: i64 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize) as i64;
        let result: u64 = if rs1 < Self::sign_extend(instruction.imm() as u32) as i64 {
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
        let rs1: u64 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let result: u64 = if rs1 < (Self::sign_extend(instruction.imm() as u32) as u64) {
            1
        } else {
            0
        };
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn xori(&mut self, instruction: IType) {
        let rs1: u64 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        self.get_state().get_regfile().write(
            instruction.rd() as usize,
            (rs1 ^ (Self::sign_extend(instruction.imm() as u32) as u64)),
        );
    }

    fn srli(&mut self, instruction: IType) {
        // rd = rs1 >> imm[0:4]
        let rs1: u64 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let result: u64 = (rs1) >> ((instruction.imm() as u32) & 0x1f) as u64;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn srai(&mut self, instruction: IType) {
        let rs1: u64 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let sign: bool = (rs1 >> 63) == 1;
        let shift: i32 = Self::sign_extend(((instruction.imm() as u32) & 0x1f));
        let mut result: u64 = rs1 << shift;
        if sign {
            // fill the leading bits with ones
            result |= 1 << (64 - shift) - 1;
        }

        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn ori(&mut self, instruction: IType) {
        let rs1: u64 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        self.get_state().get_regfile().write(
            instruction.rd() as usize,
            (rs1 | (Self::sign_extend(instruction.imm() as u32) as u64)),
        );
    }

    fn andi(&mut self, instruction: IType) {
        let rs1: u64 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        self.get_state().get_regfile().write(
            instruction.rd() as usize,
            (rs1 & (Self::sign_extend(instruction.imm() as u32) as u64)),
        );
    }

    /* LOAD OPERATIONS */
    fn lb(&mut self, instruction: IType) {
        let address: usize =
            (instruction.rs1() as u64).wrapping_add(instruction.imm() as u64) as usize;
        let value = (self.memory.read(address) as i64) as u64;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, value);
    }

    fn lh(&mut self, instruction: IType) {
        let address: usize =
            (instruction.rs1() as u64).wrapping_add(instruction.imm() as u64) as usize;
        let value =
            (((self.memory.read(address + 1) << 8) | self.memory.read(address)) as i64) as u64;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, value);
    }

    fn lw(&mut self, instruction: IType) {
        let address: usize =
            (instruction.rs1() as u64).wrapping_add(instruction.imm() as u64) as usize;
        let value = (((self.memory.read(address + 3) << 24)
            | (self.memory.read(address + 2) << 16)
            | (self.memory.read(address + 1) << 8)
            | self.memory.read(address)) as i64) as u64;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, value);
    }

    fn lbu(&mut self, instruction: IType) {
        let address: usize =
            (instruction.rs1() as u64).wrapping_add(instruction.imm() as u64) as usize;
        let value = self.memory.read(address) as u64;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, value);
    }

    fn lhu(&mut self, instruction: IType) {
        let address: usize =
            (instruction.rs1() as u64).wrapping_add(instruction.imm() as u64) as usize;
        let value =
            ((self.memory.read(address.wrapping_add(1)) << 8) | self.memory.read(address)) as u64;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, value);
    }

    /* REGISTER TYPE OPERATIONS */
    fn add(&mut self, instruction: RType) {
        let rs1 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        let result = rs1.wrapping_add(rs2);
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn sub(&mut self, instruction: RType) {
        let rs1 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        let result = rs1 - (rs2);
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn xor(&mut self, instruction: RType) {
        let rs1 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        let result = rs1 ^ rs2;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn or(&mut self, instruction: RType) {
        let rs1 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        let result = rs1 | rs2;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn and(&mut self, instruction: RType) {
        let rs1 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        let result = rs1 & rs2;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn sll(&mut self, instruction: RType) {
        let rs1 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        let result = rs1 << rs2;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn srl(&mut self, instruction: RType) {
        let rs1 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        let result = rs1 >> rs2;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn sra(&mut self, instruction: RType) {
        let rs1 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        let result = ((rs1 as i64) >> rs2) as u64;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn slt(&mut self, instruction: RType) {
        let rs1 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize) as i64;
        let rs2 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize) as i64;
        let result: u64 = if rs1 < rs2 { 1 } else { 0 };
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn sltu(&mut self, instruction: RType) {
        let rs1 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        let result: u64 = if rs1 < rs2 { 1 } else { 0 };
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    /* STORE OPERATIONS */
    fn sb(&mut self, instruction: SType) {
        let address: usize = (instruction.rs1() as u64)
            .wrapping_add((instruction.imm_upper() << 5 | instruction.imm_lower()) as u64)
            as usize;
        self.memory.write(address, (instruction.rs2() & 0xff) as u8);
    }

    fn sh(&mut self, instruction: SType) {
        let address: usize = (instruction.rs1() as u64)
            .wrapping_add((instruction.imm_upper() << 5 | instruction.imm_lower()) as u64)
            as usize;
        self.memory.write(address, (instruction.rs2() & 0xff) as u8);
        self.memory.write(
            address + 1,
            (((instruction.rs2() as u64) >> 8) & 0xff) as u8,
        );
    }

    fn sw(&mut self, instruction: SType) {
        let address: usize = (instruction.rs1() as u64)
            .wrapping_add((instruction.imm_upper() << 5 | instruction.imm_lower()) as u64)
            as usize;
        self.memory.write(address, (instruction.rs2() & 0xff) as u8);
        self.memory.write(
            address + 1,
            (((instruction.rs2() as u64) >> 8) & 0xff) as u8,
        );
        self.memory.write(
            address + 2,
            (((instruction.rs2() as u64) >> 16) & 0xff) as u8,
        );
        self.memory.write(
            address + 3,
            (((instruction.rs2() as u64) >> 24) & 0xff) as u8,
        );
    }

    /* BRANCH OPERATIONS */
    fn beq(&mut self, instruction: BType) {
        let rs1 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        if (rs1 as i64) == (rs1 as i64) {
            let imm: i32 = Self::sign_extend(
                (((instruction.imm_upper() as u32) & 0x7f) << 5)
                    | ((instruction.imm_lower() as u32) & 0x1 << 10)
                    | (((instruction.imm_upper() as u32) & 0x3f) << 5)
                    | (instruction.imm_lower() as u32) & 0x1e,
            );

            let result = (self.get_state().get_pc()) as i64 + (imm as i64);
            self.get_state().set_pc(result as u64);
        }
    }

    fn bne(&mut self, instruction: BType) {
        let rs1 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        if (rs1 as i64) != (rs2 as i64) {
            let imm: i32 = Self::sign_extend(
                (((instruction.imm_upper() as u32) & 0x7f) << 5)
                    | ((instruction.imm_lower() as u32) & 0x1 << 10)
                    | (((instruction.imm_upper() as u32) & 0x3f) << 5)
                    | (instruction.imm_lower() as u32) & 0x1e,
            );

            let result = (self.get_state().get_pc()) as i64 + (imm as i64);
            self.get_state().set_pc(result as u64);
            self.increment_pc = false;
        }
    }

    fn blt(&mut self, instruction: BType) {
        let rs1 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        if (rs1 as i64) < (rs2 as i64) {
            let imm: i32 = Self::sign_extend(
                (((instruction.imm_upper() as u32) & 0x7f) << 5)
                    | ((instruction.imm_lower() as u32) & 0x1 << 10)
                    | (((instruction.imm_upper() as u32) & 0x3f) << 5)
                    | (instruction.imm_lower() as u32) & 0x1e,
            );

            let result = (self.get_state().get_pc()) as i64 + (imm as i64);
            self.get_state().set_pc((result as u64));
            self.increment_pc = false;
        }
    }

    fn bge(&mut self, instruction: BType) {
        let rs1 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        if (rs1 as i64) >= (rs2 as i64) {
            let imm: i32 = Self::sign_extend(
                (((instruction.imm_upper() as u32) & 0x7f) << 5)
                    | ((instruction.imm_lower() as u32) & 0x1 << 10)
                    | (((instruction.imm_upper() as u32) & 0x3f) << 5)
                    | (instruction.imm_lower() as u32) & 0x1e,
            );

            let result = (self.get_state().get_pc()) as i64 + (imm as i64);
            self.get_state().set_pc((result as u64));
            self.increment_pc = false;
        }
    }

    fn bltu(&mut self, instruction: BType) {
        let rs1 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        if rs1 >= rs2 {
            let imm: i32 = Self::sign_extend(
                (((instruction.imm_upper() as u32) & 0x7f) << 5)
                    | ((instruction.imm_lower() as u32) & 0x1 << 10)
                    | (((instruction.imm_upper() as u32) & 0x3f) << 5)
                    | (instruction.imm_lower() as u32) & 0x1e,
            );

            let result = (self.get_state().get_pc()) as i64 + (imm as i64);
            self.get_state().set_pc((result as u64));
            self.increment_pc = false;
        }
    }

    fn bgeu(&mut self, instruction: BType) {
        let rs1 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs1() as usize);
        let rs2 = self
            .get_state()
            .get_regfile()
            .read(instruction.rs2() as usize);
        if rs1 >= rs2 {
            let imm: i32 = Self::sign_extend(
                (((instruction.imm_upper() as u32) & 0x7f) << 5)
                    | ((instruction.imm_lower() as u32) & 0x1 << 10)
                    | (((instruction.imm_upper() as u32) & 0x3f) << 5)
                    | (instruction.imm_lower() as u32) & 0x1e,
            );

            let result = (self.get_state().get_pc()) as i64 + (imm as i64);
            self.get_state().set_pc((result as u64));
            self.increment_pc = false;
        }
    }

    /* JUMP OPERATIONS */
    fn jal(&mut self, instruction: JType) {
        let imm: i64 = Self::sign_extend(
            ((instruction.imm_20() as u32) << 20)
                | ((instruction.imm_12_19() as u32) << 12)
                | ((instruction.imm_11() as u32) << 11)
                | ((instruction.imm_1_10() as u32) << 1) as u32,
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
        let imm = Self::sign_extend(instruction.imm() as u32);

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
        let imm: u64 = instruction.imm() as u64;
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, imm << 12);
    }

    fn auipc(&mut self, instruction: UType) {
        let imm: u64 = instruction.imm() as u64;
        let result = self.get_state().get_pc() + (imm << 12);
        self.get_state()
            .get_regfile()
            .write(instruction.rd() as usize, result);
    }

    fn sign_extend(imm: u32) -> i32 {
        let imm12 = imm & 0xFFF;

        if (imm12 & 0x800) != 0 {
            (imm12 as i32) | !0xFFF
        } else {
            imm12 as i32
        }
    }
}
