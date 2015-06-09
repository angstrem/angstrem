
expression
  = expression_with_delimiter * expression_single

expression_with_delimiter
  = expression_single expression_delimiter

expression_single
  = spaces? identifier spaces?
  / spaces
  / nothing

expression_delimiter
  = ";" { return }
