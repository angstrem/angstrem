

from .rootpath import rootpath_from as rootpath

root = rootpath(__file__)
root = rootpath(root())


from .grammar.grammar import grammar
angstrem = grammar()


example = open(root('example/example')).read()


ast = angstrem.parse(example)
print(1, ast)


from parsimonious.nodes import NodeVisitor

visitor = NodeVisitor()
visitor.grammar = angstrem

def noop (*_): pass

def v (node, children):
	print(node.expr.name, node.text)
	print(node.match)
	print('---')
	# return '000',

visitor.generic_visit = noop
visitor.visit_word = v

visitor.visit(ast)

print(2, ast)
