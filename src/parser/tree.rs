
use std::mem::replace;

#[derive(Debug)]
pub enum Tree <T>
{
	Leaf(T),
	Branch(Vec<Self>),
}

impl <T> Tree<T>
{
	pub fn root () -> Self
	{
		Tree::Branch(Vec::new())
	}

	pub fn prepend (&mut self, item: Self)
	{
		match self
		{
			Self::Leaf(_) => panic!("tree_prepend_to_leaf"),
			Self::Branch(vec) =>
			{
				vec.splice(0..0, [ item ]);
			},
		}
	}

	pub fn append (&mut self, item: Self) -> &mut Self
	{
		match self
		{
			Self::Leaf(_) =>
			{
				let leaf = replace(self, Self::root());
				self.append(leaf);
				self.append(item)
			},
			Self::Branch(vec) =>
			{
				vec.push(item);
				vec.last_mut().unwrap()
			},
		}
	}

	pub fn concat (&mut self, mut item: Self) -> &mut Self
	{
		match item
		{
			Self::Leaf(_) =>
			{
				/* self.append(item); */
				panic!("tree_concat_to_leaf")
			},
			Self::Branch(ref mut edge) =>
			{
				match edge.len()
				{
					0 => panic!("tree_concat_empty_branch"),
					1 => self.append(edge.pop().unwrap()),
					_ => self.append(item),
				}
			},
		}
	}
}


	/*
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
	*/
