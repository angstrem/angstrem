
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

	var angNumber = construct('Number', function (node)
	{
		var base;

		base = node[1].join('');
		base = parseInt(base, '10');
		if (node[0] === '-')
		{
			base *= -1;
		}
		return base;
	});
}

program
  = expression:expression { return angExpression(expression) }

//= ./expr.pegjs
//= ./id.pegjs
//= ./numeric.pegjs
//= ./space.pegjs
