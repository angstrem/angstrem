


var construct = require('../construct');

module.exports = construct('Identifier', function (node)
{
	return [].concat(node[0], node[1]).join('');
});
