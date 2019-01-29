

from .rootpath import rootpath_from as rootpath

root = rootpath(__file__)
root = rootpath(root())


from .grammar.grammar import grammar
angstrem = grammar()


example = open(root('example/example')).read()


ast = angstrem.parse(example)
print('AST', ast)


from parsimonious.nodes import NodeVisitor
from parsimonious.nodes import Node

class Visitor (NodeVisitor):
	def __init__ (self):
		self.grammar = angstrem

	def generic_visit (self, node, children):
		return node

	def visit_main (self, node, children):
		return children

	def visit_stmt (self, node, children):
		return children[0]

	def visit_word (self, node, children):
		return node.text

visitor = Visitor()

ast2 = visitor.visit(ast)

print('R', ast2)
