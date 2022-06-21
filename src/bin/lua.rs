//! The REPL used to test and run lua code from the command line.

use std::{env::args, process::ExitCode, io::{stdin, BufRead, Write}};

use luna::codegen::lex::LexerState;

fn main() -> ExitCode {
	let mut args = args();

	match args.len() {
		1 => repl(),
		2 => {
			args.next(); // skip program name
			let spath = args.next().expect("Expected script path");
			script(&spath)
		},
		_ => {
			eprintln!("Usage: {} [script]", args.next().expect("prog"));
			ExitCode::FAILURE
		}
	}
}

fn script(path: &str) -> ExitCode {
	println!("{path}");

	ExitCode::SUCCESS
}

fn repl() -> ExitCode {
	let mut io = stdin().lock();
	let mut line = String::new();

	loop {
		line.clear();

		print!(">> ");
		std::io::stdout().flush().unwrap();

		io.read_line(&mut line).expect("Couldn't read REPL line.");

		if line.starts_with("exit") { break }

		let mut lexer = LexerState::new(&line);
		println!("{:?}", lexer.scan());
	}

	ExitCode::SUCCESS
}
