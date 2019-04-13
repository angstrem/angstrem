

from ..rootpath import rootpath_from as rootpath

root = rootpath(__file__)

def read (*path):
	full = root(*path) + '.lark'

	return open(full).read()


from lark import Lark

def grammar ():
	return Lark(read('grammar'), start = 'main')
