

var
	flat = require('lodash.flattendeep');

var
	construct = require('../construct');

module.exports = construct('Comment', function (node)
{

	var
		type = node[0],
		body = node.slice(1, -1);

	switch (type)
	{
	case '//':
		type = 'line';
		break;

	case '/*':
		type = 'block';
		break;
	}

	body = flat(body);
	body = body.filter(Boolean);
	body = body.join('');

	return {
		type: type,
		body: body
	}
});
