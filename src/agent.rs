// use std::any::{Any, TypeId};

// // struct IO {
// //     b: Box<dyn IOp>,
// // }

// // enum Op {
// //     I(Box<dyn IOp>),
// //     Y(dyn YOp),
// // }

// trait IOpWrapper {
//     fn exec_any(&self, i: &dyn Any) -> Box<dyn Any>;
// }

// impl<T: IOp> IOpWrapper for T {
//     fn exec_any(&self, i: &dyn Any) -> Box<dyn Any> {
//         Box::new(self.exec(i.downcast_ref().unwrap()))
//     }
// }

// trait YOpWrapper {
//     fn exec_any(&self, i1: &dyn Any, i2: &dyn Any) -> Box<dyn Any>;
// }

// impl<T: YOp> YOpWrapper for T {
//     fn exec_any(&self, i1: &dyn Any, i2: &dyn Any) -> Box<dyn Any> {
//         Box::new(self.exec(i1.downcast_ref().unwrap(), i2.downcast_ref().unwrap()))
//     }
// }

// pub struct IAgent {
//     t_i1: TypeId,
//     op: Box<dyn IOpWrapper>,
//     t_o: TypeId,
// }
// pub struct YAgent {
//     t_i1: TypeId,
//     t_i2: TypeId,
//     op: Box<dyn YOpWrapper>,
//     t_o: TypeId,
// }

// impl YAgent {
//     fn new<Y: YOp + 'static>(op: Y) -> Self {
//         Self {
//             t_i1: TypeId::of::<Y::Input1>(),
//             t_i2: TypeId::of::<Y::Input2>(),
//             op: Box::new(op),
//             t_o: TypeId::of::<Y::Output>(),
//         }
//     }
//     fn exec_any(&self, i: &dyn Any) -> Box<dyn Any> {
//         assert_eq!(t_)
//     }
// }

use std::any::Any;

use crate::{
    operator::{IOp, YOp},
    pipe::{PipeReceiver, PipeSender},
};

struct YAgent<Y: YOp> {
    pi1: PipeReceiver<Y::Input1>,
    pi2: PipeReceiver<Y::Input2>,
    op: Y,
    po: PipeSender<Y::Output>,
}

impl<Y: YOp> YAgent<Y> {
    fn from(pi1: &dyn Any, pi2: &dyn Any, op: Y, po: &dyn Any) -> Self {
        let pi1 = pi1
            .downcast_ref::<PipeReceiver<Y::Input1>>()
            .unwrap()
            .to_owned();
        let pi2 = pi2
            .downcast_ref::<PipeReceiver<Y::Input2>>()
            .unwrap()
            .clone();
        let po = po
            .downcast_ref::<PipeSender<Y::Output>>()
            .unwrap()
            .to_owned();
        Self { pi1, pi2, op, po }
    }
}
impl<Y: YOp> Agent for YAgent<Y> {
    fn step(&self) -> bool {
        if self.pi1.is_alive() && self.pi2.is_alive() {
            if self.pi1.is_recvable() && self.pi2.is_recvable() {
                let e1 = self.pi1.recv().unwrap();
                let e2 = self.pi2.recv().unwrap();
                self.op.exec(&e1, &e2, &self.po);
            }
            true
        } else {
            false
        }
    }
}
struct IAgent<I: IOp> {
    pi: PipeReceiver<I::Input>,
    op: I,
    po: PipeSender<I::Output>,
}

impl<I: IOp> IAgent<I> {
    fn from(pi: &dyn Any, op: I, po: &dyn Any) -> Self {
        let pi = pi
            .downcast_ref::<PipeReceiver<I::Input>>()
            .unwrap()
            .to_owned();
        let po = po
            .downcast_ref::<PipeSender<I::Output>>()
            .unwrap()
            .to_owned();
        Self { pi, op, po }
    }
}
impl<I: IOp> Agent for IAgent<I> {
    fn step(&self) -> bool {
        if self.pi.is_alive() {
            if self.pi.is_recvable() {
                let e = self.pi.recv().unwrap();
                self.op.exec(&e, &self.po);
            }
            true
        } else {
            false
        }
    }
}

trait Agent {
    fn step(&self) -> bool;
}

#[cfg(test)]
mod tests {

    trait T<A> {
        fn ex(a: A) -> bool;
    }
    struct X {
        i: i32,
    }

    #[test]
    fn test() {}
}
