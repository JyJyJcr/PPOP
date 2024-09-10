use anyhow::Context;

use crate::pipe::{new, PipeInlet, PipeOutlet};
use std::any::TypeId;
use std::borrow::Borrow;
use std::fmt::Debug;
use std::hash::Hash;
use std::{any::Any, collections::HashMap};
pub struct PipeBoard<K: Eq + Hash> {
    dic: HashMap<K, (TypeId, Box<dyn Any>, Box<dyn Any>)>,
}
impl<K: Eq + Hash + Clone + Debug> PipeBoard<K> {
    pub fn new() -> Self {
        Self {
            dic: HashMap::new(),
        }
    }
    pub fn get_type<Q: ?Sized + Debug + Hash + Eq>(&self, key: &Q) -> Option<TypeId>
    where
        K: Borrow<Q>,
    {
        self.dic.get(key.borrow()).map(|(t, _, _)| t.clone())
    }

    pub fn require_receiver<T: 'static, Q: ?Sized + Debug + Hash + Eq + ToOwned<Owned = K>>(
        &mut self,
        key: &Q,
    ) -> anyhow::Result<PipeOutlet<T>>
    where
        K: Borrow<Q>,
    {
        match self.dic.get(key) {
            Some((_, _, pr)) => {
                let pr = pr.downcast_ref::<PipeOutlet<T>>().context(format!(
                    "pipe {:?} is already prepared for different type",
                    key.borrow()
                ))?;
                Ok(pr.clone())
            }
            None => {
                let (ps, pr) = new::<T>();
                self.dic.insert(
                    key.to_owned(),
                    (TypeId::of::<T>(), Box::new(ps), Box::new(pr.clone())),
                );
                Ok(pr)
            }
        }
    }

    pub fn require_sender<T: 'static, Q: ?Sized + Debug + Hash + Eq + ToOwned<Owned = K>>(
        &mut self,
        key: &Q,
    ) -> anyhow::Result<PipeInlet<T>>
    where
        K: Borrow<Q>,
    {
        match self.dic.get(key) {
            Some((_, ps, _)) => {
                let ps = ps.downcast_ref::<PipeInlet<T>>().context(format!(
                    "pipe {:?} is already prepared for different type",
                    key.borrow()
                ))?;
                Ok(ps.clone())
            }
            None => {
                let (ps, pr) = new::<T>();
                self.dic.insert(
                    key.to_owned(),
                    (TypeId::of::<T>(), Box::new(ps.clone()), Box::new(pr)),
                );
                Ok(ps)
            }
        }
    }

    // pub fn require<T: 'static, Q: ?Sized + Debug + Hash + Eq + ToOwned<Owned = K>>(
    //     &mut self,
    //     key: &Q,
    // ) -> anyhow::Result<(PipeSender<T>, PipeReceiver<T>)>
    // where
    //     K: Borrow<Q>,
    // {
    //     match self.dic.get(key) {
    //         Some((_, ps, pr)) => {
    //             let ps = ps.downcast_ref::<PipeSender<T>>().context(format!(
    //                 "pipe {:?} is already prepared for different type",
    //                 key.borrow()
    //             ))?;
    //             let pr = pr.downcast_ref::<PipeReceiver<T>>().context(format!(
    //                 "pipe {:?} is already prepared for different type",
    //                 key.borrow()
    //             ))?;
    //             Ok((ps.clone(), pr.clone()))
    //         }
    //         None => {
    //             let (ps, pr) = new::<T>();
    //             self.dic.insert(
    //                 key.to_owned(),
    //                 (
    //                     TypeId::of::<T>(),
    //                     Box::new(ps.clone()),
    //                     Box::new(pr.clone()),
    //                 ),
    //             );
    //             Ok((ps, pr))
    //         }
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use crate::{pipe::PipeSender, pipe_board::PipeBoard};

    #[test]
    fn test() -> anyhow::Result<()> {
        let mut bo = PipeBoard::<String>::new();
        let a = bo.require_sender::<i64, _>("aaa")?;
        a.send_from(10);
        let b = bo.require_receiver::<i64, _>("aaa")?;
        println!("{:?} -> {:?}", a, b);
        Ok(())
    }
}
