use crate::{
    rpn,
    util::{capitalize_first_letter, indent},
};
use serde::{Deserialize, Deserializer};
use serde_yaml::Value;
use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
    str::FromStr,
};

#[derive(Debug, Deserialize)]
pub struct CsrDatabase {
    pub regs: HashMap<String, Register>,
}

impl CsrDatabase {
    pub fn generate_bitfields(&self, src_dir: impl AsRef<Path>) {
        let bitfield_file = src_dir.as_ref().join("generated/csr_bitfields.rs");
        if matches!(fs::exists(&bitfield_file), Ok(true)) {
            return;
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(bitfield_file)
            .expect("opening csr_bitfields.rs");

        writeln!(file, "#![cfg_attr(rustfmt, rustfmt_skip)]").expect("work");
        writeln!(file, "use modular_bitfield::bitfield;\n").expect("work");

        for (reg_name, reg) in &self.regs {
            writeln!(file, "{}\n", reg.generate_bitfield(&reg_name))
                .expect("writing to csr_bitfields.rs");
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Register {
    pub number: Option<u32>,
    pub desc: Option<String>,
    #[serde(rename = "priv", default)]
    pub privilege: Option<String>,
    pub mmio: Option<bool>,
    pub width: Option<BitSpec>,
    pub arch: Option<String>,
    pub per_hart: Option<bool>,
    pub repeat: Option<String>,
    pub fields: Option<HashMap<String, Field>>,
}

impl Register {
    pub fn generate_bitfield(&self, csr_name: &str) -> String {
        let mut field_strings = Vec::new();
        let mut field_enum_strings = vec!["".to_string(), "".to_string()]; // spacing hack

        if let Some(fields) = &self.fields {
            let mut sorted = fields.into_iter().collect::<Vec<_>>();
            sorted.sort_unstable_by_key(|(_, field)| field.bits.lsb());

            let mut current_bit = 0;
            for (field_name, field) in sorted {
                if field.bits.lsb() > current_bit {
                    field_strings.push(format!("#[skip] __: B{},", field.bits.lsb() - current_bit));
                }

                if let Some(enums) = &field.enums {
                    const BITFIELD_ENUM_TPL: &str = include_str!("../tpl/csr_field_enum.txt");

                    let enum_strings = enums.iter().map(|(enum_name, enum_value)| {
                        format!("{} = {}, ", enum_name.to_ascii_uppercase(), enum_value)
                    });

                    let enum_name = capitalize_first_letter(csr_name)
                        + &field_name
                            .split('_')
                            .map(capitalize_first_letter)
                            .collect::<String>();

                    field_enum_strings.push(
                        BITFIELD_ENUM_TPL
                            .replace("{enum_name}", &enum_name)
                            .replace("{values}", &enum_strings.collect::<String>())
                            .replace("{bits}", &field.bits.length().to_string()),
                    );
                    field_strings.push(format!("{}: {},", field_name, enum_name));
                } else {
                    field_strings.push(format!("{}: B{},", field_name, field.bits.length()));
                }

                current_bit = field.bits.msb() + 1;
            }
            if current_bit < 64 {
                // total bits
                field_strings.push(format!("#[skip] __: B{}", 64 - current_bit));
            }
        } else {
            const NO_FIELD_CSR: &str = "data: B64";
            field_strings.push(NO_FIELD_CSR.to_string());
        }

        const BITFIELD_TPL: &str = include_str!("../tpl/csr_bitfield.txt");
        BITFIELD_TPL
            .replace("{csr_name}", &capitalize_first_letter(csr_name))
            .replace("{fields}", &indent(&field_strings.join("\n"), 1))
            + &field_enum_strings.join("\n")
    }
}

#[derive(Debug, Deserialize)]
pub struct Field {
    pub desc: Option<String>,
    pub bits: BitRange,
    pub enums: Option<HashMap<String, String>>,
    pub condition: Option<Condition>, // field exists only if mask && match
    pub custom: Option<bool>,         // platform-defined
    pub width: Option<BitSpec>,
    #[serde(rename = "priv")]
    pub field_spec: Option<String>, // field specification (W*R*).
}

#[derive(Debug, Deserialize)]
pub struct Condition {
    pub mask: BitSpec,
    pub value: BitSpec,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum BitRange {
    Single(BitSpec),
    Range(BitSpec, BitSpec),
}

impl BitRange {
    fn msb(&self) -> u64 {
        match self {
            Self::Single(b) => b.0,
            Self::Range(a, b) => a.0.max(b.0),
        }
    }

    fn lsb(&self) -> u64 {
        match self {
            Self::Single(b) => b.0,
            Self::Range(a, b) => a.0.min(b.0),
        }
    }

    fn length(&self) -> u64 {
        match self {
            Self::Single(_) => 1,
            Self::Range(a, b) => self.msb() - self.lsb() + 1,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub struct BitSpec(u64);

impl FromStr for BitSpec {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const XLEN: &str = "64";

        let expr = s
            .replace("mxlen", XLEN)
            .replace("sxlen", XLEN)
            .replace("uxlen", XLEN);

        rpn::eval_expr(&expr).map(|i| BitSpec(i))
    }
}

impl<'de> Deserialize<'de> for BitSpec {
    fn deserialize<D>(deserializer: D) -> Result<BitSpec, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        match value {
            Value::Number(num) => {
                if let Some(u) = num.as_u64() {
                    Ok(BitSpec(u))
                } else {
                    Err(serde::de::Error::custom("expected a non-negative integer"))
                }
            }
            Value::String(s) => FromStr::from_str(&s).map_err(serde::de::Error::custom),
            _ => Err(serde::de::Error::custom(
                "expected an integer or string for BitSpec",
            )),
        }
    }
}

impl<'de> Deserialize<'de> for BitRange {
    fn deserialize<D>(deserializer: D) -> Result<BitRange, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vec = Vec::<BitSpec>::deserialize(deserializer)?;
        match vec.len() {
            1 => Ok(BitRange::Single(vec.into_iter().next().unwrap())),
            2 => {
                let mut iter = vec.into_iter();
                let high = iter.next().unwrap();
                let low = iter.next().unwrap();
                Ok(BitRange::Range(high, low))
            }
            other => Err(serde::de::Error::custom(format!(
                "Expected 1 or 2 elements for a bit specification, got {}",
                other
            ))),
        }
    }
}

pub fn generate_csr_structs(isa_data_dir: impl AsRef<Path>, src_dir: impl AsRef<Path>) {
    let csr_yaml = fs::read_to_string(isa_data_dir.as_ref().join("csr.yaml")).unwrap();
    let db = serde_yaml::from_str::<CsrDatabase>(&csr_yaml).expect("yaml parse");

    db.generate_bitfields(&src_dir);

    panic!();
}
