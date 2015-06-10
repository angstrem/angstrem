


module.exports = function Eval (ast, process)
{
	ast.forEach(function (node)
	{
		console.dir(node, Infinity);
	});
}
