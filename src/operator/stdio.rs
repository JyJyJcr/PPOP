use crate::pipe::PipeSender;

use super::IOp;

#[derive(Debug)]
pub struct SxPrintf {}
impl IOp for SxPrintf {
    type Input = String;

    type Output = String;

    fn new(_: &str) -> Self {
        Self {}
    }

    fn exec(&self, e: &Self::Input, po: &PipeSender<Self::Output>) {
        print!("{}", e);
        po.send(e.clone());
    }
}
