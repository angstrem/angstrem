
{
	var strip = require('./strip-ast');
	var flat = require('lodash.flattendeep');

	function angExpression (node)
	{
		node = strip(node);
		node = flat(node);

		return node;
	}

	var construct = require('./construct');

	var angIdentifier = construct('Identifier', function (node)
	{
		return [].concat(node[0], node[1]).join('');
	});
}

program
  = expression:expression { return angExpression(expression) }

//= ./expr.pegjs
//= ./id.pegjs
//= ./space.pegjs
