


var readline = require('readline-sync');

module.exports = function Eval (frame)
{
	return function eval (node)
	{
		if (node.type in instr)
		{
			return instr[node.type].call(frame, node.data);
		}
		else
		{
			console.error('Unknown instruction: %s', node.type);
			process.exit(1);
		}
	}
}


var instr = {};

instr.Assignment = function (data)
{
	var id = data.left.data;
	var right = this.eval(data.right);

	this.context[id] = right;
}

instr.Identifier = function (data)
{
	var id = data;

	if (id in this.context)
	{
		return this.context[id];
	}
	else
	{
		return null;
	}
}

instr.FunctionCall = function (data)
{
	var name = data.name;

	if (name in core.fn)
	{
		var args = data.args;

		args = args.map(this.eval);

		var fn = core.fn[name];

		return fn.apply(null, args);
	}
	else
	{
		console.error('Unknown function: %s', name);
		process.exit(2);
	}
}

instr.ExpressionBinary = function (data)
{
	var operator = data.operator;

	var left = this.eval(data.left);
	var right = this.eval(data.right);

	switch (operator)
	{
	case '+': return Number(left) + Number(right);
	}
}

instr.Comment = function () {}

var core = {};

core.fn = {};

core.fn.in = function ()
{
	return readline.question('input some stuff: ');
}
core.fn.out = function (data)
{
	data = String(data);
	data += '\n';

	process.stdout.write(data);
}
