mod instructions;
mod util;

use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let src_dir = out_dir.join("src");
    let tpl_dir = out_dir.join("tpl");
    let opcodes_dir = out_dir.join("../isa-data/riscv-opcodes");

    let riscv_opcodes_ref = out_dir
        .join("../.git/modules/riscv-opcodes/refs/heads/master")
        .canonicalize()
        .expect("riscv-opcodes submodule");
    println!(
        "cargo::rerun-if-changed={}",
        riscv_opcodes_ref.to_string_lossy()
    );

    let insn_map = instructions::get_instr_map(&opcodes_dir);
    instructions::generate_instruction_templates(&src_dir, &tpl_dir, &insn_map);
    instructions::generate_cpu_execute(&src_dir, &tpl_dir, &insn_map);
    instructions::generate_insn_args(&opcodes_dir, &src_dir, &tpl_dir);
    // generate_csr_load_store(&out_dir, &mut rdr2);
}
