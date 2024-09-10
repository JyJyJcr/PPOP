use std::cell::RefCell;

use crate::{agent::Agent, deduct::AgentPrecursor, pipe::PipeSender};

#[derive(Debug)]
pub struct CliArgAgent {
    argc: usize,
    argv: Vec<String>,
    pc: PipeSender<usize>,
    pv: PipeSender<String>,
    loc: RefCell<usize>,
}

impl Agent for CliArgAgent {
    fn step(&self) -> bool {
        let loc = *self.loc.borrow();
        if loc == 0 {
            self.pc.send(self.argc);
        }
        if loc < self.argc {
            self.pv.send(self.argv[loc].clone());
            *self.loc.borrow_mut() += 1;
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct CliArgAgentPrecursor {
    args: Vec<String>,
}
impl CliArgAgentPrecursor {
    pub fn new(args: Vec<String>) -> Self {
        Self { args }
    }
}

impl AgentPrecursor<String> for CliArgAgentPrecursor {
    fn deduct(&self, idx: &mut crate::deduct::PipeTypeIndex<String>) -> anyhow::Result<()> {
        idx.require::<usize, _>("#")?;
        idx.require::<String, _>("@")?;
        Ok(())
    }

    fn build(
        self: Box<Self>,
        idx: &crate::deduct::PipeIndex<String>,
    ) -> anyhow::Result<Box<dyn Agent>> {
        let pc = idx.require_sender("#")?;
        let pv = idx.require_sender("@")?;

        Ok(Box::new(CliArgAgent {
            argc: self.args.len(),
            argv: self.args,
            pc,
            pv,
            loc: RefCell::new(0),
        }))
    }
}
