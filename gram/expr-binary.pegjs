
expression_binary
  = expression:(
    expression_atomic
      spaces?
    operator
      spaces?
    expression_strict) { return angExpressionBinary(expression) }

operator
  = "+" / "-" / "*" / "/" / "//" / "%" / "**"
