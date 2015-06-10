


module.exports = function construct (id, fnTransform)
{
	var fn = fnTransform;

	if (! fn)
	{
		fn = function (node) { return node }
	}

	var constructor = function AstConstructor ()
	{
		return {
			type: id,
			data: fn.apply(this, arguments)
		}
	}

	constructor.id = id;

	return constructor;
}
