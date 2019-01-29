

from ..rootpath import rootpath_from as rootpath

root = rootpath(__file__)

def read (*path):
	full = root(*path) + '.peg'

	return open(full).read()


from parsimonious.grammar import Grammar

def grammar ():
	peg = read('grammar')

	return Grammar(peg)