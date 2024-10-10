use crate::Rom;
pub struct Disassembler<'a> {
    rom: &'a Rom
}

impl<'a> Disassembler<'a> {
    pub fn new_disassembler(rom: &'a Rom) -> Self {
        return Disassembler {rom: rom}
    }

    fn disassemble_immediate_logic_arithmetic(instruction: u32) -> String {
        // check funct3 field
        let mut result = String::from("");
        match (instruction >> 12) & 0x7 {
            0x0 => result.push_str("addi"),
            0x1 => result.push_str("slli"),
            0x2 => result.push_str("slti"),
            0x3 => result.push_str("sltiu"),
            0x4 => result.push_str("xori"),
            0x5 => {
                match (instruction >> 25) {
                    0x00 => result.push_str("srli"),
                    0x20 => result.push_str("srai"),
                    _ => result.push_str("ILLEGAL INSTRUCTION")
                }
            }
            0x6 => result.push_str("ori"),
            0x7 => result.push_str("andi"),
            _ => result.push_str("ILLEGAL INSTRUCTION")
        }

        if result.as_str() != "ILLEGAL INSTRUCTION" {
            let operands = format!(" r{rd}, r{rs}, {imm}", rd = (instruction >> 7) & 0x1f, rs = (instruction >> 15) & 0x1f, imm = (instruction >> 20));
            result.push_str(&operands);
        }
        return result;
    }

    pub fn get_trace(&self, pc: usize) -> String {
        // decode instruction based on opcode - currently implementing RV32E instruction set
        let instruction: u32 = self.rom.get_instruction(pc);
        match (instruction & 0x7F) {
            0x13 => {
                return Self::disassemble_immediate_logic_arithmetic(instruction);
            }
            0b0110011 => return String::from("R-TYPE"),
            _ => return String::from("NOT YET IMPLEMENTED / ILLEGAL INSTRUCTIONS")
        }
    }

    
}