use crate::{op_imm::*, Element};
use anyhow::anyhow;

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
