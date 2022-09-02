// TODO: ? line trailing backslash
// TODO: ? modes? lisp compat

use std::io::BufRead;

use nom::IResult;
use nom::error::ParseError;
// use nom::error::Error;
// use nom::Err;

use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::character::complete::alpha1;
use nom::character::complete::alphanumeric1;
use nom::character::complete::digit1;
use nom::character::complete::one_of;

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
use nom::combinator::rest;
use nom::combinator::all_consuming;

mod list;

#[derive(Debug)]
enum Literal
{
	Number(i32),
	// String(String),
	// Boolean(bool),
}

#[derive(Debug)]
enum Form
{
	Literal(Literal),
	Id(String),
}

#[derive(Debug)]
struct Line
{
	depth: usize,
	list: List,
}

type List = list::List<Form>;

type ResultOf <'S, T> = IResult<&'S str, T>;
type Result <'S> = ResultOf<'S, List>;

pub fn parse (input: impl BufRead) -> ()
{
	let lines = input
	.lines()
	.map(|line| line.unwrap())
	.filter(|line| line.len() > 0)
	.map(p_line);
	// .collect();
	// .fold(parse_node_context, parse_node);

	// let foo = "\t\t A1 1 (B1? B2!) C-1 (D1 2 ; abc def".to_string();
	// println!("{:#?}\n", foo);
	// println!("{:#?}", p_line(foo).unwrap());

	for line in lines
	{
		println!("{:#?}", line);
	}

	// println!("{},\n {:#?}", lines.len(), lines);
}

fn p_line (input: String) -> std::result::Result<Line, ()>
{
	let p = pair
	(
		p_indent,
		p_list_naked,
	);
	let p = terminated(p, opt(p_comment));
	let p = all_consuming(p);
	let p = map(p, |(depth, list)| Line { depth, list });
	let mut p = p;

	match p(&input)
	{
		Err(_) => Err(()), // TODO: Err
		Ok((_, line)) => Ok(line),
	}
}

fn p_indent (input: &str) -> ResultOf<usize>
{
	let p = space0;
	let p = map(p, |s: &str| s.len());
	let mut p = p;

	p(input)
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

fn p_comment (input: &str) -> ResultOf<()>
{
	let p = pair(tag(";"), rest);
	let p = map(p, |_| ());
	let mut p = p;

	p(input)
}

fn p_list_naked (input: &str) -> Result
{
	let p = separated_list0(space1, p_form);
	let mut p = map(p, List::from_vec);

	p(input)
}

fn p_id (input: &str) -> Result
{
	let whitelist = "_-~!@#$%&?*+=<>";
	let p = pair
	(
		alt((alpha1, recognize(one_of(whitelist)))),
		many0(alt((alphanumeric1, recognize(one_of(whitelist))))),
	);
	let p = recognize(p);
	let mut p = map(p, |s: &str| List::Leaf(Form::Id(s.into())));

	p(input)
}

fn p_num (input: &str) -> Result
{
	let p = digit1;
	let p = recognize(p);
	let p = map(p, |s: &str|
	{
		let n = Literal::Number(s.parse().unwrap()); // map_res
		let n = Form::Literal(n);
		let n = List::Leaf(n);
		n
	});
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
