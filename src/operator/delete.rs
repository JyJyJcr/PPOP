use std::{fmt::Debug, marker::PhantomData};

use anyhow::anyhow;

use crate::deduct::AgentPrecursor;

use super::{IBuildable, IOp};

pub struct Delete<D> {
    ph: PhantomData<D>,
}
impl<D> Debug for Delete<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Delete").field("ph", &self.ph).finish()
    }
}

impl<D: 'static> IOp for Delete<D> {
    type Input = D;

    type Output = ();

    fn new(_imm: String) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self { ph: PhantomData })
    }

    fn exec(&self, _e: &Self::Input) -> impl IntoIterator<Item = Self::Output> {
        [()]
    }
}

#[derive(Debug)]
pub struct DeletePrecursor {
    pi: String,
    po: String,
}
impl DeletePrecursor {
    pub fn new(pi: &str, po: &str) -> Self {
        let pi = pi.to_string();
        let po = po.to_string();
        Self { pi, po }
    }
}
impl AgentPrecursor<String> for DeletePrecursor {
    fn deduct(&self, idx: &mut crate::deduct::PipeTypeIndex<String>) -> anyhow::Result<()> {
        idx.require::<(), _>(&self.po)
    }

    fn build(
        self: Box<Self>,
        idx: &crate::deduct::PipeIndex<String>,
    ) -> anyhow::Result<Box<dyn crate::agent::Agent>> {
        if idx.ask_to_be::<usize, _>(&self.pi)? {
            //println!("ill");
            return Ok(Box::new(Delete::<usize>::build(
                self.pi,
                String::new(),
                self.po,
                idx,
            )?));
        }
        Err(anyhow!("delete is still WIP"))
    }
}
