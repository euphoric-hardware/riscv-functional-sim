use crate::instruction_memory::{self, InstructionMemory};
use crate::instructions::{BType, IType, JType, RType, SType, UType};
pub struct Disassembler<'a> {
    instruction_memory: &'a InstructionMemory,
}

impl<'a> Disassembler<'a> {
    pub fn new_disassembler(instruction_memory: &'a InstructionMemory) -> Self {
        return Disassembler { instruction_memory: instruction_memory };
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
            0x5 => match (instruction.imm() >> 5) & 0x20 {
                0x00 => result.push_str("srli"),
                0x20 => result.push_str("srai"),
                _ => result.push_str("ILLEGAL INSTRUCTION"),
            },
            0x6 => result.push_str("ori"),
            0x7 => result.push_str("andi"),
            _ => result.push_str("ILLEGAL INSTRUCTION"),
        }

        let operands = format!(
            " x{rd}, x{rs}, {imm}",
            rd = (instruction.rd()) as u64,
            rs = (instruction.rs1()) as u64,
            imm = Self::sign_extend(instruction.imm() as u32)
        );
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
            _ => result.push_str("ILLEGAL INSTRUCTION"),
        }

        let operands = format!(
            " x{rd}, {imm}(x{rs}),",
            rd = (instruction.rd()) as u64,
            rs = (instruction.rs1()) as u64,
            imm = Self::sign_extend(instruction.imm() as u32)
        );
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
                result.push_str("add");
            }

            0x40000000 => {
                result.push_str("sub");
            }

            0x4000 => {
                result.push_str("xor");
            }

            0x6000 => {
                result.push_str("or");
            }

            0x7000 => {
                result.push_str("and");
            }

            0x1000 => {
                result.push_str("sll");
            }

            0x5000 => {
                result.push_str("srl");
            }

            0x4005000 => {
                result.push_str("sra");
            }

            0x4000000 => {
                result.push_str("slt");
            }

            0x6000000 => {
                result.push_str("sltu");
            }
            _ => return String::from("ILLEGAL INSTRUCTION"),
        }

        let operands = format!(
            " x{rd}, x{rs1}, x{rs2}",
            rd = (instruction.rd()) as u64,
            rs1 = (instruction.rs1()) as u64,
            rs2 = (instruction.rs2()) as u64
        );
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
            _ => return String::from("ILLEGAL INSTRUCTION"),
        }

        let operands = format!(
            " x{rs2}, {imm}(x{rs1}),",
            rs2 = (instruction.rs2() as u64),
            rs1 = (instruction.rs1() as u64),
            imm = Self::sign_extend((instruction.imm_upper() << 5 | instruction.imm_lower()) as u32)
        );
        result.push_str(&operands);
        return result;
    }

    fn disassemble_b_type(instruction_word: u32) -> String {
        let mut result = String::from("");
        let instruction: BType = BType::from_bytes(instruction_word.to_le_bytes());

        match instruction.funct3() {
            0x0 => result.push_str("beq"),
            0x1 => result.push_str("bne"),
            0x4 => result.push_str("blt"),
            0x5 => result.push_str("bge"),
            0x6 => result.push_str("bltu"),
            0x7 => result.push_str("bgeu"),
            _ => return String::from("ILLEGAL INSTRUCTION"),
        }

        let imm: i32 = Self::sign_extend((((instruction.imm_upper() as u32) & 0x7f) << 5)
            | ((instruction.imm_lower() as u32) & 0x1 << 10)
            | (((instruction.imm_upper() as u32) & 0x3f) << 5)
            | (instruction.imm_lower() as u32) & 0x1e);
        let operands = format!(
            " x{rs1}, x{rs2}, {imm}",
            rs1 = (instruction.rs1() as u64),
            rs2 = (instruction.rs2() as u64),
            imm = imm
        );
        result.push_str(&operands);
        return result;
    }

    fn disassemble_jal(instruction_word: u32) -> String {
        let mut result = String::from("");
        let instruction: JType = JType::from_bytes(instruction_word.to_le_bytes());
        result.push_str("jal");
        let imm: i32 = Self::sign_extend(((instruction.imm_20() as u32) << 20)
            | ((instruction.imm_12_19() as u32) << 12)
            | ((instruction.imm_11() as u32) << 11)
            | ((instruction.imm_1_10() as u32) << 1) as u32);
        let operands = format!(" {imm}", imm = imm);
        result.push_str(&operands);
        return result;
    }

    fn disassemble_jalr(instruction_word: u32) -> String {
        let mut result = String::from("jalr");
        let instruction: IType = IType::from_bytes(instruction_word.to_le_bytes());
        let operands = format!(
            " x{rd}, {imm}(x{rs}),",
            rd = (instruction.rd()) as u64,
            rs = (instruction.rs1()) as u64,
            imm = Self::sign_extend(instruction.imm() as u32)
        );
        result.push_str(&operands);
        return result;
    }

    fn disassemble_lui(instruction_word: u32) -> String {
        let mut result = String::from("lui");
        let instruction: UType = UType::from_bytes(instruction_word.to_le_bytes());
        let operands = format!(
            " x{rd}, {imm},",
            rd = (instruction.rd()) as u64,
            imm = Self::sign_extend(instruction.imm() as u32)
        );
        result.push_str(&operands);
        return result;
    }

    fn disassemble_auipc(instruction_word: u32) -> String {
        let mut result = String::from("auipc");
        let instruction: UType = UType::from_bytes(instruction_word.to_le_bytes());
        let operands = format!(
            " x{rd}, {imm},",
            rd = (instruction.rd()) as u64,
            imm = Self::sign_extend(instruction.imm() as u32)
        );
        result.push_str(&operands);
        return result;
    }

    fn sign_extend(imm: u32) -> i32 {
        let imm12 = imm & 0xFFF;

        if (imm12 & 0x800) != 0 {
            ((imm12 as i32) | (!0xFFF)) as i32
        } else {
            imm12 as i32
        }
    }

    pub fn get_trace(&self, pc: usize) -> String {
        // decode instruction based on opcode - currently implementing RV32E instruction set
        let instruction: u32 = self.instruction_memory.read(pc);
        match instruction & 0x7F {
            // i-type arithmetic
            0x13 => {
                return Self::disassemble_i_type_arithmetic(instruction);
            }

            // loads
            (0x3) => return Self::disassemble_load(instruction),

            // r-type arithmetic
            0x33 => return Self::disassemble_r_type(instruction),

            // stores
            0x23 => {
                return Self::disassemble_s_type(instruction);
            }

            // branches
            0x63 => {
                return Self::disassemble_b_type(instruction);
            }

            // jal
            0x6f => {
                return Self::disassemble_jal(instruction);
            }

            // jalr
            0x67 => {
                return Self::disassemble_jalr(instruction);
            }

            // lui
            0x37 => {
                return Self::disassemble_lui(instruction);
            }

            // auipc
            0x17 => {
                return Self::disassemble_auipc(instruction);
            }

            _ => return String::from("NOT YET IMPLEMENTED / ILLEGAL INSTRUCTIONS"),
        }
    }
}
