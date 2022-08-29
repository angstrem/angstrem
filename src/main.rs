#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

// #[allow(dead_code)]

use std::env::current_dir;
use std::fs::File;
use std::io::BufReader;

mod parser;
use parser::parse;

const filename_input: &'static str = "fact.ng";

fn main ()
{
	let filename_base = current_dir().unwrap();
	let filename = filename_base.join(filename_input);

	let file = File::open(filename).unwrap();
	let input = BufReader::new(file);

	let node = parse(input);

	println!("{:#?}", node.nested);
}
