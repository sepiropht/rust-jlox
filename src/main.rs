use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
#[macro_use]
extern crate error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.len() {
        arg if arg < 1 => println!("Usage: jlox [script]"),
        arg if arg == 1 => run_file(&args[0])?,
        _ => run_prompt()?,
    };

    Ok(())
}

fn run_file(file: &String) -> Result<()> {
    let mut file = File::open(file)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    run(&content)?;

    Ok(())
}

fn run_prompt() -> Result<()> {
    loop {
        println!(">");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Somethin wrong");
        run(&input)?
    }
}

fn run(source: &String) -> Result<()> {
    for token in source.split(' ') {
        println!("{:?}", token);
    }
    Ok(())
}
