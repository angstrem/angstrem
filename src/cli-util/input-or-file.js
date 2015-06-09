

var
	cat = require('concat-stream');

module.exports = function (file, callback)
{
	var input = stream(file);

	callback = stringed(callback);

	return input.pipe(cat(callback));
}

function stream (file)
{
	if (file)
	{
		var stream = require('fs').createReadStream;

		return stream(file);
	}
	else
	{
		return process.stdin;
	}
}

function stringed (callback)
{
	return function (chunk)
	{
		return callback(chunk.toString('utf-8'));
	}
}
