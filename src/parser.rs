
use std::io::BufRead;

pub fn parse (input: impl BufRead) -> impl Iterator<Item = Depth>
{
	input
	.lines()
	.map(|line| line.unwrap())
	.map(parse_depth)
}


#[derive(Debug)]
pub struct Depth
{
	pub depth: usize,
	pub value: String,
}

fn parse_depth (input: String) -> Depth
{
	let depth = input
	.chars()
	.take_while(|ch| ch.is_ascii_whitespace())
	.count();

	let value = input
	.chars()
	.skip(depth)
	.collect();

	Depth { depth, value }
}
