


var
	flat = require('lodash.flattendeep');

var
	construct = require('../construct'),
	strip = require('../strip-ast');

module.exports = construct('FunctionCall', function (name, args)
{
	name = name.data;

	args = flat(args);
	args = strip(args);

	return {
		name: name,
		args: args
	}
});
