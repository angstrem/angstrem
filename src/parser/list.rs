
#[derive(Debug)]
pub enum List <T>
{
	Leaf(T),
	Edge(Vec<Self>),
}

impl <T> List<T>
{
	// pub fn root () -> Self
	// pub fn from_value (value: T) -> Self

	pub fn from_vec (list: Vec<Self>) -> Self
	{
		List::Edge(list)
	}
}
