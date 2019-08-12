

from .rootpath import rootpath_from as rootpath

root = rootpath(__file__)
root = rootpath(root())


from .grammar.grammar import grammar
angstrem = grammar()

example = open(root('example/example')).read()
ast = angstrem.model_from_str(example)


for x in ast.stmt:
	print(x)
