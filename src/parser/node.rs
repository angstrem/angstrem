
#[derive(Debug)]
pub struct Node
{
	pub value: String,
	pub nodes: Vec<Node>,
}

impl Node
{
	pub fn new (value: String) -> Node
	{
		Node
		{
			value,
			nodes: Vec::new(),
		}
	}
}

impl From<&str> for Node
{
	fn from (str: &str) -> Node
	{
		Node::new(str.into())
	}
}
