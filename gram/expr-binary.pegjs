
expression_binary
  = expression_assignment
  / expression_binary_generic

expression_binary_generic
  = expr:(expression_atomic spaces? operator spaces? expression_strict)
  { return angExpressionBinary(expr) }

expression_assignment
  = expr:(identifier spaces? "=" spaces? expression_strict)
  { return angExpressionAssignment(expr) }

operator
  = "+" / "-" / "*" / "/" / "//" / "%" / "**"
