

#from ..rootpath import rootpath_from as rootpath

#root = rootpath(__file__)

#def read (*path):
#	full = root(*path) + '.lark'

#	return open(full).read()


#from lark import Lark

#def grammar ():
#	return Lark(read('grammar'), start = 'main')


from textx import metamodel_from_str

def grammar ():
	return metamodel_from_str("""
	Model: stmt *= Stmt;
	Stmt: Decl | Expr;
	Decl: Decl__Func;

	Decl__Func: 'fn' ID '(' Func__Args? ')' Func__Body;
	Func__Args: ID ( ',' ID )*;
	Func__Body: Expr;

	Expr:
		Block | Comment;

	Block: '{' Block__Body? '}';
	Block__Body: Expr ( Delimiter Expr )* ;

	Delimiter: ';' | '\n';

	Comment: Comment__Line | Comment__Block;
	Comment__Line: /\/\/.*$/;
	Comment__Block: /\/\*.*?\*\//;
	""")
