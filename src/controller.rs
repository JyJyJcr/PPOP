use crate::{
    agent::Agent,
    operator::{add::SSAdd, replace::LsReplace, stdio::SxPrintf, IOp, YOp},
    pipe_board::PipeBoard,
};

pub struct Builder {
    board: PipeBoard<String>,
    agents: Vec<Box<dyn Agent>>,
}
impl Builder {
    pub fn new() -> Self {
        Self {
            board: PipeBoard::new(),
            agents: Vec::new(),
        }
    }

    pub fn compile(&mut self, li1: &str, li2: &str, op: &str, lo: &str) {
        self.agents.push(match op {
            "+" => Box::new(SSAdd::build(li1, li2, lo, &mut self.board)),
            "S" => Box::new(LsReplace::build(li1, li2, lo, &mut self.board)),
            "P" => Box::new(SxPrintf::build(li1, li2, lo, &mut self.board)),
            _ => return,
        });
    }
    pub fn register<A: Agent + 'static, F: FnOnce(&mut PipeBoard<String>) -> A>(&mut self, f: F) {
        self.agents.push(Box::new(f(&mut self.board)))
    }

    pub fn build(self) -> Executor {
        Executor {
            agents: self.agents,
        }
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
