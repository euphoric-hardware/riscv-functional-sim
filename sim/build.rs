use serde::Deserialize;
use std::{
    collections::BTreeMap,
    convert::identity,
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

#[rustfmt::skip]
static EXCLUDED_INSNS: &[&str] = &[
    "mv", "neg", "nop", "zext_b", "ret", "bleu", "bgtu", "ble", "bgez", "blez", "bgt", "bgtz",
    "bltz", "bnez", "beqz", "seqz", "snez", "sltz", "sgtz", "jr", "j", "sext_w", "csrr",
    "csrw", "csrs", "csrc", "csrwi", "csrsi", "csrci", "jal_pseudo", "jalr_pseudo", "fence_tso",
    "scall", "sbreak", "pause"
];

fn main() {
    let out_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let src_dir = out_dir.join("src");
    let tpl_dir = out_dir.join("tpl");
    let opcodes_dir = out_dir.join("../riscv-opcodes");

    let riscv_opcodes_ref = out_dir
        .join("../.git/modules/riscv-opcodes/refs/heads/master")
        .canonicalize()
        .expect("riscv-opcodes submodule");
    println!(
        "cargo::rerun-if-changed={}",
        riscv_opcodes_ref.to_string_lossy()
    );

    let instructions = get_instr_dict(&opcodes_dir);
    let arg_luts = get_arg_luts(&opcodes_dir);

    generate_instruction_templates(&src_dir, &tpl_dir, &instructions);
    generate_cpu_execute(&src_dir, &tpl_dir, &instructions);
    generate_insn_arg_luts(&src_dir, &tpl_dir, &arg_luts);
    // generate_csr_load_store(&out_dir, &mut rdr2);
}

fn generate_instruction_templates(
    src_dir: impl AsRef<Path>,
    tpl_dir: impl AsRef<Path>,
    insns: &BTreeMap<String, Insn>,
) {
    let insn_tpl = fs::read_to_string(tpl_dir.as_ref().join("instruction_template.txt")).unwrap();

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
        if !fs::exists(&insn_file).is_ok_and(identity) {
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

fn generate_cpu_execute(
    src_dir: impl AsRef<Path>,
    tpl_dir: impl AsRef<Path>,
    insns: &BTreeMap<String, Insn>,
) {
    let mut cpu_execute_tpl = fs::read_to_string(tpl_dir.as_ref().join("cpu_execute.txt")).unwrap();

    let insn_cases = insns
        .iter()
        .enumerate()
        .map(|(i, (insn_name, insn))| {
            let prefix = if i == 0 { "if" } else { "else if" };
            format!(
                "{prefix} bits & {} == {} {{ insn_impl::{insn_name}::{insn_name}(insn, self, bus) }}\n",
                insn.r#match, insn.mask
            )
        })
        .collect::<String>();
    cpu_execute_tpl = cpu_execute_tpl.replace("{insn_cases}", indent(&insn_cases, 2).trim_end());

    let cpu_execute = src_dir.as_ref().join("generated/cpu_execute.rs");
    if !fs::exists(&cpu_execute).is_ok_and(identity) {
        fs::write(&cpu_execute, cpu_execute_tpl)
            .expect("generate_cpu_execute write cpu_execute.rs");
    }
}

fn generate_insn_arg_luts(
    src_dir: impl AsRef<Path>,
    tpl_dir: impl AsRef<Path>,
    arg_luts: &[ArgLut],
) {
    let mut arg_luts_tpl = fs::read_to_string(tpl_dir.as_ref().join("insn_arg_luts.txt")).unwrap();

    let arg_lut_fns = arg_luts
        .iter()
        .map(|lut| {
            let offset = lut.start;
            let len = lut.end - lut.start + 1;
            format!(
                "pub fn {}(&self) -> u64 {{ self.bit_range({offset}, {len}) }}\n",
                lut.arg
            )
        })
        .collect::<String>();
    arg_luts_tpl = arg_luts_tpl.replace("{arg_lut_fns}", indent(&arg_lut_fns, 1).trim_end());

    let insn_arg_luts = src_dir.as_ref().join("generated/insn_arg_luts.rs");
    if !fs::exists(&insn_arg_luts).is_ok_and(identity) {
        fs::write(&insn_arg_luts, arg_luts_tpl)
            .expect("generate_insn_arg_luts write insn_arg_luts.rs");
    }
}

// fn generate_csr_load_store<T: io::Read>(out_dir: &Path, csv_reader: &mut csv::Reader<T>) {
//     let csr_load_store = out_dir
//         .join("src")
//         .join("generated")
//         .join("csr_load_store.rs");
//     if fs::exists(&csr_load_store).unwrap_or(true) {
//         fs::remove_file(&csr_load_store).expect("remove csr_load_store");
//     }
//     let mut handle = OpenOptions::new()
//         .write(true)
//         .create(true)
//         .open(&csr_load_store)
//         .expect("csr_load_store.rs");

//     let records = csv_reader.records().collect::<Vec<_>>();

//     let raw = r#"use crate::{csrs::Csrs, cpu::{Exception, Result}};

// impl Csrs {"#;
//     writeln!(handle, "{}", raw).expect("write");

//     for line in &records {
//         let line = line.as_ref().expect("csv parse failed");
//         let address = &line[0];
//         let csr_name = line[1].trim().trim_matches('"').to_uppercase();

//         writeln!(handle, "    pub const {csr_name}: u64 = {address};").expect("write");
//     }

//     writeln!(
//         handle,
//         r#"
//     pub fn load(&self, address: u64) -> Result<u64> {{
//         match address {{"#
//     )
//     .expect("write");

//     for line in &records {
//         let line = line.as_ref().expect("csv parse failed");
//         let csr_name = line[1].trim().trim_matches('"').to_uppercase();

//         writeln!(
//             handle,
//             "            Self::{csr_name} => Ok(self.regs[Self::{csr_name} as usize]),"
//         )
//         .expect("write");
//     }

//     writeln!(
//         handle,
//         "            _ => Err(Exception::IllegalInstruction)
//         }}
//     }}
// "
//     )
//     .expect("write");

//     writeln!(
//         handle,
//         r#"
//     pub fn store(&mut self, address: u64, value: u64) -> Result<()> {{
//         match address {{"#
//     )
//     .expect("write");

//     for line in &records {
//         let line = line.as_ref().expect("csv parse failed");
//         let csr_name = line[1].trim().trim_matches('"').to_uppercase();

//         writeln!(
//             handle,
//             "            Self::{csr_name} => Ok(self.regs[Self::{csr_name} as usize] = value),"
//         )
//         .expect("write");
//     }

//     writeln!(
//         handle,
//         "            _ => Err(Exception::IllegalInstruction)
//         }}
//     }}
// }}"
//     )
//     .expect("write");
// }

#[derive(Deserialize)]
struct Insn {
    mask: String,
    r#match: String,
    variable_fields: Vec<String>,
}

fn get_instr_dict(opcodes_dir: impl AsRef<Path>) -> BTreeMap<String, Insn> {
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

#[derive(Deserialize, Debug)]
struct ArgLut {
    arg: String,
    end: u8,
    start: u8,
}

fn get_arg_luts(opcodes_dir: impl AsRef<Path>) -> Vec<ArgLut> {
    let mut arg_lut_csv = fs::read_to_string(opcodes_dir.as_ref().join("arg_lut.csv"))
        .expect("riscv-opcodes arg_lut.csv");
    arg_lut_csv = arg_lut_csv.replace(' ', ""); // causes issues with CSV parsing

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(arg_lut_csv.as_bytes());

    reader
        .deserialize()
        .map(|r| r.expect("riscv-opcodes arg_lut.csv invalid record"))
        .collect::<Vec<_>>()
}

fn indent(source: &str, n: usize) -> String {
    source
        .split("\n")
        .enumerate()
        .map(|(idx, line)| {
            if idx == 0 {
                line.to_owned() + "\n"
            } else {
                "    ".repeat(n) + line + "\n"
            }
        })
        .collect()
}
