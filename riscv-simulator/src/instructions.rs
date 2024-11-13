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
pub struct JType {
    pub opcode: B7,
    pub rd: B5,
    pub imm_12_19: B8,
    pub imm_11: B1,
    pub imm_1_10: B10,
    pub imm_20: B1
}

#[bitfield]
pub struct UType {
    pub opcode: B7,
    pub rd: B5,
    pub imm: B20
}
