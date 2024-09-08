pub mod add;
pub mod replace;
pub mod stdio;

use std::fmt::Debug;

use crate::{
    agent::Agent,
    pipe::{PipeReceiver, PipeSender},
    pipe_board::PipeBoard,
};

pub trait IOp: Debug {
    type Input: 'static + Debug;
    type Output: 'static + Debug;
    fn new(imm: &str) -> Self;
    fn exec(&self, e: &Self::Input, po: &PipeSender<Self::Output>);
    fn build(pi: &str, imm: &str, po: &str, board: &mut PipeBoard<String>) -> IAgent<Self>
    where
        Self: Sized,
    {
        let (_, pi) = board.get(pi.to_string());
        let (po, _) = board.get(po.to_string());
        IAgent::<Self> {
            pi,
            op: Self::new(imm),
            po,
        }
    }
}

#[derive(Debug)]
pub struct IAgent<I: IOp> {
    pi: PipeReceiver<I::Input>,
    op: I,
    po: PipeSender<I::Output>,
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

pub trait YOp: Debug {
    type Input1: 'static + Debug;
    type Input2: 'static + Debug;
    type Output: 'static + Debug;
    fn new() -> Self;
    fn exec(&self, e1: &Self::Input1, e2: &Self::Input2, po: &PipeSender<Self::Output>);
    fn build(pi1: &str, pi2: &str, po: &str, board: &mut PipeBoard<String>) -> YAgent<Self>
    where
        Self: Sized,
    {
        let (_, pi1) = board.get(pi1.to_string());
        let (_, pi2) = board.get(pi2.to_string());
        let (po, _) = board.get(po.to_string());
        YAgent::<Self> {
            pi1,
            pi2,
            op: Self::new(),
            po,
        }
    }
}

#[derive(Debug)]
pub struct YAgent<Y: YOp> {
    pi1: PipeReceiver<Y::Input1>,
    pi2: PipeReceiver<Y::Input2>,
    op: Y,
    po: PipeSender<Y::Output>,
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
