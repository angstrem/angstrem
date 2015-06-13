

var
	strip = require('../strip-ast'),
	flat = require('lodash.flattendeep');

module.exports = function angExpression (node)
{
	node = strip(node);
	node = flat(node);

	return node;
}
