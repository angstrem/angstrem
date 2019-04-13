

from .rootpath import rootpath_from as rootpath

root = rootpath(__file__)
root = rootpath(root())


from .grammar.grammar import grammar
angstrem = grammar()


example = open(root('example/example')).read()


ast = angstrem.parse(example)
print('AST', ast)
print('---')
print('AST', ast.pretty())
