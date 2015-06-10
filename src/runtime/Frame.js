


var Eval = require('./Eval');

var Frame = module.exports = function Frame (ast)
{
	this.ast = ast;
	this.context = {};

	this.eval = Eval(this);
}

Frame.prototype.run = function ()
{
	this.ast.forEach(this.eval);
}
