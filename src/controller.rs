use crate::{
    agent::Agent,
    deduct::{AgentPrecursor, PipeTypeIndex},
};
use std::hash::Hash;

#[derive(Debug)]
pub struct Builder<K> {
    idx: PipeTypeIndex<K>,
    precursors: Vec<Box<dyn AgentPrecursor<K>>>,
}
impl<K> Builder<K> {
    pub fn new() -> Self {
        Self {
            idx: PipeTypeIndex::new(),
            precursors: Vec::new(),
        }
    }

    pub fn put(&mut self, precursor: Box<dyn AgentPrecursor<K>>) {
        self.precursors.push(precursor);
    }

    pub fn deduct_once(&mut self) -> anyhow::Result<bool> {
        let count_pre = self.idx.concrete_count();
        for precursor in self.precursors.iter_mut() {
            let precursor = precursor.as_mut();
            precursor.deduct(&mut self.idx)?;
        }
        let count_post = self.idx.concrete_count();
        Ok(count_post > count_pre)
    }

    pub fn deduct(&mut self) -> anyhow::Result<()> {
        while self.deduct_once()? {}
        Ok(())
    }

    pub fn build(self) -> anyhow::Result<Executor>
    where
        K: Eq + Hash,
    {
        let idx = self.idx.generate();

        let agents: anyhow::Result<Vec<Box<dyn Agent>>> = self
            .precursors
            .into_iter()
            .map(|p| {
                let a = p.build(&idx);
                a
            })
            .collect();
        let agents = agents?;

        Ok(Executor { agents })
    }
}

#[derive(Debug)]
pub struct Executor {
    agents: Vec<Box<dyn Agent>>,
}
impl Executor {
    pub fn step(&mut self) -> bool {
        self.agents.retain(|agent| agent.step());
        !self.agents.is_empty()
    }
}
