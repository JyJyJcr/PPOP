mod agent;
mod controller;
//mod op;
//mod op_imm;
mod cli_arg;
mod operator;
mod pipe;
mod pipe_board;

use std::{env::args, fs::read_to_string, path::PathBuf};

use anyhow::{anyhow, Context};
use cli_arg::CliArgAgent;
use controller::Builder;
use unicode_segmentation::UnicodeSegmentation;

fn main() -> anyhow::Result<()> {
    let it = args();
    let mut it = it.skip(1);
    let script: PathBuf = it
        .next()
        .ok_or(anyhow!("script file not specified"))?
        .into();
    let args: Vec<String> = it.collect();
    //println!("exec {}", script.to_str().unwrap());

    let s = read_to_string(script).context(anyhow!("failed to read script"))?;
    let gv = s.graphemes(true).collect::<Vec<&str>>();
    if gv.len() % 4 != 0 {
        return Err(anyhow!("script alignment invalid"));
    }

    let mut builder = Builder::new();

    builder.register(|board| CliArgAgent::build(args, board));

    for i in 0..gv.len() / 4 {
        //parse
        let li1 = gv[4 * i];
        let li2 = gv[4 * i + 1];
        let lop = gv[4 * i + 2];
        let lo = gv[4 * i + 3];

        builder.compile(li1, li2, lop, lo);
    }

    let mut executor = builder.build();

    //let mut t: usize = 0;

    while {
        //println!("t={}", t);
        //println!("{:?}", executor);
        executor.step()
    } {
        //t += 1;
    }
    Ok(())
}
