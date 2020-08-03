pub mod ast;
mod scanner;
pub mod token;
mod token_type;

use exit::Exit;
use io::{BufRead, BufReader, Read, Write};
use lazy_static::lazy_static;
use parking_lot::RwLock;
use scanner::Scanner;
use std::{env, error::Error, fs::File, io};

lazy_static! {
    static ref HAD_ERROR: RwLock<bool> = RwLock::new(false);
}

fn main() -> Exit<i8> {
    let args: Vec<_> = env::args().skip(1).collect();

    match args.len() {
        0 => run_prompt().map(|_| Exit::Ok).map_err(|_| 1)?,
        1 => run_file(&args[0]).map_err(|_| 1).map(|_| Exit::Ok)?,
        _ => {
            eprintln!("Usage: jlox [script]");
            Exit::Err(64)
        }
    }
}

fn run_file(path: &str) -> Result<(), i8> {
    let mut bytes = vec![];
    (|| -> Result<(), Box<dyn Error>> {
        File::open(path)?.read_to_end(&mut bytes)?;
        run(String::from_utf8(bytes)?)?;
        Ok(())
    })()
    .map_err(|_| 1)?;

    if *HAD_ERROR.read() {
        return Err(65);
    }
    Ok(())
}

fn run_prompt() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            break;
        }

        run(line)?;

        *HAD_ERROR.write() = false;
    }
    Ok(())
}

fn run<S>(source: S) -> Result<(), Box<dyn Error>>
where
    S: AsRef<str>,
{
    let source = source.as_ref();

    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token.to_string());
    }
    Ok(())
}

fn error<S>(line: usize, message: S)
where
    S: AsRef<str>,
{
    report(line, "", message.as_ref())
}

fn report<S, S2>(line: usize, whence: S, message: S2)
where
    S: AsRef<str>,
    S2: AsRef<str>,
{
    eprintln!(
        "[line {}] Error{}: {}",
        line,
        whence.as_ref(),
        message.as_ref()
    );
    *HAD_ERROR.write() = true;
}
