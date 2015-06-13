
{
	var strip = require('./strip-ast');
	var flat = require('lodash.flattendeep');
	var ast = require('./ast');

	var construct = require('./construct');

	var angExpressionBinary = construct('ExpressionBinary', function (node)
	{
		return {
			operator: node[2],

			left:  node[0],
			right: node[4]
		}
	});

	var angFunctionCall = construct('FunctionCall', function (name, args)
	{
		name = name.data;

		args = flat(args);
		args = strip(args);

		return {
			name: name,
			args: args
		}
	});

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
