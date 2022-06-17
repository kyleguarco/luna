//! The REPL used to test and run lua code from the command line.

use std::{env::args, process::ExitCode};

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
	loop {
		break
	}

	ExitCode::SUCCESS
}
