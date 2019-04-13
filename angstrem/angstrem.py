

from .rootpath import rootpath_from as rootpath

root = rootpath(__file__)
root = rootpath(root())


from .grammar.grammar import grammar
angstrem = grammar()


example = open(root('example/example')).read()


ast = angstrem.parse(example)

# print(ast)

# print(ast.data)
# print(ast.children)
# print(ast.meta)

# for t in ast.iter_subtrees():
# for t in ast.iter_subtrees_topdown():
#	print(t)


from lark import Token, Tree

def vis (node, depth = 0):
	def name (node):
		if isinstance(node, Token):
			return '. {} {}'.format(node.type, node)
		if isinstance(node, Tree):
			return '= {}'.format(node.data)

	print('{}{}'.format('  ' * depth, name(node)))

	if isinstance(node, Tree):
		for n in node.children:
			vis(n, depth + 1)


print(example)
print('---')
vis(ast)
