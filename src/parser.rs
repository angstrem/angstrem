
use std::io;
// use std::io::BufRead as Buf;
use std::io::Read;
use std::io::ErrorKind;

use char_reader::CharReader as Reader;

mod node;
use node::Node;

mod pos;
use pos::Pos;


#[derive(Debug)]
enum Next <T>
{
	Value(T),
	Eof,
	Err(io::Error),
}

impl <T> From<Next<T>> for io::Result<T>
{
	fn from (next: Next<T>) -> Self
	{
		match next
		{
			Next::Err(error)
			 => Err(error),
			Next::Eof
			 => Err(ErrorKind::UnexpectedEof.into()),
			Next::Value(value)
			 => Ok(value),
		}
	}
}


struct Context <R: Read>
{
	depth: usize,
	pos: Pos,
	reader: Reader<R>,
}

impl <R: Read> Context<R>
{
	fn new (reader: Reader<R>) -> Context<R>
	{
		Context
		{
			depth: 0,
			pos: Pos::new(),
			reader,
		}
	}

	fn peek (&mut self) -> Next<char>
	{
		let next = self.reader.peek_char();

		if let Err(error) = next
		{
			return Next::Err(error);
		}

		let next = next.unwrap();

		if next.is_none()
		{
			return Next::Eof;
		}

		let ch = next.unwrap();
		Next::Value(ch)
	}

	fn read (&mut self) -> Next<char>
	{
		let next = self.peek();

		if let Next::Value(ch) = next
		{
			self.reader.next_char().unwrap();

			self.pos.advance(ch);
		}

		next
	}
}


pub fn parse (input: impl Read) -> io::Result<Node>
{
	let reader = Reader::new(input);
	let mut context = Context::new(reader);

	parse_module(&mut context)
}


fn parse_module (context: &mut Context<impl Read>) -> io::Result<Node>
{
	let root: Node = "".into();
	// let mut root: Node = "".into();
	// let mut sibl: Option<Node> = None;

	loop
	{
		let line = parse_line(context);

		let line = match line
		{
			Next::Err(error) => return Err(error),
			Next::Eof => break,
			Next::Value(line) => line,
		};

		println!("{:?} {:?}", context.pos, line);
	}

	Ok(root)
}


#[derive(Debug)]
struct Line
{
	depth: usize,
	node: Node,
}

fn parse_line (context: &mut Context<impl Read>) -> Next<Line>
{
	let depth = parse_ident(context);
	let depth = match depth
	{
		Next::Value(depth)
		 => depth,
		Next::Eof
		 => return Next::Eof,
		Next::Err(error)
		 => return Next::Err(error),
	};

	let node = parse_node(context);
	let node = match node
	{
		Next::Value(node)
		 => node,
		Next::Eof
		 => return Next::Eof,
		Next::Err(error)
		 => return Next::Err(error),
	};

	let line = Line { depth, node };

	Next::Value(line)
}


fn parse_ident (context: &mut Context<impl Read>) -> Next<usize>
{
	let mut ident = 0usize;

	loop
	{
		let next = context.peek();

		let ch = match next
		{
			Next::Value(ch) => ch,
			Next::Eof => break,
			Next::Err(error) => return Next::Err(error),
		};

		match ch
		{
			' ' | '\t' => ident = (ident + 1),
			_ => break,
		}

		context.read();
	}

	Next::Value(ident)
}


fn parse_node (context: &mut Context<impl Read>) -> Next<Node>
{
	let mut value = String::new();

	loop
	{
		let next = context.read();

		let ch = match next
		{
			Next::Value(ch) => ch,
			Next::Eof => return Next::Eof,
			Next::Err(error) => return Next::Err(error),
		};

		match ch
		{
			'\n' => break,
			_ => value.push(ch),
		}
	}

	Next::Value(Node::new(value))
}
