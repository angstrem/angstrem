


var
	construct = require('../construct');

module.exports = construct('Number', function (node)
{
	var base;

	base = node[1].join('');
	base = parseInt(base, '10');
	if (node[0] === '-')
	{
		base *= -1;
	}
	return base;
});
