use crate::pipe::PipeSender;

use super::IOp;

#[derive(Debug)]
pub struct IsReplace {
    imm: String,
}
impl IOp for IsReplace {
    type Input = i64;

    type Output = String;

    fn new(imm: &str) -> Self {
        Self {
            imm: imm.to_owned(),
        }
    }

    fn exec(&self, _: &Self::Input, po: &PipeSender<Self::Output>) {
        po.send(self.imm.clone());
    }
}

#[derive(Debug)]
pub struct LsReplace {
    imm: String,
}
impl IOp for LsReplace {
    type Input = usize;

    type Output = String;

    fn new(imm: &str) -> Self {
        Self {
            imm: imm.to_owned(),
        }
    }

    fn exec(&self, _: &Self::Input, po: &PipeSender<Self::Output>) {
        po.send(self.imm.clone());
    }
}
