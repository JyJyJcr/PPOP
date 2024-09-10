use std::{
    any::{Any, TypeId},
    cell::Ref,
    fmt::Debug,
};

use anyhow::{anyhow, Context};
use pipe::{PipeInlet, PipeOutlet, PipeStem};

mod pipe;

pub struct PipeBuilder {
    ty: TypeId,
    holder: PipeHolder,
}

enum PipeHolder {
    Concrete(Box<dyn Any>),
    Any(PipeStem<Box<dyn Any>>),
}

// enum PipeSenderWrapper {
//     Concrete(Box<dyn Any>),
//     Any((TypeId, PipeInlet<Box<dyn Any>>)),
// }

// enum PipeReceiverWrapper {
//     Concrete(Box<dyn Any>),
//     Any((TypeId, PipeOutlet<Box<dyn Any>>)),
// }

impl PipeBuilder {
    pub fn new<T: 'static>() -> Self {
        Self {
            ty: TypeId::of::<T>(),
            holder: PipeHolder::Concrete(Box::new(PipeStem::<T>::new())),
        }
    }
    pub fn new_by_type_id(ty: TypeId) -> Self {
        if ty == TypeId::of::<String>() {
            return Self::new::<String>();
        }
        if ty == TypeId::of::<()>() {
            return Self::new::<()>();
        }
        Self {
            ty,
            holder: PipeHolder::Any(PipeStem::new()),
        }
    }
    pub fn ask_type_id(&self) -> TypeId {
        self.ty
    }

    pub fn require_sender<T: 'static>(&self) -> anyhow::Result<PipeSender<T>> {
        match &self.holder {
            PipeHolder::Concrete(pc) => Ok(PipeSender::Concrete(
                pc.downcast_ref::<PipeStem<T>>()
                    .context("pipe type conflict")?
                    .require_inlet(),
            )),
            PipeHolder::Any(ps) => {
                if self.ty == TypeId::of::<T>() {
                    Ok(PipeSender::Any(ps.require_inlet()))
                } else {
                    Err(anyhow!("pipe type conflict"))
                }
            }
        }
    }
    pub fn require_receiver<T: 'static>(&self) -> anyhow::Result<PipeReceiver<T>> {
        match &self.holder {
            PipeHolder::Concrete(pc) => Ok(PipeReceiver::Concrete(
                pc.downcast_ref::<PipeStem<T>>()
                    .context("pipe type conflict")?
                    .require_outlet(),
            )),
            PipeHolder::Any(ps) => {
                if self.ty == TypeId::of::<T>() {
                    Ok(PipeReceiver::Any(ps.require_outlet()))
                } else {
                    Err(anyhow!("pipe type conflict"))
                }
            }
        }
    }
}

//#[derive(Debug)]
pub enum PipeSender<T> {
    Concrete(PipeInlet<T>),
    Any(PipeInlet<Box<dyn Any>>),
}
impl<T> Debug for PipeSender<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Concrete(arg0) => f.debug_tuple("Concrete").field(arg0).finish(),
            Self::Any(arg0) => f.debug_tuple("Any").field(arg0).finish(),
        }
    }
}

impl<T> PipeSender<T> {
    pub fn send(&self, t: T)
    where
        T: 'static,
    {
        match self {
            PipeSender::Concrete(pc) => pc.send(t),
            PipeSender::Any(pa) => pa.send(Box::new(t)),
        }
    }
}

//#[derive(Debug)]
pub enum PipeReceiver<T> {
    Concrete(PipeOutlet<T>),
    Any(PipeOutlet<Box<dyn Any>>),
}
impl<T> Debug for PipeReceiver<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Concrete(arg0) => f.debug_tuple("Concrete").field(arg0).finish(),
            Self::Any(arg0) => f.debug_tuple("Any").field(arg0).finish(),
        }
    }
}

