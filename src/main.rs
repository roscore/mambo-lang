use mambo_lang::{execute, parse, Limits};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::ExitCode;

const HELP: &str = "Mambo Lang (맘보랭) 0.1.0\n\nUsage:\n  mambo run <file>    Execute a UTF-8 source file\n  mambo eval <source> Execute source text\n  mambo check <file>  Parse and validate a source file\n  mambo help          Show this help\n  mambo version       Show version\n";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match command(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err((code, message)) => {
            eprintln!("mambo: {message}");
            ExitCode::from(code)
        }
    }
}

fn command(args: &[String]) -> Result<(), (u8, String)> {
    match args {
        [command] if command == "help" || command == "--help" || command == "-h" => {
            print!("{HELP}");
            Ok(())
        }
        [command] if command == "version" || command == "--version" || command == "-V" => {
            println!("mambo {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        [command, source] if command == "eval" => run_source(source),
        [command, file] if command == "run" => {
            let source = read(file)?;
            run_source(&source)
        }
        [command, file] if command == "check" => {
            let source = read(file)?;
            parse(&source).map_err(|error| (1, error.to_string()))?;
            println!("OK: {file}");
            Ok(())
        }
        _ => Err((2, format!("invalid arguments\n{HELP}"))),
    }
}

fn read(file: &str) -> Result<String, (u8, String)> {
    fs::read_to_string(file).map_err(|error| (1, format!("{file}: {error}")))
}

fn run_source(source: &str) -> Result<(), (u8, String)> {
    let program = parse(source).map_err(|error| (1, error.to_string()))?;
    let output = execute(&program, Limits::default()).map_err(|error| (1, error.to_string()))?;
    io::stdout()
        .write_all(output.as_bytes())
        .map_err(|error| (1, error.to_string()))
}
