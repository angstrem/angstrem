
expression
  = expression_with_delimiter* expression_element

expression_with_delimiter
  = expression_element expression_delimiter

expression_element
  = expression_loose
  / spaces
  / nothing

expression_loose
  = spaces? expression_strict spaces?

expression_strict
  = expression_binary
  / expression_atomic

expression_atomic
  = identifier
  / number

//= ./expr-binary.pegjs

expression_delimiter
  = ";" { return }
