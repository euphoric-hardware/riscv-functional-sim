use crate::util::indent;
use serde::Deserialize;
use std::{collections::BTreeMap, env, fs, path::Path, process::Command};

#[rustfmt::skip]
static EXCLUDED_INSNS: &[&str] = &[
    "mv", "neg", "nop", "zext_b", "ret", "bleu", "bgtu", "ble", "bgez", "blez", "bgt", "bgtz",
    "bltz", "bnez", "beqz", "seqz", "snez", "sltz", "sgtz", "jr", "j", "sext_w", "csrr",
    "csrw", "csrs", "csrc", "csrwi", "csrsi", "csrci", "jal_pseudo", "jalr_pseudo", "fence_tso",
    "scall", "sbreak", "pause"
];

#[derive(Deserialize)]
pub struct Insn {
    pub mask: String,
    #[serde(rename = "match")]
    pub r_match: String,
    pub variable_fields: Vec<String>,
}

pub type InstructionMap = BTreeMap<String, Insn>;

pub fn get_instr_map(opcodes_dir: impl AsRef<Path>) -> InstructionMap {
    // Activate Python virtualenv
    let mut path = env::var("PATH").expect("reading PATH failed");
    if let Ok(venv_path) = env::var("VIRTUAL_ENV") {
        path = format!("{}/bin:{}", venv_path, path);
        println!("cargo::warning={}", path);
    }

    let cmd = Command::new("make")
        .arg("EXTENSIONS=rv_i rv64_i rv_zicsr rv_system")
        .current_dir(&opcodes_dir)
        .env("PATH", path)
        .output()
        .expect("riscv-opcodes make failed");

    if !cmd.status.success() {
        panic!(
            "riscv-opcodes make failed with output: {}",
            String::from_utf8_lossy(&cmd.stderr)
        );
    }

    let instr_dict_yaml = fs::read_to_string(opcodes_dir.as_ref().join("instr_dict.yaml"))
        .expect("riscv-opcodes instr_dict.yaml");
    let mut instr_dict: BTreeMap<String, Insn> =
        serde_yaml::from_str(&instr_dict_yaml).expect("riscv-opcodes instr_dict.yaml");

    for &insn in EXCLUDED_INSNS {
        instr_dict.remove(insn);
    }

    instr_dict
}

pub fn generate_cpu_execute(
    src_dir: impl AsRef<Path>,
    tpl_dir: impl AsRef<Path>,
    insns: &InstructionMap,
) {
    let mut cpu_execute_tpl = fs::read_to_string(tpl_dir.as_ref().join("cpu_execute.txt")).unwrap();

    let insn_cases = insns
        .iter()
        .enumerate()
        .map(|(i, (insn_name, insn))| {
            let prefix = if i == 0 { "if" } else { "else if" };
            format!(
                "{prefix} bits & {} == {} {{ insn_impl::{insn_name}::{insn_name}(insn, self, bus) }}\n",
                insn.r_match, insn.mask
            )
        })
        .collect::<String>();
    cpu_execute_tpl = cpu_execute_tpl.replace("{insn_cases}", indent(&insn_cases, 2).trim_end());

    let cpu_execute = src_dir.as_ref().join("generated/cpu_execute.rs");
    if !matches!(fs::exists(&cpu_execute), Ok(true)) {
        fs::write(&cpu_execute, cpu_execute_tpl)
            .expect("generate_cpu_execute write cpu_execute.rs");
    }
}

pub fn generate_instruction_templates(
    src_dir: impl AsRef<Path>,
    tpl_dir: impl AsRef<Path>,
    insns: &InstructionMap,
) {
    let insn_tpl = fs::read_to_string(tpl_dir.as_ref().join("insn_template.txt")).unwrap();

    for (insn_name, insn) in insns.iter() {
        let mut insn_tpl = insn_tpl.clone();
        insn_tpl = insn_tpl.replace("{insn_name}", &insn_name);

        let insn_fields = insn
            .variable_fields
            .iter()
            .map(|f| format!("let {f} = insn.{f}();\n"))
            .collect::<String>();
        insn_tpl = insn_tpl.replace("{insn_fields}", &indent(&insn_fields, 1));

        let insn_file = src_dir.as_ref().join(format!("insn_impl/{}.rs", insn_name));
        if !matches!(fs::exists(&insn_file), Ok(true)) {
            fs::write(&insn_file, insn_tpl).expect(&format!("insn_impl write {}.rs", insn_name));
        }
    }

    let mod_rs = src_dir.as_ref().join("insn_impl/mod.rs");
    let mod_decls = insns
        .keys()
        .map(|i| format!("pub mod {i};\n"))
        .collect::<String>();
    fs::write(&mod_rs, mod_decls).expect("insn_impl write mod.rs");
}

#[derive(Deserialize, Debug)]
struct Arg {
    arg: String,
    end: u8,
    start: u8,
}

fn get_args(opcodes_dir: impl AsRef<Path>) -> Vec<Arg> {
    let mut arg_lut_csv = fs::read_to_string(opcodes_dir.as_ref().join("arg_lut.csv"))
        .expect("riscv-opcodes arg_lut.csv");
    arg_lut_csv = arg_lut_csv.replace(' ', ""); // causes issues with CSV parsing

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(arg_lut_csv.as_bytes());

    reader
        .deserialize()
        .collect::<Result<Vec<Arg>, _>>()
        .expect("riscv-opcodes arg_lut.csv")
}

pub fn generate_insn_args(
    opcodes_dir: impl AsRef<Path>,
    src_dir: impl AsRef<Path>,
    tpl_dir: impl AsRef<Path>,
) {
    let args = get_args(&opcodes_dir);
    let mut args_tpl = fs::read_to_string(tpl_dir.as_ref().join("insn_args.txt")).unwrap();

    let arg_fns = args
        .iter()
        .map(|arg| {
            let offset = arg.start;
            let len = arg.end - arg.start + 1;
            format!(
                "pub fn {}(&self) -> u64 {{ self.bit_range({offset}, {len}) }}\n",
                arg.arg
            )
        })
        .collect::<String>();
    args_tpl = args_tpl.replace("{arg_fns}", indent(&arg_fns, 1).trim_end());

    let insn_args_file = src_dir.as_ref().join("generated/insn_args.rs");
    if !matches!(fs::exists(&insn_args_file), Ok(true)) {
        fs::write(&insn_args_file, args_tpl).expect("generate_insn_args write insn_args.rs");
    }
}
