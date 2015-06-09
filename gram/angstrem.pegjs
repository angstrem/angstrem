
{
	var strip = require('./strip-ast');

	function angIdentifier (rule)
	{
		var id = [].concat(rule[0], rule[1]).join('');

		return [ "Identifier", id ];
	}
}

program
  = expression:expression { return strip(expression) }

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
  = identifier: (identifier_first identifier_succedent *) { return angIdentifier(identifier); }

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
