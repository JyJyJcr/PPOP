use std::{
    cell::{OnceCell, Ref, RefCell},
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct PipeEntry<T> {
    buf: Rc<PipeBuf<T>>,
}
#[derive(Debug)]
struct PipeBuf<T> {
    entry: OnceCell<Weak<PipeEntry<T>>>,
    queue: RefCell<Vec<T>>,
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
        self.entry.get().unwrap().upgrade().is_some()
    }
}

#[derive(Debug)]
pub struct PipeSender<T> {
    entry: Rc<PipeEntry<T>>,
}
impl<T> Clone for PipeSender<T> {
    fn clone(&self) -> Self {
        Self {
            entry: self.entry.clone(),
        }
    }
}
#[derive(Debug)]
pub struct PipeReceiver<T> {
    loc: RefCell<usize>,
    buf: Rc<PipeBuf<T>>,
}
impl<T> Clone for PipeReceiver<T> {
    fn clone(&self) -> Self {
        Self {
            loc: self.loc.clone(),
            buf: self.buf.clone(),
        }
    }
}
impl<T> PipeSender<T> {
    pub fn send(&self, e: T) {
        self.entry.pass(e);
    }
}
impl<T> PipeReceiver<T> {
    pub fn recv(&self) -> Option<Ref<T>> {
        let r = self.buf.get(*self.loc.borrow());
        if r.is_some() {
            *self.loc.borrow_mut() += 1;
        }
        r
    }
    pub fn is_recvable(&self) -> bool {
        let r = self.buf.get(*self.loc.borrow());
        r.is_some()
    }
    pub fn is_alive(&self) -> bool {
        self.buf.is_active() || self.buf.is_gettable(*self.loc.borrow())
    }
}

pub fn new<T>() -> (PipeSender<T>, PipeReceiver<T>) {
    let buf = Rc::new(PipeBuf {
        entry: OnceCell::new(),
        queue: RefCell::new(Vec::new()),
    });
    let entry = Rc::new(PipeEntry { buf: buf.clone() });
    buf.entry.set(Rc::downgrade(&entry)).unwrap();
    let s = PipeSender { entry };
    let r = PipeReceiver {
        loc: RefCell::new(0),
        buf,
    };
    (s, r)
}

// pub trait PSAny {}
// pub trait PRAny {}
// impl<T> PSAny for PipeSender<T> {}
// impl<T> PRAny for PipeReceiver<T> {}

#[cfg(test)]
mod tests {
    use crate::pipe::new;

    #[test]
    fn test() {
        println!("hello");
        let (s, r) = new::<u64>();
        println!("{:?} {:?}", s, r);
        let ss = s.clone();
        let rr = r.clone();
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
}