impl<T> PipeReceiver<T> {
    pub fn recv(&self) -> Option<Ref<T>>
    where
        T: 'static,
    {
        match self {
            PipeReceiver::Concrete(pc) => pc.recv(),
            PipeReceiver::Any(pa) => pa.recv().map(|t| {
                Ref::map(t, |t| {
                    t.downcast_ref::<T>()
                        .expect("pipe error: unexpected object sent")
                })
            }),
        }
    }
    pub fn is_recvable(&self) -> bool {
        match self {
            PipeReceiver::Concrete(pc) => pc.is_recvable(),
            PipeReceiver::Any(pa) => pa.is_recvable(),
        }
    }
    pub fn is_alive(&self) -> bool {
        match self {
            PipeReceiver::Concrete(pc) => pc.is_alive(),
            PipeReceiver::Any(pa) => pa.is_alive(),
        }
    }
}

// pub trait PipeBuilderWrapper {
//     fn require_sender(&self) -> Box<dyn Any>;
//     fn require_receiver(&self) -> Box<dyn Any>;
// }

// impl<T: 'static> PipeBuilderWrapper for PipeBuilder<T> {
//     fn require_sender(&self) -> Box<dyn Any> {
//         Box::new(self.require_sender())
//     }

//     fn require_receiver(&self) -> Box<dyn Any> {
//         Box::new(self.require_receiver())
//     }
// }

// pub fn require_sender<T: 'static>(pb: &PipeBuilder) -> anyhow::Result<PipeSender<T>> {
//     match pb.require_sender() {
//         Ok(pb) => Ok(pb),
//         Err(_) => Err(anyhow!("pipe type conflict")),
//     }
// }

// pub fn require_receiver<T: 'static>(pb: &PipeBuilder) -> anyhow::Result<PipeReceiver<T>> {
//     match pb.require_receiver().try_into() {
//         Ok(pb) => Ok(*pb),
//         Err(_) => Err(anyhow!("pipe type conflict")),
//     }
// }

#[cfg(test)]
mod tests {
    use std::{any::Any, fmt::Display, marker::PhantomData};

    use crate::pipe::{pipe::PipeStem, PipeBuilder};

    #[test]
    fn test() -> anyhow::Result<()> {
        println!("hello");

        let b = PipeBuilder::new::<usize>();

        let r = b.require_receiver::<usize>()?;
        let s = b.require_sender::<usize>()?;
        //println!("{:?} {:?}", s, r);
        let rr = b.require_receiver::<usize>()?;
        let ss = b.require_sender::<usize>()?;

        println!("{:?} {:?} {:?} {:?}", s, r, ss, rr);

        drop(b);

        s.send(10);

        println!("{:?} {:?} {:?} {:?}", s, r, ss, rr);

        // match &s {
        //     crate::pipe::PipeSender::Concrete(pc) => pc.,
        //     crate::pipe::PipeSender::Any(pa) => todo!(),
        // }

        drop(s);
        println!("{}", r.is_alive());
        println!("{}", rr.is_alive());
        drop(ss);
        rr.recv();
        rr.recv();

        println!("{}", r.is_alive());
        drop(r);
        println!("{}", rr.is_alive());
        //println!("{:?} {:?}", r, rr);
        Ok(())
    }

    // fn convert<'a, T: ?Sized, Q: ?Sized>(t: &'a T) -> &'a Q
    // where
    //     &'a Q: From<&'a T>,
    // {
    //     t.into()
    // }

    // #[test]
    // fn dyntest() {
    //     let x = PhantomData::<String>;

    //     let s: String = "aaa".to_string();
    //     let xx: String = s.into();

    //     let y: &dyn Any = convert(&x);

    //     println!("{:?}", y.downcast_ref::<PhantomData<dyn Display>>());

    //     // let a = TypeId::of::<dyn Display>();

    //     // let s: dyn Display = String::from("aaa") as dyn Display;
    // }
}
