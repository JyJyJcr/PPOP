use std::fmt::Debug;

use super::IOp;
use anyhow::{anyhow, Context};

pub trait GraphemeImmediate: Sized {
    fn parse(_imm: String) -> anyhow::Result<Self> {
        Err(anyhow!("not implemented"))
    }
    fn replicate(&self) -> Self {
        panic!("GraphemeImmediate::replicate() called nut not implemented");
    }
}

pub struct Load<M> {
    imm: M,
}
impl<M> Debug for Load<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Load").field("imm", &"(IMM)").finish()
    }
}
impl<M: 'static + GraphemeImmediate> IOp for Load<M> {
    type Input = ();

    type Output = M;

    fn new(imm: String) -> anyhow::Result<Self> {
        Ok(Self {
            imm: M::parse(imm)?,
        })
    }

    fn exec(&self, _: &Self::Input) -> impl IntoIterator<Item = <Self as IOp>::Output> {
        [self.imm.replicate()]
    }
}

impl GraphemeImmediate for String {
    fn parse(imm: String) -> anyhow::Result<Self> {
        Ok(imm)
    }
    fn replicate(&self) -> Self {
        self.clone()
    }
}
impl GraphemeImmediate for u64 {
    fn parse(imm: String) -> anyhow::Result<Self> {
        imm.parse().context("grapheme parse error")
    }
    fn replicate(&self) -> Self {
        self.clone()
    }
}

// pub fn build<M: 'static + GraphemeImmediate>(
//     pi: &str,
//     imm: &str,
//     po: &str,
//     board: &mut PipeBoard<String>,
// ) -> anyhow::Result<Box<dyn Agent>> {
//     let ti = board.get_type(pi);

//     match ti {
//         Some(ti) => {
//             infer!(pi, imm, po, board, Replace<String,M>, ti, String);
//             infer!(pi, imm, po, board, Replace<u8,M>, ti, u8);
//             infer!(pi, imm, po, board, Replace<i64,M>, ti, i64);
//             infer!(pi, imm, po, board, Replace<u64,M>, ti, u64);
//             infer!(pi, imm, po, board, Replace<f64,M>, ti, f64);
//             infer!(pi, imm, po, board, Replace<isize,M>, ti, isize);
//             infer!(pi, imm, po, board, Replace<usize,M>, ti, usize);
//         }
//         None => infer!(pi, imm, po, board, Replace<String,M>),
//     }
//     infer_fail!();
// }
