
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

	var angExpressionAssignment = construct('ExpressionAssignment', function (node)
	{
		return {
			left:  node[0],
			right: node[4]
		}
	});

	var angExpressionBinary = construct('ExpressionBinary', function (node)
	{
		return {
			operator: node[2],

			left:  node[0],
			right: node[4]
		}
	});

	var angFunctionCall = construct('FunctionCall', function (node)
	{
		var name = node[0].data;

		var body = node.slice(3, -1);

		body = strip(body);
		body = flat(body);

		//body = body.filter(Boolean);

		//console.dir(node, Infinity);

		return {
			name: name,
			args: body
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
			body = node.slice(1);

		switch (type)
		{
		case '//':
			type = 'line';
			body = body.slice(0, -1);
			break;

		case '/*':
			type = 'block';
			body = body.slice(0, -3);
			break;
		}

		body = body[0];

		body = body.join('');

		return {
			type: type,
			body: body
		}
	});
}

program
  = expression:expression { return angExpression(expression) }

//= ./expr.pegjs
//= ./fn.pegjs
//= ./id.pegjs
//= ./numeric.pegjs
//= ./space.pegjs
