pub mod add;
pub mod delete;
pub mod load;
pub mod stdio;

use std::{fmt::Debug, marker::PhantomData};

use crate::{
    agent::Agent,
    deduct::{AgentPrecursor, PipeIndex},
    pipe::{PipeReceiver, PipeSender},
};

pub trait IOp: Debug {
    type Input: 'static;
    type Output: 'static;
    fn new(imm: String) -> anyhow::Result<Self>
    where
        Self: Sized;
    fn exec(&self, e: &Self::Input) -> impl IntoIterator<Item = Self::Output>;
}
pub trait YOp: Debug {
    type Input1: 'static;
    type Input2: 'static;
    type Output: 'static;
    fn new() -> anyhow::Result<Self>
    where
        Self: Sized;
    fn exec(&self, e1: &Self::Input1, e2: &Self::Input2) -> impl IntoIterator<Item = Self::Output>;
}

pub struct IAgent<I: IOp> {
    pi: PipeReceiver<I::Input>,
    op: I,
    po: PipeSender<I::Output>,
}

impl<I: IOp> Debug for IAgent<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IAgent")
            .field("pi", &self.pi)
            .field("op", &self.op)
            .field("po", &self.po)
            .finish()
    }
}

impl<I: IOp> Agent for IAgent<I> {
    fn step(&self) -> bool {
        if self.pi.is_alive() {
            if self.pi.is_recvable() {
                let e = self.pi.recv().unwrap();
                for eo in self.op.exec(&e).into_iter() {
                    self.po.send(eo);
                }
            }
            true
        } else {
            false
        }
    }
}

pub struct YAgent<Y: YOp> {
    pi1: PipeReceiver<Y::Input1>,
    pi2: PipeReceiver<Y::Input2>,
    op: Y,
    po: PipeSender<Y::Output>,
}

impl<Y: YOp> Debug for YAgent<Y> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("YAgent")
            .field("pi1", &self.pi1)
            .field("pi2", &self.pi2)
            .field("op", &self.op)
            .field("po", &self.po)
            .finish()
    }
}

impl<Y: YOp> Agent for YAgent<Y> {
    fn step(&self) -> bool {
        if self.pi1.is_alive() && self.pi2.is_alive() {
            if self.pi1.is_recvable() && self.pi2.is_recvable() {
                let e1 = self.pi1.recv().unwrap();
                let e2 = self.pi2.recv().unwrap();
                for eo in self.op.exec(&e1, &e2) {
                    self.po.send(eo);
                }
            }
            true
        } else {
            false
        }
    }
}

pub trait IBuildable: IOp + Sized {
    fn build(
        li: String,
        imm: String,
        lo: String,
        idx: &PipeIndex<String>,
    ) -> anyhow::Result<IAgent<Self>>;
}
impl<I: IOp> IBuildable for I {
    fn build(
        li: String,
        imm: String,
        lo: String,
        idx: &PipeIndex<String>,
    ) -> anyhow::Result<IAgent<Self>> {
        let pi = idx.require_receiver(&li)?;
        let po = idx.require_sender(&lo)?;
        Ok(IAgent::<Self> {
            pi,
            op: Self::new(imm)?,
            po,
        })
    }
}
pub trait YBuildable: YOp + Sized {
    fn build(
        li1: String,
        li2: String,
        lo: String,
        idx: &PipeIndex<String>,
    ) -> anyhow::Result<YAgent<Self>>;
}

impl<Y: YOp> YBuildable for Y {
    fn build(
        li1: String,
        li2: String,
        lo: String,
        idx: &PipeIndex<String>,
    ) -> anyhow::Result<YAgent<Self>> {
        let pi1 = idx.require_receiver(&li1)?;
        let pi2 = idx.require_receiver(&li2)?;
        let po = idx.require_sender(&lo)?;
        Ok(YAgent::<Self> {
            pi1,
            pi2,
            op: Self::new()?,
            po,
        })
    }
}

