
#[derive(Debug)]
pub struct Pos
{
	pub line: usize,
	pub char: usize,
}

impl Pos
{
	pub fn new () -> Pos
	{
		Pos { line: 1, char: 1 }
	}

	fn next (&mut self)
	{
		self.char = (self.char + 1);
	}

	fn feed (&mut self)
	{
		self.line = (self.line + 1);
		self.char = 1;
	}

	pub fn advance (&mut self, ch: char)
	{
		match ch
		{
			'\n' => self.feed(),
			  _  => self.next(),
		}
	}
}
