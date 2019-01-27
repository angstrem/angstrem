

from .rootpath import rootpath_from as rootpath

root = rootpath(__file__)
root = rootpath(root())


from .grammar.grammar import grammar
angstrem = grammar()


example = open(root('example/example')).read()


ast = angstrem.parse(example)
print(1, ast)


from parsimonious.nodes import NodeVisitor
from parsimonious.nodes import Node

class Visitor (NodeVisitor):
	def __init__ (self):
		self.grammar = angstrem

	def generic_visit (self, node, children):
		node = Node(node.expr, node.full_text, node.start, node.end)
		node.children = children
		return node

	def visit_word (self, node, children):
	# def visit_stmt (self, node, children):
		print(node.expr.name, node.text)
		print('match' in node and node.match)
		print(children)
		print('---')
		# return ''
		return node

visitor = Visitor()

ast2 = visitor.visit(ast)

print(1, ast)
print(2, ast2)
