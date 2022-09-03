
#[derive(Debug)]
pub struct Stack <T>
{
	stack: Vec<T>,
}

impl <T> Stack<T>
{
	pub fn new (head: T) -> Self
	{
		Stack { stack: vec![ head ] }
	}

	pub fn head (&mut self) -> &mut T
	{
		self.stack.last_mut().expect("stack_empty")
	}

	pub fn push (&mut self, head: T)
	{
		self.stack.push(head);
	}

	pub fn pop (&mut self)
	{
		self.stack.pop().expect("stack_pop_empty");
	}

	pub fn take (mut self) -> T
	{
		assert!(self.stack.len() == 1, "stack_take_1");
		self.stack.pop().unwrap()
	}
}
