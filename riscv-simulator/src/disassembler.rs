use crate::Rom;
use crate::instructions::{IType, RType, SType};
pub struct Disassembler<'a> {
    rom: &'a Rom
}

impl<'a> Disassembler<'a> {
    pub fn new_disassembler(rom: &'a Rom) -> Self {
        return Disassembler {rom: rom}
    }

    fn disassemble_i_type_arithmetic(instruction_word: u32) -> String {
        // check funct3 field
        let mut result = String::from("");
        let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
        match instruction.funct3() {
            0x0 => result.push_str("addi"),
            0x1 => result.push_str("slli"),
            0x2 => result.push_str("slti"),
            0x3 => result.push_str("sltiu"),
            0x4 => result.push_str("xori"),
            0x5 => {
                match (instruction.imm() >> 5) & 0x20 {
                    0x00 => result.push_str("srli"),
                    0x20 => result.push_str("srai"),
                    _ => result.push_str("ILLEGAL INSTRUCTION")
                }
            }
            0x6 => result.push_str("ori"),
            0x7 => result.push_str("andi"),
            _ => result.push_str("ILLEGAL INSTRUCTION")
        }

        
        let operands = format!(" x{rd}, x{rs}, {imm}", rd = (instruction.rd()) as u64, rs = (instruction.rs1()) as u64, imm = instruction.imm() as i32);
        result.push_str(&operands);
        
        return result;
    }

    fn disassemble_load(instruction_word: u32) -> String {
        // check funct3 field
        let mut result = String::from("");
        let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
        match instruction.funct3() {
            0x0 => result.push_str("lb"),
            0x1 => result.push_str("lh"),
            0x2 => result.push_str("lw"),
            0x4 => result.push_str("lbu"),
            0x5 => result.push_str("lhu"),
            _ => result.push_str("ILLEGAL INSTRUCTION")
        }

        
        let operands = format!(" x{rd}, {imm}(x{rs}),", rd = (instruction.rd()) as u64, rs = (instruction.rs1()) as u64, imm = instruction.imm() as i32);
        result.push_str(&operands);
        
        return result;
    }

    fn disassemble_r_type(instruction_word: u32) -> String {
        // check funct3/func7 fields
        let mut result = String::from("");
        let instruction: RType = RType::from_bytes(instruction_word.to_le_bytes());
        
        // TODO - disassembly based on funct3 and func7 fields - create masks
        match instruction_word & 0xfe007000 {
            0x0 => {
                result.push_str("ADD");
            },

            0x40000000 => {
                result.push_str("SUB");
            },

            0x4000 => {
                result.push_str("XOR");
            },

            0x6000 => {
                result.push_str("OR");
            },

            0x7000 => {
                result.push_str("AND");
            },

            0x1000 => {
                result.push_str("SLL");
            },

            0x5000 => {
                result.push_str("SRL");
            },

            0x4005000 => {
                result.push_str("SRA");
            },

            0x4000000 => {
                result.push_str("SLT");
            },

            0x6000000 => {
                result.push_str("SLTU");
            },
            _ => return String::from("ILLEGAL INSTRUCTION")
        }
        
        let operands = format!(" x{rd}, x{rs1}, x{rs2}", rd = (instruction.rd()) as u64, rs1 = (instruction.rs1()) as u64, rs2 = (instruction.rs2()) as u64);
        result.push_str(&operands);
        
        return result;
    }

    fn disassemble_s_type(instruction_word: u32) -> String {
        let mut result = String::from("");
        let instruction: SType = SType::from_bytes(instruction_word.to_le_bytes());

        match instruction.funct3() {
            0x0 => result.push_str("sb"),
            0x1 => result.push_str("sh"),
            0x2 => result.push_str("sw"),
            _ => return String::from("ILLEGAL INSTRUCTION") 
        }

        let operands = format!(" x{rs2}, {imm}(x{rs}),", rs2 = (instruction.rs2() as u64), rs = (instruction.rs1() as u64), imm = (instruction.imm_upper() << 5 | instruction.imm_lower()) as i32);
        result.push_str(&operands);
        return result
    }

    pub fn get_trace(&self, pc: usize) -> String {
        // decode instruction based on opcode - currently implementing RV32E instruction set
        let instruction: u32 = self.rom.get_instruction(pc);
        match instruction & 0x7F {
            // i-type arithmetic
            0x13 => {
                return Self::disassemble_i_type_arithmetic(instruction);
            },
            
            // loads
            0x3 => {
                return Self::disassemble_load(instruction)
            },
            
            // r-type arithmetic
            0x33 => {
                return Self::disassemble_r_type(instruction)
            },

            // stores
            0x23 => {
                return Self::disassemble_s_type(instruction);
            }
            _ => return String::from("NOT YET IMPLEMENTED / ILLEGAL INSTRUCTIONS")
        }
    }

    
}