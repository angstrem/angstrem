
#[derive(Debug)]
pub enum List <T>
{
	Leaf(T),
	Edge(Vec<Self>),
}

impl <T> List<T>
{
	pub fn root () -> Self
	{
		List::Edge(Vec::new())
	}

	pub fn from_vec (list: Vec<Self>) -> Self
	{
		List::Edge(list)
	}

	pub fn append (&mut self, item: Self)
	{
		match self
		{
			Self::Leaf(_)   => panic!("cannot_append_to_leaf"),
			Self::Edge(vec) => vec.push(item),
		}
	}

	pub fn concat (&mut self, mut item: Self)
	{
		match &mut item
		{
			Self::Leaf(_) =>
			{
				// self.append(item);
				panic!("list_concat_leaf");
			},
			Self::Edge(edge) =>
			{
				match edge.len()
				{
					0 => panic!("empty"),
					1 => self.append(edge.pop().unwrap()),
					_ => self.append(item),
				}
			},
		}
	}
}