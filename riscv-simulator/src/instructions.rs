use modular_bitfield::prelude::*;

#[bitfield]
pub struct IType {
    pub opcode: B7,
    pub rd: B5,
    pub funct3: B3,
    pub rs1: B5,
    pub imm: B12
}

#[bitfield]
pub struct RType {
    pub opcode: B7,
    pub rd: B5,
    pub funct3: B3,
    pub rs1: B5,
    pub rs2: B5,
    pub funct7: B7
}

#[bitfield]
pub struct SType {
    pub opcode: B7,
    pub imm_lower: B5,
    pub funct3: B3,
    pub rs1: B5,
    pub rs2: B5,
    pub imm_upper: B7
}

#[bitfield]
pub struct BType {
    pub opcode: B7,
    pub imm_lower: B5,
    pub funct3: B3,
    pub rs1: B5,
    pub rs2: B5,
    pub imm_upper:B7
}

#[bitfield]
// For fused instructions
pub struct R4Type {
    pub opcode: B7,  // Opcode (7 bits)
    pub rd: B5,      // Destination register (5 bits)
    pub funct3: B3,  // Function field (3 bits)
    pub rs1: B5,     // Source register 1 (5 bits)
    pub rs2: B5,     // Source register 2 (5 bits)
    pub rs3: B5,     // Source register 3 (5 bits)
    pub funct2: B2,  // Additional function field (2 bits)
}