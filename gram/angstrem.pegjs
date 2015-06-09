
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

expression
  = expression_with_delimiter * expression_single

expression_with_delimiter
  = expression_single expression_delimiter

expression_single
  = spaces? identifier spaces?
  / spaces
  / nothing

expression_delimiter
  = ";" { return }

identifier
  = identifier: (identifier_first identifier_succedent *) { return angIdentifier(identifier) }

identifier_succedent
  = [01-9]
  / identifier_almost_any

identifier_first
  = identifier_almost_any

identifier_almost_any
  = ([a-z]i / [@$_?])


spaces
  = space +

space
  = [ \n\r\t]

nothing
  = ""
