use modular_bitfield::prelude::*;

#[bitfield]
pub struct Immediate {
    pub opcode: B7,
    pub rd: B5,
    pub funct3: B3,
    pub rs1: B5,
    pub imm: B12
}


