


var construct = require('../construct');

module.exports = construct('String', function (node)
{
	var content = node
	.slice(1, -1)
	[0]
	.map(function (chunk)
	{
		if (! chunk[0])
		{
			return chunk[1];
		}
		else
		{
			return escape(chunk[1]);
		}
	})
	.join('');

	return content;
});

function escape (character)
{
	switch (character)
	{
		case 'n': return '\n';
		case 'r': return '\r';
		case 't': return '\t';
		case '\\': return '\\';
		default: return character;
	}
}
