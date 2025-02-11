use crate::rpn;
use serde::{Deserialize, Deserializer};
use serde_yaml::Value;
use std::{collections::HashMap, fs, path::Path};

#[derive(Debug, Deserialize)]
pub struct CsrDatabase {
    pub regs: HashMap<String, Register>,
}

#[derive(Debug, Deserialize)]
pub struct Register {
    #[serde(default)]
    pub number: Option<u32>,

    #[serde(default)]
    pub desc: Option<String>,

    #[serde(rename = "priv", default)]
    pub priv_field: Option<String>,

    #[serde(default)]
    pub mmio: Option<bool>,

    #[serde(default)]
    pub width: Option<BitSpec>,

    #[serde(default)]
    pub arch: Option<String>,

    #[serde(default)]
    pub per_hart: Option<bool>,

    #[serde(default)]
    pub repeat: Option<String>,

    /// If the register is divided into bit–fields, this is the mapping
    /// from field name to its description.
    #[serde(default)]
    pub fields: Option<HashMap<String, Field>>,
}

#[derive(Debug, Deserialize)]
pub struct Field {
    #[serde(default)]
    pub desc: Option<String>,

    #[serde(default)]
    pub bits: Option<BitRange>,

    // #[serde(default)]
    // pub enums: Option<HashMap<String, String>>,
    /// If the field only applies under certain conditions.
    #[serde(default)]
    pub condition: Option<Condition>,

    /// Indicates whether a field is platform-defined
    #[serde(default)]
    pub custom: Option<bool>,

    #[serde(default)]
    pub width: Option<BitSpec>,

    /// The field's specification (W*R*).
    #[serde(rename = "priv", default)]
    pub field_spec: Option<String>,
}

/// A condition for a field's interpretation.
#[derive(Debug, Deserialize)]
pub struct Condition {
    pub mask: BitSpec,
    pub value: BitSpec,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BitSpec(u64);

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
            Value::String(s) => {
                let evaluated = parse_bitspec(&s);
                Ok(BitSpec(evaluated))
            }
            _ => Err(serde::de::Error::custom(
                "expected an integer or string for BitSpec",
            )),
        }
    }
}

fn parse_bitspec(expr: &str) -> u64 {
    const XLEN: &str = "64";

    let expr = expr
        .replace("mxlen", XLEN)
        .replace("sxlen", XLEN)
        .replace("uxlen", XLEN);

    rpn::eval_expr(&expr).expect("invalid bitspec")
}

/// A bit range that can either be a single bit or a range defined by two BitSpec values.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum BitRange {
    /// A single bit.
    Single(BitSpec),
    /// A range of bits (first value is the high‐bit, second is the low–bit).
    Range(BitSpec, BitSpec),
}

impl<'de> Deserialize<'de> for BitRange {
    fn deserialize<D>(deserializer: D) -> Result<BitRange, D::Error>
    where
        D: Deserializer<'de>,
    {
        // We expect the YAML to provide a sequence (a list) of BitSpec values.
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

pub fn generate_csr_structs(isa_data_dir: impl AsRef<Path>) {
    let csr_yaml = fs::read_to_string(isa_data_dir.as_ref().join("csr.yaml")).unwrap();
    let db = serde_yaml::from_str::<CsrDatabase>(&csr_yaml).expect("yaml parse");

    dbg!(&db);

    panic!();
}
