


var construct = require('../construct');

module.exports = construct('Assignment', function (node)
{
	return {
		left:  node[0],
		right: node[4]
	}
});
