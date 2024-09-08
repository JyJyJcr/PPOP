use std::cell::RefCell;

use crate::{agent::Agent, pipe::PipeSender, pipe_board::PipeBoard};

#[derive(Debug)]
pub struct CliArgAgent {
    argc: usize,
    argv: Vec<String>,
    pc: PipeSender<usize>,
    pv: PipeSender<String>,
    loc: RefCell<usize>,
}
impl CliArgAgent {
    pub fn build(args: Vec<String>, board: &mut PipeBoard<String>) -> Self {
        let (pc, _) = board.get(String::from("#"));
        let (pv, _) = board.get(String::from("@"));
        Self {
            argc: args.len(),
            argv: args,
            pc,
            pv,
            loc: RefCell::new(0),
        }
    }
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
