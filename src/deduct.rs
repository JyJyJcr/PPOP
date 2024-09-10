use std::{any::TypeId, borrow::Borrow, collections::HashMap, fmt::Debug};

use anyhow::{anyhow, Context};

use crate::{
    agent::Agent,
    pipe::{PipeBuilder, PipeReceiver, PipeSender},
};
use std::hash::Hash;

#[derive(Debug)]
pub struct PipeTypeIndex<K> {
    concrete_count: usize,
    idx: HashMap<K, TypeId>,
}

impl<K> PipeTypeIndex<K> {
    pub fn new() -> Self {
        Self {
            concrete_count: 0,
            idx: HashMap::new(),
        }
    }
    pub fn ask<Q>(&self, k: &Q) -> Option<&TypeId>
    where
        Q: ?Sized + Hash + Eq,
        K: Borrow<Q> + Hash + Eq,
    {
        self.idx.get(k)
    }

    pub fn require_by_type_id<Q>(&mut self, k: &Q, type_id: TypeId) -> anyhow::Result<()>
    where
        Q: ?Sized + Hash + Eq + ToOwned<Owned = K>,
        K: Borrow<Q> + Hash + Eq,
    {
        match self.idx.get(k) {
            Some(ty) => {
                if *ty != type_id {
                    return Err(anyhow!("pipe type requirement conflict"));
                }
            }
            None => {
                self.idx.insert(k.to_owned(), type_id);
                self.concrete_count += 1;
            }
        }
        Ok(())
    }

    pub fn require<T: 'static, Q>(&mut self, k: &Q) -> anyhow::Result<()>
    where
        Q: ?Sized + Hash + Eq + ToOwned<Owned = K>,
        K: Borrow<Q> + Hash + Eq,
    {
        self.require_by_type_id(k, TypeId::of::<T>())
    }

    pub fn concrete_count(&self) -> usize {
        self.concrete_count
    }
    pub fn generate(self) -> PipeIndex<K>
    where
        K: Eq + Hash,
    {
        let idx: HashMap<K, PipeBuilder> = self
            .idx
            .into_iter()
            .map(|(k, ty)| (k, PipeBuilder::new_by_type_id(ty)))
            .collect();
        PipeIndex { idx }
    }
}

pub struct PipeIndex<K> {
    idx: HashMap<K, PipeBuilder>,
}

impl<K> PipeIndex<K> {
    pub fn ask_to_be<T: 'static, Q>(&self, k: &Q) -> anyhow::Result<bool>
    where
        Q: ?Sized + Hash + Eq,
        K: Borrow<Q> + Hash + Eq,
    {
        Ok(self
            .idx
            .get(k)
            .context("pipe not implemented")?
            .ask_type_id()
            == TypeId::of::<T>())
    }

    pub fn ask<Q>(&self, k: &Q) -> anyhow::Result<TypeId>
    where
        Q: ?Sized + Hash + Eq,
        K: Borrow<Q> + Hash + Eq,
    {
        Ok(self
            .idx
            .get(k)
            .context("pipe not implemented")?
            .ask_type_id())
    }

    pub fn require_sender<T: 'static, Q>(&self, k: &Q) -> anyhow::Result<PipeSender<T>>
    where
        Q: ?Sized + Hash + Eq,
        K: Borrow<Q> + Hash + Eq,
    {
        self.idx
            .get(k)
            .context("pipe not implemented")?
            .require_sender()
    }
    pub fn require_receiver<T: 'static, Q>(&self, k: &Q) -> anyhow::Result<PipeReceiver<T>>
    where
        Q: ?Sized + Hash + Eq,
        K: Borrow<Q> + Hash + Eq,
    {
        self.idx
            .get(k)
            .context("pipe not implemented")?
            .require_receiver()
    }
}

pub trait AgentPrecursor<K>: Debug {
    fn deduct(&self, idx: &mut PipeTypeIndex<K>) -> anyhow::Result<()>;
    fn build(self: Box<Self>, idx: &PipeIndex<K>) -> anyhow::Result<Box<dyn Agent>>;
}
