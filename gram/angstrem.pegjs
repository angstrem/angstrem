
{
	var strip = require('./strip-ast');
	var flat = require('lodash.flattendeep');

	function angExpression (rule)
	{
		rule = strip(rule);
		rule = flat(rule);

		return rule;
	}

	function angIdentifier (rule)
	{
		var id = [].concat(rule[0], rule[1]).join('');

		return {
			type: "Identifier",
			data: id
		}
	}
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
  = ";"

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
