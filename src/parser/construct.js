


module.exports = function construct (id, fnTransform)
{
	var fn = fnTransform;

	if (! fn)
	{
		fn = function (node) { return node }
	}

	var constructor = function AstConstructor (node)
	{
		return {
			type: id,
			data: fn(node)
		}
	}

	constructor.id = id;

	return constructor;
}
