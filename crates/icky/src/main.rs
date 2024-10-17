use std::{env, fs, io, process::exit};

use icky::eval;

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("expected usage:\n  icky <FILE>");
        exit(1);
    }
    let source = fs::read_to_string(&args[1])?;
    println!("{:?}", eval(&source));
    Ok(())
}
