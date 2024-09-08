use crate::{op_imm::*, Element};
use anyhow::anyhow;

#[derive(Debug)]
pub enum ImmOperator {
    SLOAD,
    ILOAD,
    ULOAD,
    FLOAD,
    BLOAD,
}

impl ImmOperator {
    pub fn exec(&self, _e: &Element, imm: &str) -> Option<Element> {
        Some(match self {
            ImmOperator::SLOAD => sload(imm),
            ImmOperator::ILOAD => iload(imm),
            ImmOperator::ULOAD => uload(imm),
            ImmOperator::FLOAD => fload(imm),
            ImmOperator::BLOAD => bload(imm),
        })
    }
}

#[derive(Debug)]
pub enum MergeOperator {
    FILTER,
    ADD,
    SUB,
    EQ,
    REPLACE,
}
impl MergeOperator {
    pub fn exec(&self, e1: &Element, e2: &Element) -> Option<Element> {
        match self {
            MergeOperator::FILTER => filter(e1, e2),
            MergeOperator::ADD => Some(add(e1, e2)),
            MergeOperator::SUB => Some(sub(e1, e2)),
            MergeOperator::EQ => Some(Element::BOOL(eq(e1, e2))),
            MergeOperator::REPLACE => Some(e2.to_owned()),
        }
    }
}

pub enum Operator {
    IMM(ImmOperator),
    MRG(MergeOperator),
}
impl Operator {
    pub fn from_grapheme(op: &str) -> anyhow::Result<Self> {
        use ImmOperator::*;
        use MergeOperator::*;
        use Operator::*;
        Ok(match op {
            "?" => MRG(FILTER),
            "+" => MRG(ADD),
            "-" => MRG(SUB),
            "=" => MRG(EQ),
            "r" => MRG(REPLACE),
            "S" => IMM(SLOAD),
            "I" => IMM(ILOAD),
            "U" => IMM(ULOAD),
            "F" => IMM(FLOAD),
            "B" => IMM(BLOAD),
            _ => return Err(anyhow!("invalid op")),
        })
    }
}
