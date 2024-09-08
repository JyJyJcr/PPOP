use crate::pipe::PipeSender;

pub trait IOp {
    type Input: 'static;
    type Output: 'static;
    fn exec(&self, e: &Self::Input, po: &PipeSender<Self::Output>);
}
pub trait YOp {
    type Input1: 'static;
    type Input2: 'static;
    type Output: 'static;
    fn exec(&self, e1: &Self::Input1, e2: &Self::Input2, po: &PipeSender<Self::Output>);
}

struct SSAdd {}
impl YOp for SSAdd {
    type Input1 = String;

    type Input2 = String;

    type Output = String;

    fn exec(&self, e1: &Self::Input1, e2: &Self::Input2, po: &PipeSender<Self::Output>) {
        po.send(format!("{}{}", e1, e2));
    }
}
struct IsReplace {
    imm: String,
}
impl IOp for IsReplace {
    type Input = i64;

    type Output = String;

    fn exec(&self, _: &Self::Input, po: &PipeSender<Self::Output>) {
        po.send(self.imm.clone());
    }
}
