use indexmap::IndexMap;
// use serde::Deserialize;
use serde::Deserialize;
use serde_json::{self, value::Index};
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
    encoding: String,
    variable_fields: Vec<String>,
    extension: Vec<String>,
    #[serde(rename = "match")]
    i_match: String,
    mask: String,
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

        let field_bindings_raw_insn_params = insn
            .variable_fields
            .iter()
            .enumerate()
            .map(|(i, field)| {
                let comma = if i + 1 < insn.variable_fields.len() {
                    ","
                } else {
                    ""
                };
                format!("{field}{comma}")
            })
            .collect::<Vec<_>>()
            .join("");

        let mut bus_param: &str = "";
        if [
            "lb", "lh", "lw", "lbu", "lhu", "sb", "sh", "sw", "ld", "sd", "lwu", "flw", "fsw",
            "fld", "fsd", "c_lw", "c_sw", "c_ld", "c_sd", "c_lwsp", "c_swsp", "c_ldsp", "c_sdsp",
            "c_flw", "c_fsw", "c_fld", "c_fsd", "c_flwsp", "c_fswsp", "c_fldsp", "c_fsdsp",
        ]
        .iter()
        .any(|&s| s == insn_name)
        {
            bus_param = "bus,";
        }

        let raw = format!(
            r#"use crate::{{cpu::{{self, Cpu, Insn}}, bus::Bus}};
use super::insn_raw;

pub fn {insn_name}(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {{
    {field_bindings}
    insn_raw::{insn_name}_raw::{insn_name}_raw(cpu, {bus_param}{field_bindings_raw_insn_params})
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
    let mut mod_decls = config
        .keys()
        .map(|i| format!("pub mod {i};"))
        .collect::<Vec<_>>()
        .join("\n")
        + "\npub mod insn_cached;\npub mod insn_raw;";
    fs::write(&mod_rs, mod_decls).expect("mod.rs");
}

fn generate_raw_instruction_files(out_dir: &Path, config: &BTreeMap<String, ParsedInsn>) {
    for (insn_name, insn) in config.iter() {
        let field_bindings = insn
            .variable_fields
            .iter()
            .enumerate()
            .map(|(i, field)| {
                let comma = if i + 1 < insn.variable_fields.len() {
                    ","
                } else {
                    ""
                };
                format!("{field}: u64{comma}")
            })
            .collect::<Vec<_>>()
            .join("");

        let mut bus_param: &str = "";
        if [
            "lb", "lh", "lw", "lbu", "lhu", "sb", "sh", "sw", "ld", "sd", "lwu", "flw", "fsw",
            "fld", "fsd", "c_lw", "c_sw", "c_ld", "c_sd", "c_lwsp", "c_swsp", "c_ldsp", "c_sdsp",
            "c_flw", "c_fsw", "c_fld", "c_fsd", "c_flwsp", "c_fswsp", "c_fldsp", "c_fsdsp",
        ]
        .iter()
        .any(|&s| s == insn_name)
        {
            bus_param = "bus: &mut Bus, ";
        }
        let raw = format!(
            r#"use crate::{{cpu::{{self, Cpu, Insn}}, bus::Bus}};

pub fn {insn_name}_raw(cpu: &mut Cpu, {bus_param}todo()) -> cpu::Result<u64> {{
    todo!();
}}"#
        );

        let f = out_dir
            .join("src")
            .join("insn_impl/insn_raw")
            .join(format!("{}_raw.rs", insn_name));
        if !fs::exists(&f).unwrap_or_default() {
            fs::write(&f, raw).expect("insn_impl write");
        }
    }

    let mod_rs = out_dir
        .join("src")
        .join("insn_impl/insn_raw")
        .join("mod.rs");
    let mut mod_decls = config
        .keys()
        .map(|i| format!("pub mod {i}_raw;"))
        .collect::<Vec<_>>()
        .join("\n")
        + "\npub mod nop_raw;";
    fs::write(&mod_rs, mod_decls).expect("mod.rs");
}



fn generate_cached_instruction_files(out_dir: &Path, config: &BTreeMap<String, ParsedInsn>) {
    for (insn_name, insn) in config.iter() {
        let field_bindings = insn
            .variable_fields
            .iter()
            .enumerate()
            .map(|(i, field)| {
                let comma = if i + 1 < insn.variable_fields.len() {
                    ","
                } else {
                    ""
                };
                format!("{field}: u64{comma}")
            })
            .collect::<Vec<_>>()
            .join("");

        let field_bindings_args = insn
            .variable_fields
            .iter()
            .enumerate()
            .map(|(i, field)| {
                let comma = if i + 1 < insn.variable_fields.len() {
                    ","
                } else {
                    ""
                };
                format!("cache_entry.{field}{comma}")
            })
            .collect::<Vec<_>>()
            .join("");

        let mut bus_param: &str = "";
        if [
            "lb", "lh", "lw", "lbu", "lhu", "sb", "sh", "sw", "ld", "sd", "lwu", "flw", "fsw",
            "fld", "fsd", "c_lw", "c_sw", "c_ld", "c_sd", "c_lwsp", "c_swsp", "c_ldsp", "c_sdsp",
            "c_flw", "c_fsw", "c_fld", "c_fsd", "c_flwsp", "c_fswsp", "c_fldsp", "c_fsdsp",
        ]
        .iter()
        .any(|&s| s == insn_name)
        {
            bus_param = "bus, ";
        }

        let raw = format!(
            r#"use crate::{{bus::Bus, cpu::{{self, Cpu, Insn}}, insn_impl::insn_raw, uop_cache::uop_cache::UopCacheEntry}};

pub fn {insn_name}_cached(cpu: &mut Cpu, bus: &mut Bus, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {{
    insn_raw::{insn_name}_raw::{insn_name}_raw(cpu, {bus_param}, todo())
}}"#
        );

        let f = out_dir
            .join("src")
            .join("insn_impl/insn_cached")
            .join(format!("{}_cached.rs", insn_name));
        if !fs::exists(&f).unwrap_or_default() {
            fs::write(&f, raw).expect("insn_impl write");
        }
    }
    let mod_rs = out_dir
        .join("src")
        .join("insn_impl/insn_cached")
        .join("mod.rs");
    let mut mod_decls = config
        .keys()
        .map(|i| format!("pub mod {i}_cached;"))
        .collect::<Vec<_>>()
        .join("\n")
        + "\npub mod nop_cached;";
    fs::write(&mod_rs, mod_decls).expect("mod.rs");
}


fn generate_set_cached_insn(out_dir: &Path, config: &IndexMap<String, ParsedInsn>) {
    let set_cached_insn = out_dir.join("src").join("uop_cache").join("set_cached_insn.rs");
    if fs::exists(&set_cached_insn).unwrap_or(true) {
        fs::remove_file(&set_cached_insn).expect("remove set_cached_insn");
    }
    let mut handle = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&set_cached_insn)
        .expect("set_cached_insn.rs");

    let base = r#"use crate::{bus::Bus, cpu::{self, Cpu}, insn_impl::insn_cached};
use super::uop_cache::UopCacheEntry;

impl UopCacheEntry {
    pub fn set_cached_insn(bits: u64) -> Option<fn(cpu: &mut Cpu, bus: &mut Bus, &UopCacheEntry) -> cpu::Result<u64>> {"#;

    writeln!(handle, "{}", base).expect("write");

    for (idx, (insn_name, insn)) in config.iter().enumerate() {
        writeln!(
            handle,
            r#"
        {}if bits & {} == {} {{
            Some(insn_cached::{insn_name}_cached::{insn_name}_cached)
        }}"#,
            if idx != 0 { "else " } else { "" },
            insn.mask,
            insn.i_match
        )
        .expect("write");
    }

    let end = r#"
        else {
            None
        }
    }
}
"#;
    writeln!(handle, "{}", end).expect("write");
}

fn generate_cpu_execute_arms(out_dir: &Path, config: &IndexMap<String, ParsedInsn>) {
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
    branch_hints::unlikely, bus::{Bus, Device}, cpu::{self, Cpu, Insn}, insn_impl, uop_cache::uop_cache::UopCacheEntry
};

impl Cpu {
    pub fn execute_insn(&mut self, cache_ptr: Option<*const UopCacheEntry>, bus: &mut Bus) -> cpu::Result<u64> {
        if let Some(cached_insn) = cache_ptr {
                // println!("bits: {:#08x}", cached_insn.insn_bits);
                unsafe {
                    let result = (*cached_insn).execute_cached_insn(self, bus);
                    return result;
                }
        } else if unlikely(true) {
            let mut bytes = [0; std::mem::size_of::<u32>()];
            bus.read(self.pc, &mut bytes).expect("invalid dram address");
            let insn = Insn::from_bytes(&bytes);
            let bits = insn.bits();"#;

    writeln!(handle, "{}", base).expect("write");

    for (idx, (insn_name, insn)) in config.iter().enumerate() {
        writeln!(
            handle,
            r#"
            {}if bits & {} == {} {{
                insn_impl::{insn_name}::{insn_name}(insn, self, bus)
            }}"#,
            if idx != 0 { "else " } else { "" },
            insn.mask,
            insn.i_match
        )
        .expect("write");
    }

    let end = r#"
            else {
                Err(cpu::Exception::IllegalInstruction)
            }
        } else {
            panic!();
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

fn generate_csr_load_store<T: io::Read>(out_dir: &Path, csv_reader: &mut csv::Reader<T>) {
    let csr_load_store = out_dir
        .join("src")
        .join("generated")
        .join("csr_load_store.rs");
    if fs::exists(&csr_load_store).unwrap_or(true) {
        fs::remove_file(&csr_load_store).expect("remove csr_load_store");
    }
    let mut handle = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&csr_load_store)
        .expect("csr_load_store.rs");

    let records = csv_reader.records().collect::<Vec<_>>();

    let raw = r#"use crate::{csrs::Csrs, cpu::{Exception, Result}};

impl Csrs {"#;
    writeln!(handle, "{}", raw).expect("write");

    for line in &records {
        let line = line.as_ref().expect("csv parse failed");
        let address = &line[0];
        let csr_name = line[1].trim().trim_matches('"').to_uppercase();

        writeln!(handle, "    pub const {csr_name}: u64 = {address};").expect("write");
    }

    writeln!(
        handle,
        r#"
    pub fn load(&self, address: u64) -> Result<u64> {{
        match address {{"#
    )
    .expect("write");

    for line in &records {
        let line = line.as_ref().expect("csv parse failed");
        let csr_name = line[1].trim().trim_matches('"').to_uppercase();

        writeln!(
            handle,
            "            Self::{csr_name} => Ok(self.regs[Self::{csr_name} as usize]),"
        )
        .expect("write");
    }

    writeln!(
        handle,
        "            _ => Err(Exception::IllegalInstruction)
        }}
    }}
"
    )
    .expect("write");

    writeln!(
        handle,
        r#"
    pub fn store(&mut self, address: u64, value: u64) -> Result<()> {{
        match address {{"#
    )
    .expect("write");

    for line in &records {
        let line = line.as_ref().expect("csv parse failed");
        let csr_name = line[1].trim().trim_matches('"').to_uppercase();

        writeln!(
            handle,
            "            Self::{csr_name} => Ok(self.regs[Self::{csr_name} as usize] = value),"
        )
        .expect("write");
    }

    writeln!(
        handle,
        "            _ => Err(Exception::IllegalInstruction)
        }}
    }}
}}"
    )
    .expect("write");
}

fn main() {
    let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    let spec_dir = out_dir.join("..").join("riscv-opcodes");
    println!(
        "cargo::rerun-if-changed={}",
        out_dir
            .join("../.git/modules/riscv-opcodes/refs/heads/master")
            .canonicalize()
            .unwrap()
            .to_string_lossy()
    );

    let mut path = env::var("PATH").expect("reading PATH failed");
    if let Ok(venv_path) = env::var("VIRTUAL_ENV") {
        path = format!("{}/bin:{}", venv_path, path);
        println!("cargo::warning={}", path);
    }

    let cmd = Command::new("make")
        .arg("EXTENSIONS=rv_i rv64_i rv_zicsr rv_system rv_c rv64_c rv_f rv64_f rv_d rv64_d rv_m rv64_m rv_c_d rv_zifencei")
        .current_dir(&spec_dir)
        .env("PATH", path)
        .output()
        .expect("running make failed");

    // commented out because of my intellisense/pyenv shenanigans
    if !cmd.status.success() {
        panic!(
            "make failed with output: {}",
            String::from_utf8_lossy(&cmd.stderr)
        );
    }

    let mut execute_config: IndexMap<String, ParsedInsn> = serde_json::from_reader(
        File::open(&spec_dir.join("instr_dict.json")).expect("instr_dict.json not found"),
    )
    .expect("json deserialize");
    let mut config: BTreeMap<String, ParsedInsn> = serde_yaml::from_reader(
        File::open(&spec_dir.join("instr_dict.json")).expect("instr_dict.json not found"),
    )
    .expect("json deserialize");

    const EXCLUDED_INSNS: &[&str] = &[
        "mv",
        "neg",
        "zext_b",
        "ret",
        "bleu",
        "bgtu",
        "ble",
        "bgez",
        "blez",
        "bgt",
        "bgtz",
        "bltz",
        "bnez",
        "beqz",
        "seqz",
        "snez",
        "sltz",
        "sgtz",
        "jr",
        "j",
        "sext_w",
        "csrr",
        "csrw",
        "csrs",
        "csrc",
        "csrwi",
        "csrsi",
        "csrci",
        "jal_pseudo",
        "jalr_pseudo",
        "fence_tso",
        "scall",
        "sbreak",
        "pause",
    ];
    for insn in EXCLUDED_INSNS {
        config.remove(insn.to_owned());
    }

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(&spec_dir.join("arg_lut.csv"))
        .expect("arg_lut.csv not found");
    let mut rdr2 = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(&spec_dir.join("csrs.csv"))
        .expect("csrs.csv not found");

    // generate_instruction_files(&out_dir, &config);
    // generate_set_cached_insn(&out_dir, &execute_config);
    // generate_raw_instruction_files(&out_dir, &config);
    // generate_cached_instruction_files(&out_dir, &config);
    // generate_cpu_execute_arms(&out_dir, &execute_config);
    // generate_insn_arg_luts(&out_dir, &mut rdr);
    // generate_csr_load_store(&out_dir, &mut rdr2);
}
