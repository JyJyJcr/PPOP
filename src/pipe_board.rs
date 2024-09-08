use crate::pipe::{new, PipeReceiver, PipeSender};
use std::hash::Hash;
use std::{any::Any, collections::HashMap};

pub struct PipeBoard<K: Eq + Hash> {
    dic: HashMap<K, (Box<dyn Any>, Box<dyn Any>)>,
}
impl<K: Eq + Hash + Clone> PipeBoard<K> {
    pub fn new() -> Self {
        Self {
            dic: HashMap::new(),
        }
    }
    pub fn get<T: 'static>(&mut self, key: K) -> (PipeSender<T>, PipeReceiver<T>) {
        match self.dic.get(&key) {
            Some((ps, pr)) => {
                let ps = ps.downcast_ref::<PipeSender<T>>().unwrap();
                let pr = pr.downcast_ref::<PipeReceiver<T>>().unwrap();
                (ps.clone(), pr.clone())
            }
            None => {
                let (ps, pr) = new::<T>();
                self.dic
                    .insert(key, (Box::new(ps.clone()), Box::new(pr.clone())));
                (ps, pr)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pipe_board::PipeBoard;

    #[test]
    fn test() {
        let mut bo = PipeBoard::<String>::new();
        let (a, b) = bo.get::<i64>(String::from("aaa"));
        println!("{:?} -> {:?}", a, b);
        a.send(10);
        let (a, b) = bo.get::<i64>(String::from("aaa"));
        println!("{:?} -> {:?}", a, b);
    }
}
