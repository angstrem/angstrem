// TODO: rm

use std::io::BufRead;

pub fn parse (input: impl BufRead) -> Node
{
	let mut root = Node::root();
	let parse_node_context = ParseNodeContext::new(&mut root);

	input
	.lines()
	.map(|line| line.unwrap())
	.enumerate()
	.map(parse_depth)
	.filter(filter_empty)
	.fold(parse_node_context, parse_node);

	root
}


#[derive(Debug)]
pub struct Indented
{
	pub line:  usize,
	pub depth: usize,
	pub value: String,
}

fn parse_depth ((line, input): (usize, String)) -> Indented
{
	let line = (line + 1);

	let mut skip  = true;
	let mut depth = 0usize;

	let value = input
	.chars()
	.skip_while(|ch|
	{
		if (skip)
		{
			if (ch.is_ascii_whitespace())
			{
				depth = (depth + 1);
			}
			else
			{
				skip = false
			}
		}

		skip
	})
	.collect();

	Indented { line, depth, value }
}


fn filter_empty (indented: &Indented) -> bool
{
	indented.value.len() != 0
}


#[derive(Debug)]
pub struct Node
{
	pub value: String,
	pub nested: Vec<Node>,
}

impl Node
{
	fn new (value: String) -> Node
	{
		Node { value, nested: vec![] }
	}

	fn root () -> Node
	{
		Self::new(String::new())
	}
}

struct ParseNodeContext <'N>
{
	node_tip: &'N mut Node,
	node_prev_o: Option<&'N mut Node>,
	depth: usize,
	stack: Vec<(&'N mut Node, usize)>,
}

impl <'N> ParseNodeContext<'N>
{
	fn new (root: &'N mut Node) -> ParseNodeContext<'N>
	{
		ParseNodeContext
		{
			node_tip: root,
			node_prev_o: None,
			depth: 0,
			stack: Vec::new(),
		}
	}
}

fn parse_node (mut parse_node_context: ParseNodeContext, next: Indented) -> ParseNodeContext
{
	let next_depth = next.depth;

	if next_depth > parse_node_context.depth
	{
		/* >>> */
		match parse_node_context.node_prev_o
		{
			None => {},
			Some(node_prev) =>
			{
				parse_node_context.stack.push((parse_node_context.node_tip, parse_node_context.depth));

				parse_node_context.node_tip = node_prev;
				parse_node_context.node_prev_o = None;
			},
		}

		parse_node_context.depth = next_depth;
	}
	else
	{
		/* <<< */
		loop
		{
			if (parse_node_context.stack.is_empty())    { break }
			if (next_depth >= parse_node_context.depth) { break }

			(parse_node_context.node_tip, parse_node_context.depth) = parse_node_context.stack.pop().unwrap();
		}

		assert!(next_depth == parse_node_context.depth, "incorrect_nesting_pop, LINE {}", next.line);
	}

	/* === */
	let next_node = Node::new(next.value);
	parse_node_context.node_tip.nested.push(next_node);

	unsafe
	{
		let ref1 = parse_node_context.node_tip.nested.last_mut().unwrap();
		let ptr = ref1 as *mut Node;
		let ref2 = &mut * ptr;

		parse_node_context.node_prev_o = Some(ref2);
	}

	parse_node_context
}
