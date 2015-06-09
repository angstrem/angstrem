


var isArray = Array.isArray;

module.exports = function strip (ast)
{
	ast = ast.filter(filterOnlyLeaves(like));
	ast = ast.map(mapOnlyBranches(strip));

	return ast;
}

function filterOnlyLeaves (fn)
{
	return function (node)
	{
		if (isArray(node))
		{
			return true;
		}
		else
		{
			return fn(node);
		}
	}
}

function mapOnlyBranches (fn)
{
	return function (node)
	{
		if (isArray(node))
		{
			return fn(node);
		}
		else
		{
			return node;
		}
	}
}

function like (term)
{
	if (term == null)
	{
		return false;
	}
	if (! String(term).trim())
	{
		return false;
	}
	return true;
}
