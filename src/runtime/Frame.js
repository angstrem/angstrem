


var Eval = require('./Eval');

var Frame = module.exports = function Frame (ast)
{
	this.ast = ast;
	this.context = {};

	this.eval = Eval(this);
}

Frame.prototype.run = function ()
{
	return this.ast.reduce(reducer(this.eval), null);
}

function reducer (eval)
{
	return function (memo, ast)
	{
		return eval(ast);
	}
}
