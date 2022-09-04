
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

	pub fn append (&mut self, item: Self) -> &mut Self
	{
		match self
		{
			Self::Leaf(_) => panic!("cannot_append_to_leaf"),
			Self::Edge(vec) =>
			{
				vec.push(item);
				vec.last_mut().unwrap()
			},
		}
	}

	pub fn visit <F> (&mut self, fn_visit: &mut F)
		where F: FnMut(&mut Self)
	{
		match self
		{
			Self::Leaf(_) =>
			{
				fn_visit(self);
			},
			Self::Edge(edge) =>
			{
				for item in edge
				{
					item.visit(fn_visit);
				}

				fn_visit(self);
			},
		}
	}

	pub fn visit_top <F> (&mut self, fn_visit: &mut F)
		where F: FnMut(&mut Self)
	{
		fn_visit(self);

		if let Self::Edge(edge) = self
		{
			for item in edge
			{
				item.visit_top(fn_visit);
			}
		}
	}

	/*
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
					0 => panic!("list_concat_empty"),
					1 => self.append(edge.pop().unwrap()),
					_ => self.append(item),
				}
			},
		}
	}
	*/

	pub fn is_edge_empty (&self) -> bool
	{
		match self
		{
			Self::Leaf(_) => false,
			Self::Edge(edge) => edge.is_empty(),
		}
	}
}
