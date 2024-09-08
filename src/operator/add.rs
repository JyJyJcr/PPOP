use crate::pipe::PipeSender;

use super::YOp;

#[derive(Debug)]
pub struct SSAdd {}
impl YOp for SSAdd {
    type Input1 = String;

    type Input2 = String;

    type Output = String;

    fn new() -> Self {
        Self {}
    }

    fn exec(&self, e1: &Self::Input1, e2: &Self::Input2, po: &PipeSender<Self::Output>) {
        po.send(format!("{}{}", e1, e2));
    }
}
