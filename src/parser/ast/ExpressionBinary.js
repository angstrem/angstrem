

var construct = require('../construct');

module.exports = construct('ExpressionBinary', function (node)
{
	return {
		operator: node[2],

		left:  node[0],
		right: node[4]
	}
});
