// use serde::Deserialize;
use serde::Deserialize;
use std::{
    collections::BTreeMap,
    env,
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::Path,
    process::Command,
};

#[derive(Deserialize)]
struct ParsedInsn {
    mask: String,
    #[serde(rename = "match")]
    i_match: String,
    variable_fields: Vec<String>,
}

fn generate_instruction_files(out_dir: &Path, config: &BTreeMap<String, ParsedInsn>) {
    for (insn_name, insn) in config.iter() {
        let trace_args = insn
            .variable_fields
            .iter()
            .map(|field| format!("{field} = insn.{field}()"))
            .collect::<Vec<_>>()
            .join(", ");

        let field_bindings = insn
            .variable_fields
            .iter()
            .map(|field| format!("let {field} = insn.{field}();"))
            .collect::<Vec<_>>()
            .join("\n    ");

        let comma_if = if insn.variable_fields.len() > 0 {
            ", "
        } else {
            ""
        };

        let newline_if = if insn.variable_fields.len() > 0 {
            "\n"
        } else {
            ""
        };
        let raw = format!(
            r#"use crate::cpu::{{Cpu, Insn}};

pub fn {insn_name}(insn: Insn, cpu: &mut Cpu) {{
    crate::trace_insn!("{insn_name}"{comma_if}{trace_args});{newline_if}
    {field_bindings}{newline_if}
    todo!();
}}"#
        );

        let f = out_dir
            .join("src")
            .join("insn_impl")
            .join(format!("{}.rs", insn_name));
        if !fs::exists(&f).unwrap_or_default() {
            fs::write(&f, raw).expect("insn_impl write");
        }
    }

    let mod_rs = out_dir.join("src").join("insn_impl").join("mod.rs");
    let mod_decls = config
        .keys()
        .map(|i| format!("pub mod {i};"))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(&mod_rs, mod_decls).expect("mod.rs");
}

fn generate_cpu_execute_arms(out_dir: &Path, config: &BTreeMap<String, ParsedInsn>) {
    let cpu_execute = out_dir.join("src").join("generated").join("cpu_execute.rs");
    if fs::exists(&cpu_execute).unwrap_or(true) {
        fs::remove_file(&cpu_execute).expect("remove cpu_execute");
    }
    let mut handle = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&cpu_execute)
        .expect("cpu_execute.rs");

    let base = r#"use crate::{
    cpu::{Cpu, Insn},
    insn_impl,
};

impl Cpu {
    pub fn execute(&mut self, insn: Insn) {
        let bits = insn.bits();"#;

    writeln!(handle, "{}", base).expect("write");

    for (insn_name, insn) in config.iter() {
        writeln!(
            handle,
            r#"
        if bits & {} == {} {{
            insn_impl::{insn_name}::{insn_name}(insn, self);
        }}"#,
            insn.mask, insn.i_match
        )
        .expect("write");
    }

    let end = r#"
        else {
            panic!("unknown instruction!")
        }
    }
}
"#;
    writeln!(handle, "{}", end).expect("write");
}

fn generate_insn_arg_luts<T: io::Read>(out_dir: &Path, csv_reader: &mut csv::Reader<T>) {
    let insn_arg_luts = out_dir
        .join("src")
        .join("generated")
        .join("insn_arg_luts.rs");
    if fs::exists(&insn_arg_luts).unwrap_or(true) {
        fs::remove_file(&insn_arg_luts).expect("remove insn_arg_luts");
    }
    let mut handle = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&insn_arg_luts)
        .expect("insn_arg_luts.rs");

    let raw = r#"use crate::cpu::Insn;

impl Insn {"#;

    writeln!(handle, "{}", raw).expect("write");

    for line in csv_reader.records() {
        let line = line.expect("csv parse failed");
        let insn = &line[0];
        let offset = line[2].trim().parse::<u64>().expect("invalid offset");
        let len = line[1].trim().parse::<u64>().expect("invalid end") - offset + 1;

        writeln!(
            handle,
            "    pub fn {insn}(&self) -> u64 {{ self.bit_range({offset}, {len}) }}"
        )
        .expect("write");
    }
    writeln!(handle, "}}").expect("write");
}

fn main() {
    let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    let spec_dir = out_dir.join("..").join("riscv-opcodes");
    let cmd = Command::new("make")
        .arg("EXTENSIONS=rv_i rv64_i")
        .current_dir(&spec_dir)
        .output()
        .expect("running make failed");
    // commented out because of my intellisense/pyenv shenanigans
    // if !cmd.status.success() {
    //     panic!(
    //         "make failed with output: {}",
    //         String::from_utf8_lossy(&cmd.stderr)
    //     );
    // }

    let config: BTreeMap<String, ParsedInsn> = serde_yaml::from_reader(
        File::open(&spec_dir.join("instr_dict.yaml")).expect("instr_dict.yaml not found"),
    )
    .expect("yaml deserialize");
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(&spec_dir.join("arg_lut.csv"))
        .expect("arg_lut.csv not found");

    generate_instruction_files(&out_dir, &config);
    generate_cpu_execute_arms(&out_dir, &config);
    generate_insn_arg_luts(&out_dir, &mut rdr);
}
