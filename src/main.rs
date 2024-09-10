mod agent;
mod controller;
//mod type_enum;
//mod op;
//mod op_imm;
mod cli_arg;
mod deduct;
mod operator;
mod pipe;
//mod pipe_board;

use std::{env::args, fs::read_to_string, path::PathBuf};

use anyhow::{anyhow, Context};
use cli_arg::CliArgAgentPrecursor;
use controller::Builder;
use operator::{add, delete::DeletePrecursor, load::Load, stdio::SxPrintf, IPrecursor};
use unicode_segmentation::UnicodeSegmentation;

struct Param {
    script: PathBuf,
}

fn parse_args() -> anyhow::Result<(Param, Vec<String>)> {
    let it = args();
    let mut it = it.skip(1);
    let script: PathBuf = it
        .next()
        .ok_or(anyhow!("script file not specified"))?
        .into();
    let args: Vec<String> = it.collect();
    Ok((Param { script }, args))
}

fn main() -> anyhow::Result<()> {
    let (param, args) = parse_args()?;

    //println!("exec {}", script.to_str().unwrap());

    let s = read_to_string(param.script).context(anyhow!("failed to read script"))?;
    let gv = s.graphemes(true).collect::<Vec<&str>>();
    if gv.len() % 4 != 0 {
        return Err(anyhow!("script alignment invalid"));
    }

    let mut builder = Builder::new();

    builder.put(Box::new(CliArgAgentPrecursor::new(args)));

    for i in 0..gv.len() / 4 {
        //parse
        let li1 = gv[4 * i];
        let li2 = gv[4 * i + 1];
        let lop = gv[4 * i + 2];
        let lo = gv[4 * i + 3];

        builder.put(match lop {
            "+" | "加" => add::precursor(li1, li2, lo)?,
            "S" | "字" => IPrecursor::<Load<String>>::new(li1, li2, lo),
            "~" => Box::new(DeletePrecursor::new(li1, lo)),
            // "x" => replace::build::<u8>(li1, li2, lo, &mut self.board)?,
            // "i" | "整" => replace::build::<i64>(li1, li2, lo, &mut self.board)?,
            // "u" => replace::build::<u64>(li1, li2, lo, &mut self.board)?,
            // "f" => replace::build::<f64>(li1, li2, lo, &mut self.board)?,
            // "U" => replace::build::<usize>(li1, li2, lo, &mut self.board)?,
            // "I" => replace::build::<isize>(li1, li2, lo, &mut self.board)?,
            "P" | "印" => IPrecursor::<SxPrintf>::new(li1, li2, lo),
            _ => return Err(anyhow!("operator {} is not registered", lop)),
        });
    }

    // println!("builder: {:?}", builder);

    // println!("deduct");

    builder.deduct()?;

    //builder.deduct()?;

    // println!("builder: {:?}", builder);

    // println!("build");

    let mut executor = builder.build()?;

    // println!("exe");

    // println!("{:?}", executor);

    let mut t: usize = 0;

    while {
        //println!("t={}", t);
        //println!("{:?}", executor);
        executor.step() && t < 5
    } {
        t += 1;
    }

    // println!("{:?}", executor);

    Ok(())
}
