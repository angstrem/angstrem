
{
	var ast = require('./ast');

	var construct = require('./construct');

	var angIdentifier = construct('Identifier', function (node)
	{
		return [].concat(node[0], node[1]).join('');
	});
}

program
  = expression:expression { return ast.Expression(expression) }

//= ./expr.pegjs
//= ./fn.pegjs
//= ./id.pegjs
//= ./numeric.pegjs
//= ./space.pegjs
