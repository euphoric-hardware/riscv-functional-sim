use crate::cpu::{Cpu, Instruction};

pub fn fadd(instruction: Instruction, cpu: &mut Cpu) {
    let rd = instruction.rd();
    let rs1 = instruction.rs1();
    let rs2 = instruction.rs2();
    let value = cpu.fregisters[rs1 as usize] + cpu.fregisters[rs2 as usize];
    cpu.fregisters[rd as usize] = value;
}

pub fn fsub(instruction: Instruction, cpu: &mut Cpu) {
    let rd = instruction.rd();
    let rs1 = instruction.rs1();
    let rs2 = instruction.rs2();
    let value = cpu.fregisters[rs1 as usize] - cpu.fregisters[rs2 as usize];
    cpu.fregisters[rd as usize] = value;
}

pub fn fmul(instruction: Instruction, cpu: &mut Cpu) {
    let rd = instruction.rd();
    let rs1 = instruction.rs1();
    let rs2 = instruction.rs2();
    let value = cpu.fregisters[rs1 as usize] * cpu.fregisters[rs2 as usize];
    cpu.fregisters[rd as usize] = value;
}

pub fn fdiv(instruction: Instruction, cpu: &mut Cpu) {
    let rd = instruction.rd();
    let rs1 = instruction.rs1();
    let rs2 = instruction.rs2();
    let value = cpu.fregisters[rs1 as usize] / cpu.fregisters[rs2 as usize];
    cpu.fregisters[rd as usize] = value;
}

pub fn fmadd(instruction: Instruction, cpu: &mut Cpu) {
    let rd = instruction.rd();
    let rs1 = instruction.rs1();
    let rs2 = instruction.rs2();
    let rs3 = instruction.rs3();
    let value = cpu.fregisters[rs1 as usize] * cpu.fregisters[rs2 as usize] + cpu.fregisters[rs3 as usize];
    cpu.fregisters[rd as usize] = value;
}

pub fn fmsub(instruction: Instruction, cpu: &mut Cpu) {
    let rd = instruction.rd();
    let rs1 = instruction.rs1();
    let rs2 = instruction.rs2();
    let rs3 = instruction.rs3();
    let value = cpu.fregisters[rs1 as usize] * cpu.fregisters[rs2 as usize] - cpu.fregisters[rs3 as usize];
    cpu.fregisters[rd as usize] = value;
}

pub fn fnmadd(instruction: Instruction, cpu: &mut Cpu) {
    let rd = instruction.rd();
    let rs1 = instruction.rs1();
    let rs2 = instruction.rs2();
    let rs3 = instruction.rs3();
    let value = -(cpu.fregisters[rs1 as usize] * cpu.fregisters[rs2 as usize]) + cpu.fregisters[rs3 as usize];
    cpu.fregisters[rd as usize] = value;
}

pub fn fnmsub(instruction: Instruction, cpu: &mut Cpu) {
    let rd = instruction.rd();
    let rs1 = instruction.rs1();
    let rs2 = instruction.rs2();
    let rs3 = instruction.rs3();
    let value = -(cpu.fregisters[rs1 as usize] * cpu.fregisters[rs2 as usize]) - cpu.fregisters[rs3 as usize];
    cpu.fregisters[rd as usize] = value;
}

pub fn fcvt_w_s(instruction: Instruction, cpu: &mut Cpu) {
    let rd = instruction.rd();
    let rs1 = instruction.rs1();
    let value = cpu.fregisters[rs1 as usize] as i32 as f64;
    cpu.fregisters[rd as usize] = value;
}

pub fn fcvt_s_w(instruction: Instruction, cpu: &mut Cpu) {
    let rd = instruction.rd();
    let rs1 = instruction.rs1();
    let value = cpu.fregisters[rs1 as usize] as f32 as f64;
    cpu.fregisters[rd as usize] = value;
}

pub fn feq(instruction: Instruction, cpu: &mut Cpu) {
    let rd = instruction.rd();
    let rs1 = instruction.rs1();
    let rs2 = instruction.rs2();
    let value = if cpu.fregisters[rs1 as usize] == cpu.fregisters[rs2 as usize] {
        1.0
    } else {
        0.0
    };
    cpu.fregisters[rd as usize] = value;
}

pub fn flt(instruction: Instruction, cpu: &mut Cpu) {
    let rd = instruction.rd();
    let rs1 = instruction.rs1();
    let rs2 = instruction.rs2();
    let value = if cpu.fregisters[rs1 as usize] < cpu.fregisters[rs2 as usize] {
        1.0
    } else {
        0.0
    };
    cpu.fregisters[rd as usize] = value;
}

pub fn fle(instruction: Instruction, cpu: &mut Cpu) {
    let rd = instruction.rd();
    let rs1 = instruction.rs1();
    let rs2 = instruction.rs2();
    let value = if cpu.fregisters[rs1 as usize] <= cpu.fregisters[rs2 as usize] {
        1.0
    } else {
        0.0
    };
    cpu.fregisters[rd as usize] = value;
}

pub fn fsqrt(instruction: Instruction, cpu: &mut Cpu) {
    let rd = instruction.rd();
    let rs1 = instruction.rs1();
    let value = cpu.fregisters[rs1 as usize].sqrt();
    cpu.fregisters[rd as usize] = value;
}

pub fn fcvt_wu_s(instruction: Instruction, cpu: &mut Cpu) {
    let rd = instruction.rd();
    let rs1 = instruction.rs1();
    let value = cpu.fregisters[rs1 as usize] as u32 as f64;
    cpu.fregisters[rd as usize] = value;
}
