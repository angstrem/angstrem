
{
	var ast = require('./ast');
}

program
  = expression:expression { return ast.Expression(expression) }

//= ./expr.pegjs
//= ./fn.pegjs
//= ./id.pegjs
//= ./numeric.pegjs
//= ./space.pegjs
