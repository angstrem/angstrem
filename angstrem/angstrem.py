

from .rootpath import rootpath_from as rootpath

root = rootpath(__file__)
root = rootpath(root())


from .grammar.grammar import grammar


example = open(root('example/example')).read()


ast = grammar().parse(example)
print(ast)
