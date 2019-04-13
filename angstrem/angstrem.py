

from .rootpath import rootpath_from as rootpath

root = rootpath(__file__)
root = rootpath(root())


from .grammar.grammar import grammar
angstrem = grammar()


example = open(root('example/example')).read()


ast = angstrem.parse(example)

print(ast.data)
print(ast.children)
print(ast.meta)
