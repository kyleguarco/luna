//! The REPL used to test and run lua code from the command line.

use std::{
	env::args,
	fs::read_to_string,
	io::{stdin, BufRead, Write},
};

use luna_parser::chunk;

fn main() {
	let mut args = args();

	match args.len() {
		1 => repl(),
		2 => {
			args.next(); // skip program name
			let spath = args.next().expect("Expected script path");
			script(&spath)
		}
		_ => {
			eprintln!("Usage: {} [script]", args.next().expect("prog"));
		}
	}
}

fn script(path: &str) {
	let data = read_to_string(path).expect("Couldn't open the script (Does it exist?)");
	let chunk = chunk(&data);
	println!("{chunk:?}")
}

fn repl() {
	let mut io = stdin().lock();
	let mut line = String::new();

	loop {
		line.clear();

		print!(">> ");
		std::io::stdout().flush().unwrap();

		io.read_line(&mut line).expect("Couldn't read REPL line.");

		if line.starts_with("exit") {
			break;
		}

		let chunk = chunk(&line);
		println!("{chunk:?}")
	}
}
