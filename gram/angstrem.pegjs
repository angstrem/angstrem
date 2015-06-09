
program
  = expression

expression
  = expression_with_delimiter * expression_single

expression_with_delimiter
  = expression_single expression_delimiter

expression_single
  = spaces? identifier spaces?
  / spaces
  / nothing

expression_delimiter
  = ";"


identifier
  = identifier_first identifier_succedent *

identifier_succedent
  = [01-9]
  / identifier_almost_any

identifier_first
  = identifier_almost_any

identifier_almost_any
  = ([a-z]i / [@$_?])


spaces
  = space +

space
  = [ \n\t]

nothing
  = ""
