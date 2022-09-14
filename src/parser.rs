// TODO: ? line trailing backslash
// TODO: ? modes? lisp compat
// TODO: multiline comment

use std::io::BufRead;

use nom::IResult;
use nom::error::ParseError;
// use nom::error::Error;
// use nom::Err;

use nom::character::complete::char;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::character::complete::alpha1;
use nom::character::complete::alphanumeric1;
use nom::character::complete::digit1;
use nom::character::complete::one_of;

use nom::bytes::complete::tag;
use nom::bytes::complete::is_not;

use nom::sequence::preceded;
use nom::sequence::delimited;
use nom::sequence::terminated;

use nom::sequence::pair;
use nom::sequence::tuple;
use nom::multi::many0;
use nom::multi::separated_list0;
use nom::multi::fold_many0;

use nom::branch::alt;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::combinator::map;
use nom::combinator::verify;
use nom::combinator::rest;
use nom::combinator::all_consuming;

mod tree;

mod stack;
use stack::Stack;

#[derive(Debug)]
pub enum Literal
{
	Number(i32),
	String(String),
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
	tree: Tree,
	comment: String,
}

pub type Tree = tree::Tree<Form>;

type ResultOf <'S, T> = IResult<&'S str, T>;
type Result <'S> = ResultOf<'S, Tree>;

pub fn parse (input: impl BufRead)
 -> std::result::Result<Tree, nom::Err<nom::error::Error<usize>>>
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
		tree: &'L mut Tree,
	}

	let mut root = Tree::root();
	let mut prev = None as Option<&mut Tree>;
	let mut stack = Stack::new(Frame { depth: 0, tree: &mut root });

	for line in lines
	{
		let line = line?;

		if (is_tree_empty(&line.tree))
		{
			continue
		}

		/*
			>>>
		*/
		if (line.depth > stack.head().depth)
		{
			match prev.take()
			{
				None => {},
				Some(prev) =>
				{
					stack.push(Frame { depth: line.depth, tree: prev });
				}
			}
		}
		/*
			<<<
		*/
		else
		{
			while (line.depth < stack.head().depth)
			{
				stack.pop()
			}

			assert!(line.depth == stack.head().depth, "incorrect_nesting_pop, LINE {}", line.line_no);
		}
		/*
			===
		*/

		let prev_tree = stack.head().tree.concat(line.tree);

		let prev_tree = unsafe
		{
			let r = (prev_tree as *mut Tree);
			(&mut * r)
		};

		prev = Some(prev_tree);
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
	let p = map(p, |(depth, tree, comment)|
	{
		Line
		{
			line_no,
			depth,
			tree,
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
		p_str,
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
	let mut p = map(p, Tree::Branch);

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
	let mut p = map(p, |s: &str| Tree::Leaf(Form::Id(s.into())));

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
		let n = Tree::Leaf(n);
		n
	});
	let mut p = p;

	p(input)
}

fn p_str (input: &str) -> Result
{
	let escape_list = one_of("\"\\nrt");
	let escape = preceded(tag("\\"), escape_list);
	let escape = map(escape, |c|
	{
		match c
		{
			'\\' => '\\'.to_string(),
			 '"' =>  '"'.to_string(),
			 'n' => "\n".to_string(),
			 'r' => "\r".to_string(),
			 't' => "\t".to_string(),
			_ => panic!(),
		}
	});

	let literal = is_not("\"\\");
	let literal = verify(literal, |s: &str| (! s.is_empty()));
	let literal = map(literal, |s: &str| s.to_string());

	let fragment = alt
	((
		escape,
		literal,
	));

	let contents = fold_many0
	(
		fragment,
		String::new,
		|mut total, s|
		{
			total.push_str(&s);
			total
		},
	);

	let p = delimited(char('"'), contents, char('"'));
	let p = map(p, |s|
	{
		let s = Literal::String(s);
		let s = Form::Literal(s);
		let s = Tree::Leaf(s);
		s
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

fn is_tree_empty (tree: &Tree) -> bool
{
	match tree
	{
		Tree::Leaf(_) => panic!(),
		Tree::Branch(vec) => vec.is_empty(),
	}
}
