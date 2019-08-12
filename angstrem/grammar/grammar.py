
from textx import metamodel_from_str

def grammar ():
	return metamodel_from_str(read('grammar'))


#
def read (*path):
	full = root(*path) + '.textx'

	return open(full).read()


#
from ..rootpath import rootpath_from as rootpath

root = rootpath(__file__)
