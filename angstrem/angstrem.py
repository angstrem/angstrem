

from .rootpath import rootpath_from as rootpath

root = rootpath(__file__)
root = rootpath(root())


#
from .grammar.grammar import grammar
angstrem = grammar()

example = open(root('example/example')).read()
ast = angstrem.model_from_str(example)


#
class Visitor:
	def __init__ (self, node_type):
		self.node_type = node_type

	def visit (self, ast):
		for node in getattr(ast, self.node_type):
			type_name = node.__class__.__name__
			getattr(self, type_name)(node)

class AngstremVisitor (Visitor):
	def __init__ (self):
		super().__init__('stmt')

	def Decl__Func (self, node):
		print(node.name, node.args)


#
v = AngstremVisitor()
v.visit(ast)
