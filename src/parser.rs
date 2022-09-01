
use std::io::BufRead;

use nom::IResult;
// use nom::error::Error;

use nom::character::complete::space1;
use nom::character::complete::alpha1;
use nom::character::complete::alphanumeric1;

use nom::bytes::complete::tag;

use nom::branch::alt;
use nom::sequence::pair;
use nom::multi::many0;
use nom::multi::separated_list0;

use nom::combinator::recognize;
use nom::combinator::map;


#[derive(Debug)]
enum Node
{
	String(String),
	List(Vec<Node>),
}

type Result <'S> = IResult<&'S str, Node>;

pub fn parse (_input: impl BufRead) -> ()
{
	let foo = "a1bcd_d b1 c2 _d_d";
	// let foo = "a1bcd_d (b c) d";

	// println!("{:#?}", p_id(foo));
	// println!("{:#?}", p_id("bar1"));
	println!("{:#?}", p_list_naked(foo));
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

fn p_list_naked (input: &str) -> Result
{
	// let open = tag("(");
	// let close = tag(")");

	let p = separated_list0(space1, p_id);
	let mut p = map(p, Node::List);

	p(input)
}

/*
pub fn parse(input: &str) -> IResult<&str, &str> {
  recognize(
    pair(
      alt((alpha1, tag("_"))),
      many0_count(alt((alphanumeric1, tag("_"))))
    )
  )(input)
}
*/
