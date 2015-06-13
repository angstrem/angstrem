
expression
  = expression_with_delimiter* expression_element

expression_with_delimiter
  = expression_element expression_delimiter

expression_element
  = expression_loose
  / spaces
  / nothing


// with spaces
expression_loose
  = spaces? expression_strict spaces?

// pure expression
expression_strict
  = expression_binary
  / expression_atomic


// binary operator
expression_binary
  = expression_assignment
  / expression_binary_generic

expression_binary_generic
  = expr:(expression_atomic spaces? operator spaces? expression_strict)
  { return ast.ExpressionBinary(expr) }

expression_assignment
  = expr:(identifier spaces? "=" spaces? expression_strict)
  { return ast.Assignment(expr) }

operator
  = "+" / "-" / "*" / "/" / "div" / "mod" / "**"


// atomic
expression_atomic
  = function_call
  / identifier
  / number


expression_delimiter
  = ";" {}
