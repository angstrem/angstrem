
{
	var strip = require('./strip-ast');
	var flat = require('lodash.flattendeep');
	var ast = require('./ast');

	var construct = require('./construct');

	var angIdentifier = construct('Identifier', function (node)
	{
		return [].concat(node[0], node[1]).join('');
	});

	var angComment = construct('Comment', function (node)
	{

		var
			type = node[0],
			body = node.slice(1, -1);

		switch (type)
		{
		case '//':
			type = 'line';
			break;

		case '/*':
			type = 'block';
			break;
		}

		body = flat(body);
		body = body.filter(Boolean);
		body = body.join('');

		return {
			type: type,
			body: body
		}
	});
}

program
  = expression:expression { return ast.Expression(expression) }

//= ./expr.pegjs
//= ./fn.pegjs
//= ./id.pegjs
//= ./numeric.pegjs
//= ./space.pegjs
