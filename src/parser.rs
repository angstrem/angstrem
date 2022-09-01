
use std::io::BufRead;

use nom::IResult;
// use nom::error::Error;
use nom::error::ParseError;

use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::character::complete::alpha1;
use nom::character::complete::alphanumeric1;
use nom::character::complete::digit1;

use nom::bytes::complete::tag;

use nom::sequence::preceded;
use nom::sequence::delimited;
use nom::sequence::terminated;

use nom::sequence::pair;
use nom::multi::many0;
use nom::multi::separated_list0;

use nom::branch::alt;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::combinator::map;


#[derive(Debug)]
enum Node
{
	String(String),
	Number(i32),
	List(Vec<Node>),
}

type Result <'S> = IResult<&'S str, Node>;

pub fn parse (_input: impl BufRead) -> ()
{
	let foo = "a_1bcd_d (b1 c2) 123 _d (x 1";

	println!("{:#?}\n", foo);
	println!("{:#?}", p_list_naked(foo));
}

fn p_form (input: &str) -> Result
{
	let p = alt((
		p_list,
		p_id,
		p_num,
	));
	let mut p = p;

	p(input)
}

fn p_list (input: &str) -> Result
{
	let p = preceded
	(
		tag("("),
		ws(p_list_naked),
	);
	let p = terminated(p, opt(tag(")")));
	let mut p = p;

	p(input)
}

fn p_list_naked (input: &str) -> Result
{
	let p = separated_list0(space1, p_form);
	let mut p = map(p, Node::List);

	p(input)
}

fn p_id (input: &str) -> Result
{
	let p = pair
	(
		alt((alpha1, tag("_"))),
		many0(alt((alphanumeric1, tag("_")))),
	);
	let p = recognize(p);
	let mut p = map(p, |s: &str| Node::String(s.into()));

	p(input)
}

fn p_num (input: &str) -> Result
{
	let p = digit1;
	let p = recognize(p);
	let p = map(p, |s: &str| Node::Number(s.parse().unwrap())); // map_res
	let mut p = p;

	p(input)
}

fn ws <'a, F: 'a, O, E> (inner: F)
	-> impl FnMut(&'a str) -> IResult<&'a str, O, E>
	where
	E: ParseError<&'a str>,
	F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
	delimited
	(
		space0,
		inner,
		space0,
	)
}
