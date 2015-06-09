
expression
  = expression_with_delimiter* expression_single

expression_with_delimiter
  = expression_single expression_delimiter

expression_single
  = expression_single_loose
  / spaces
  / nothing

expression_single_loose
  = spaces? expression_single_strict spaces?

expression_single_strict
  = identifier
  / number

expression_delimiter
  = ";" { return }
