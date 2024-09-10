use std::{
    cell::{OnceCell, Ref, RefCell},
    fmt::Debug,
    rc::{Rc, Weak},
};

struct PipeEntry<T> {
    buf: Rc<PipeBuf<T>>,
}
impl<T> Debug for PipeEntry<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PipeEntry").field("buf", &self.buf).finish()
    }
}

struct PipeBuf<T> {
    entry: OnceCell<Weak<PipeEntry<T>>>,
    queue: RefCell<Vec<T>>,
}
impl<T> Debug for PipeBuf<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PipeBuf")
            .field("entry", &self.entry)
            .field("queue", &format!("[={}=]", self.queue.borrow().len()))
            .finish()
    }
}

impl<T> PipeEntry<T> {
    fn pass(&self, e: T) {
        self.buf.push(e);
    }
}

impl<T> PipeBuf<T> {
    fn push(&self, e: T) {
        self.queue.borrow_mut().push(e);
    }
    fn get(&self, i: usize) -> Option<Ref<T>> {
        let q = self.queue.borrow();
        if i < q.len() {
            Some(Ref::map(self.queue.borrow(), |v| &v[i]))
        } else {
            None
        }
    }
    fn is_gettable(&self, i: usize) -> bool {
        let q = self.queue.borrow();
        i < q.len()
    }
    fn is_active(&self) -> bool {
        self.entry.get().unwrap().strong_count() != 0
    }
}

pub struct PipeInlet<T> {
    entry: Rc<PipeEntry<T>>,
}
impl<T> Debug for PipeInlet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PipeInlet")
            .field("entry", &self.entry)
            .finish()
    }
}

pub struct PipeOutlet<T> {
    loc: RefCell<usize>,
    buf: Rc<PipeBuf<T>>,
}
impl<T> Debug for PipeOutlet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PipeOutlet")
            .field("loc", &self.loc)
            .field("buf", &self.buf)
            .finish()
    }
}

impl<T> PipeInlet<T> {
    pub fn send(&self, t: T) {
        self.entry.pass(t);
    }
}

impl<T> PipeOutlet<T> {
    pub fn recv(&self) -> Option<Ref<T>> {
        let r = self.buf.get(*self.loc.borrow());
        if r.is_some() {
            *self.loc.borrow_mut() += 1;
        }
        r
    }
    pub fn is_recvable(&self) -> bool {
        self.buf.is_gettable(*self.loc.borrow())
    }
    pub fn is_alive(&self) -> bool {
        self.buf.is_active() || self.buf.is_gettable(*self.loc.borrow())
    }
}

// pub trait PipeSender<P> {
//     fn send_from(&self, e: P);
// }

// pub trait PipeReceiveState {
//     fn is_recvable(&self) -> bool;
//     fn is_alive(&self) -> bool;
// }
// pub trait PipeReceiver<P>: PipeReceiveState {
//     fn recv_into(&self) -> Option<Ref<P>>;
// }

// impl<P: Into<T>, T> PipeSender<P> for PipeInlet<T> {
//     fn send_from(&self, e: P) {
//         self.entry.pass(e.into())
//     }
// }

// impl<T> PipeReceiveState for PipeOutlet<T> {
//     fn is_recvable(&self) -> bool {
//         todo!()
//     }

//     fn is_alive(&self) -> bool {
//         todo!()
//     }
// }

// impl<P, T: Into<P>> PipeReceiver<P> for PipeOutlet<T> {
//     fn recv_into(&self) -> Option<Ref<P>> {
//         todo!()
//     }
// }

pub struct PipeStem<T> {
    entry: Rc<PipeEntry<T>>,
    buf: Rc<PipeBuf<T>>,
}

impl<T> PipeStem<T> {
    pub fn new() -> Self {
        let buf = Rc::new(PipeBuf {
            entry: OnceCell::new(),
            queue: RefCell::new(Vec::new()),
        });
        let entry = Rc::new(PipeEntry { buf: buf.clone() });
        buf.entry.set(Rc::downgrade(&entry)).unwrap();
        Self { entry, buf }
    }
    pub fn require_inlet(&self) -> PipeInlet<T> {
        PipeInlet {
            entry: self.entry.clone(),
        }
    }
    pub fn require_outlet(&self) -> PipeOutlet<T> {
        PipeOutlet {
            loc: RefCell::new(0),
            buf: self.buf.clone(),
        }
    }
}

// pub fn new<T>() -> (PipeInlet<T>, PipeOutlet<T>) {
//     let buf = Rc::new(PipeBuf {
//         entry: OnceCell::new(),
//         queue: RefCell::new(Vec::new()),
//     });
//     let entry = Rc::new(PipeEntry { buf: buf.clone() });
//     buf.entry.set(Rc::downgrade(&entry)).unwrap();
//     let s = PipeInlet { entry };
//     let r = PipeOutlet {
//         loc: RefCell::new(0),
//         buf,
//     };
//     (s, r)
// }

// pub trait PipeConnecter {
//     fn require_sender<P>(&self) -> anyhow::Result<PipeInlet<P>>;
//     fn require_receiver<P>(&self) -> anyhow::Result<PipeOutlet<P>>;
// }

// impl<T> PipeConnecter for PipeBuilder<T> {
//     fn require_sender<P>(&self) -> anyhow::Result<PipeSender<P>> {
//         let php = PhantomData::<P>;
//         let
//         //let a=Any::new(PhantomData::<P>);

//         Ok(PipeSender { entry: self.entry })
//     }

//     fn require_receiver<P>(&self) -> anyhow::Result<PipeReceiver<P>> {
//         todo!()
//     }
// }

// pub trait PSAny {}
// pub trait PRAny {}
// impl<T> PSAny for PipeSender<T> {}
// impl<T> PRAny for PipeReceiver<T> {}

#[cfg(test)]
mod tests {
    use std::{any::Any, fmt::Display, marker::PhantomData};

    use crate::pipe::pipe::PipeStem;

    #[test]
    fn test() {
        println!("hello");

        let b = PipeStem::<u64>::new();

        let s = b.require_inlet();
        let r = b.require_outlet();
        //println!("{:?} {:?}", s, r);
        let ss = b.require_inlet();
        let rr = b.require_outlet();

        drop(b);
        println!("{:?} {:?} {:?} {:?}", s, r, ss, rr);

        s.send(10);

        drop(s);
        println!("{}", r.is_alive());
        println!("{}", rr.is_alive());
        drop(ss);
        rr.recv();
        println!("{}", r.is_alive());
        println!("{}", rr.is_alive());
        println!("{:?} {:?}", r, rr);
    }

    fn convert<'a, T: ?Sized, Q: ?Sized>(t: &'a T) -> &'a Q
    where
        &'a Q: From<&'a T>,
    {
        t.into()
    }

    #[test]
    fn dyntest() {
        let x = PhantomData::<String>;

        let s: String = "aaa".to_string();
        let xx: String = s.into();

        let y: &dyn Any = convert(&x);

        println!("{:?}", y.downcast_ref::<PhantomData<dyn Display>>());

        // let a = TypeId::of::<dyn Display>();

        // let s: dyn Display = String::from("aaa") as dyn Display;
    }
}
