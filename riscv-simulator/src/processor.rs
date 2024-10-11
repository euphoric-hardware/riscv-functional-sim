use crate::{instruction_memory::InstructionMemory, instructions::{Immediate}, state::State};
pub struct Processor<'a> {
    state: &'a mut State<'a>,
    instruction_memory: &'a InstructionMemory
}

impl<'a> Processor<'a> {
    pub fn new_processor(state: &'a mut State<'a>, instruction_memory: &'a InstructionMemory) -> Processor<'a> {
        Processor {state, instruction_memory}
    }    

    pub fn set_state(&mut self, new_state: &'a mut State<'a>) {
        self.state = new_state;
    }

    pub fn get_state(&mut self) -> &mut State<'a>{
        return self.state;
    }

    pub fn display_state(&mut self) {
        // display pc and registers for basic debugging
        println!("Current PC = {:#04x}", self.state.get_pc());
        for i in (0..self.state.get_regfile().get_num_registers()) {
            println!("\tr{i} = {value}", value=self.state.get_regfile().read(i));
        }
    }

    pub fn step(&mut self) {
        // TODO - read instruction, identify its type, execute the instruction, and increment the PC
        let instruction_word: u32 = self.instruction_memory.read(self.get_state().get_pc() as usize);
        let opcode: u8 = (instruction_word & 0x7f) as u8;
        match opcode {
            0x13 => {
                // TODO - implement immediate arithmetic instructions
                let instruction: Immediate = Immediate::from_bytes(instruction_word.to_le_bytes());
                match instruction.funct3(){
                    0x0 => self.addi(instruction),
                    0x1 => self.slli(instruction),
                    0x2 => self.slti(instruction),
                    0x3 => self.sltiu(instruction),
                    0x4 => self.xori(instruction),
                    0x5 => {
                        if ((instruction.imm() >> 5) & 0x20) > 0 {
                            self.srai(instruction);
                        }

                        else {
                            self.srli(instruction);
                        }
                    } 
                    0x6 => self.ori(instruction), 
                    0x7 => self.andi(instruction),

                    _ => println!("ILLEGAL INSTRUCTION")
                }
                
            }
            _ => println!("ILLEGAL INSTRUCTION")
        }

    }

    fn addi(& mut self, instruction: Immediate) {
        // rd = rs1 + imm
        let rs1: u64 = self.get_state().get_regfile().read(instruction.rs1() as usize);
        self.get_state().get_regfile().write(
            instruction.rd() as usize, 
            ((rs1 as i64) + Self::sign_extend_immediate(instruction.imm())) as u64
        );
    }
    
    fn slli(& mut self, instruction: Immediate) {
        // rd = rs1 << imm[0:4]
        let result: u64 = ((self.get_state().get_regfile().read(instruction.rs1() as usize)) << ((instruction.imm() as u32) & 0x1f)) as u64;
        self.get_state().get_regfile().write(
            instruction.rd() as usize,
            result
        );
    }

    fn slti(& mut self, instruction: Immediate) {
        // rd = (rs1 < imm) ? 1 : 0
        let rs1: i64 = self.get_state().get_regfile().read(instruction.rs1() as usize) as i64;
        let result: u64 = if rs1 < Self::sign_extend_immediate(instruction.imm()) {1} else {0};
        self.get_state().get_regfile().write(
            instruction.rd() as usize,
        result as u64);
    }

    fn sltiu(& mut self, instruction: Immediate) {
        // rd = (rs1 < imm) ? 1 : 0
        let rs1: u64 = self.get_state().get_regfile().read(instruction.rs1() as usize);
        let result: u64 = if rs1 < (Self::sign_extend_immediate(instruction.imm()) as u64) {1} else {0};
        self.get_state().get_regfile().write(
            instruction.rd() as usize,
        result);
    }

    fn xori(& mut self, instruction: Immediate) {
        let rs1: u64 = self.get_state().get_regfile().read(instruction.rs1() as usize);
        self.get_state().get_regfile().write(
            instruction.rd() as usize, 
            (rs1 ^ instruction.imm() as u64)
        );
    }

    fn srli(& mut self, instruction: Immediate) {
        // rd = rs1 >> imm[0:4]
        let rs1: u64 = self.get_state().get_regfile().read(instruction.rs1() as usize);
        let result: u64 = (rs1) >> ((instruction.imm() as u32) & 0x1f) as u64;
        self.get_state().get_regfile().write(
            instruction.rd() as usize,
            result
        );

    }

    fn srai(& mut self, instruction: Immediate) {
        let rs1: u64 = self.get_state().get_regfile().read(instruction.rs1() as usize);
        let sign: bool = (rs1 >> 63) == 1;
        let shift: i64 = Self::sign_extend_immediate(((instruction.imm() as u32) & 0x1f) as u16);
        let mut result: u64 = rs1 << shift;
        if sign {
            // fill the leading bits with ones
            result |= (1 << (64 - shift) - 1);
        }

        self.get_state().get_regfile().write(instruction.rd() as usize, result);
        
    }

    fn ori(& mut self, instruction: Immediate) {
        let rs1: u64 = self.get_state().get_regfile().read(instruction.rs1() as usize);
        self.get_state().get_regfile().write(
            instruction.rd() as usize, 
            (rs1 | instruction.imm() as u64)
        );
    }
    
    fn andi(& mut self, instruction: Immediate) {
        let rs1: u64 = self.get_state().get_regfile().read(instruction.rs1() as usize);
        self.get_state().get_regfile().write(
            instruction.rd() as usize, 
            (rs1 & instruction.imm() as u64)
        );
    }

    fn sign_extend_immediate(imm: u16) -> i64 {
        let imm12 = imm & 0xFFF;

        if (imm12 & 0x800) != 0 {
            (imm12 as i64) | !0xFFF
        } else {
            imm12 as i64
        }
    }
    
    
}