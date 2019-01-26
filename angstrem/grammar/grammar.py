
from os.path import dirname
from os.path import join

def read (*path):
	full = join(dirname(__file__), *path)
	full = full + '.peg'

	return open(full).read()


from parsimonious.grammar import Grammar

def grammar ():
	peg = read('grammar')

	return Grammar(peg)
