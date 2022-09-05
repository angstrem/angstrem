// TODO: ? line trailing backslash
// TODO: ? modes? lisp compat
// TODO: multiline comment
// TODO: Edge-[1] Edge/Leaf elim
// TODO: empty Edge // if (next.list.is_empty()) { continue }

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
use nom::sequence::tuple;
use nom::multi::many0;
use nom::multi::separated_list0;

use nom::branch::alt;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::combinator::map;
use nom::combinator::rest;
use nom::combinator::all_consuming;

mod list;

mod stack;
use stack::Stack;

#[derive(Debug)]
pub enum Literal
{
	Number(i32),
	// String(String),
	// Boolean(bool),
}

#[derive(Debug)]
pub enum Form
{
	Literal(Literal),
	Id(String),
}

#[derive(Debug)]
struct Line
{
	line_no: usize,
	depth: usize,
	list: List,
	comment: String,
}

pub type List = list::List<Form>;

type ResultOf <'S, T> = IResult<&'S str, T>;
type Result <'S> = ResultOf<'S, List>;

pub fn parse (input: impl BufRead)
 -> std::result::Result<List, nom::Err<nom::error::Error<usize>>>
{
	let lines = input
	.lines()
	.enumerate()
	.map(|(n, line)| (n, line.unwrap()))
	.filter(|(_, line)| line.len() > 0)
	.map(p_line);

	#[derive(Debug)]
	struct Frame <'L>
	{
		depth: usize,
		list: &'L mut List,
	}

	let mut root = List::root();
	let mut prev = None as Option<&mut List>;
	let mut stack = Stack::new(Frame { depth: 0, list: &mut root });

	for next in lines
	{
		let next = next?;

		if (next.list.is_edge_empty())
		{
			continue
		}

		/*
			>>>
		*/
		if (next.depth > stack.head().depth)
		{
			match prev.take()
			{
				None => {},
				Some(prev) =>
				{
					stack.push(Frame { depth: next.depth, list: prev });
				}
			}
		}
		/*
			<<<
		*/
		else
		{
			while (next.depth < stack.head().depth)
			{
				stack.pop()
			}

			assert!(next.depth == stack.head().depth, "incorrect_nesting_pop, LINE {}", next.line_no);
		}
		/*
			===
		*/

		// println!("{:?};{:?}", stack.head().list, next.list);
		let prev_list = stack.head().list.concat(next.list);

		let prev_list = unsafe
		{
			let r = (prev_list as *mut List);
			(&mut * r)
		};

		prev = Some(prev_list);
	}

	Ok(root)
}

fn p_line ((line_no, input): (usize, String))
 -> std::result::Result<Line, nom::Err<nom::error::Error<usize>>>
{
	let line_no = (line_no + 1);

	let p = tuple
	((
		p_indent,
		p_list_naked,
		map(opt(p_comment), |comment| comment.map_or_else(|| "".into(), |s| s.into())),
	));
	let p = map(p, |(depth, list, comment)|
	{
		Line
		{
			line_no,
			depth,
			list,
			comment,
		}
	});
	let p = all_consuming(p);
	let mut p = p;

	p(&input)
	.map(|(_, line)| line)
	.map_err(|err| err.map_input(|_| line_no))
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

fn p_comment (input: &str) -> ResultOf<&str>
{
	let p = preceded(tag(";"), rest);
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
