

from os.path import dirname

def rootpath_from (file):
	return rootpath(dirname(file))


def rootpath (*path):
	root = resolve(*path)

	def rootpath_join (*path):
		return resolve(root, *path)

	return rootpath_join


from os.path import join

def resolve (*path):
	return join(*path)
