use anyhow::anyhow;
use std::{env, fs, process::exit, str::FromStr};

use icky::{eval, parse, Context};

#[derive(Debug)]
enum Mode {
    Parse,
    Run,
}

impl FromStr for Mode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "parse" => Ok(Self::Parse),
            "run" => Ok(Self::Run),
            _ => Err(anyhow!("unexpected mode: {}", s)),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        println!("expected usage:\n  icky <MODE> <FILE>");
        exit(1);
    }
    let mode = Mode::from_str(&args[1])?;
    let source = fs::read_to_string(&args[2])?;
    match mode {
        Mode::Parse => {
            let ctx = Context::new(&source);
            parse(&source)?
                .pretty(&ctx)
                .print(0, &mut std::io::stdout())?;
        }
        Mode::Run => {
            println!("{:#?}", eval(&source)?);
        }
    }
    Ok(())
}