#[derive(Debug)]
pub struct IPrecursor<I: IOp> {
    pi: String,
    imm: String,
    po: String,
    ph: PhantomData<I>,
}

impl<I: 'static + IOp> IPrecursor<I> {
    pub fn new(pi: &str, imm: &str, po: &str) -> Box<dyn AgentPrecursor<String>> {
        Box::new(Self {
            pi: pi.to_string(),
            imm: imm.to_string(),
            po: po.to_string(),
            ph: PhantomData,
        })
    }
}
impl<I: 'static + IOp> AgentPrecursor<String> for IPrecursor<I> {
    fn deduct(&self, idx: &mut crate::deduct::PipeTypeIndex<String>) -> anyhow::Result<()> {
        idx.require::<I::Input, _>(&self.pi)?;
        idx.require::<I::Output, _>(&self.po)?;
        Ok(())
    }

    fn build(self: Box<Self>, idx: &PipeIndex<String>) -> anyhow::Result<Box<dyn Agent>> {
        Ok(Box::new(I::build(self.pi, self.imm, self.po, idx)?))
    }
}
#[derive(Debug)]
pub struct YPrecursor<Y: YOp> {
    pi1: String,
    pi2: String,
    po: String,
    ph: PhantomData<Y>,
}
impl<Y: 'static + YOp> YPrecursor<Y> {
    pub fn new(pi1: &str, pi2: &str, po: &str) -> Box<dyn AgentPrecursor<String>> {
        Box::new(Self {
            pi1: pi1.to_string(),
            pi2: pi2.to_string(),
            po: po.to_string(),
            ph: PhantomData,
        })
    }
}
impl<Y: 'static + YOp> AgentPrecursor<String> for YPrecursor<Y> {
    fn deduct(&self, idx: &mut crate::deduct::PipeTypeIndex<String>) -> anyhow::Result<()> {
        idx.require::<Y::Input1, _>(&self.pi1)?;
        idx.require::<Y::Input2, _>(&self.pi2)?;
        idx.require::<Y::Output, _>(&self.po)?;
        Ok(())
    }

    fn build(self: Box<Self>, idx: &PipeIndex<String>) -> anyhow::Result<Box<dyn Agent>> {
        Ok(Box::new(Y::build(self.pi1, self.pi2, self.po, idx)?))
    }
}

// #[macro_export]
// macro_rules! infer {
//     ($li1:expr,$li2:expr,$lo:expr,$board:expr,$op:ty) => {{
//         return Ok(std::boxed::Box::new(<$op>::build($li1, $li2, $lo, $board)?));
//     }};

//     ($li1:expr,$li2:expr,$lo:expr,$board:expr,$op:ty,$t1:expr,$ti1:ty) => {
//         if $t1 == std::any::TypeId::of::<$ti1>() {
//             return Ok(std::boxed::Box::new(<$op>::build($li1, $li2, $lo, $board)?));
//         }
//     };

//     ($li1:expr,$li2:expr,$lo:expr,$board:expr,$op:ty,$t1:expr,$ti1:ty,$t2:expr,$ti2:ty) => {
//         if $t1 == std::any::TypeId::of::<$ti1>() && $t2 == std::any::TypeId::of::<$ti2>() {
//             return Ok(std::boxed::Box::new(<$op>::build($li1, $li2, $lo, $board)?));
//         }
//     };

//     ($li1:expr,$li2:expr,$lo:expr,$board:expr,$op:ty,$t1:expr,$ti1:ty,$t2:expr,$ti2:ty,$t3:expr,$ti3:ty) => {
//         if $t1 == std::any::TypeId::of::<$ti1>()
//             && $t2 == std::any::TypeId::of::<$ti2>()
//             && $t3 == std::any::TypeId::of::<$ti3>()
//         {
//             return Ok(std::boxed::Box::new(<$op>::build($li1, $li2, $lo, $board)?));
//         }
//     };
// }

// #[macro_export]
// macro_rules! infer_fail {
//     () => {
//         return Err(anyhow::anyhow!("type infer failed"));
//     };
// }
