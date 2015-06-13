


function_call
  = name:identifier spaces? "(" args:function_call_args ")" { return ast.FunctionCall(name, args) }

function_call_args
  = function_call_arglist
  / spaces
  / nothing

function_call_arglist
  = (spaces? function_call_arg spaces? function_call_arg_delimiter)* spaces? function_call_arg spaces?

function_call_arg_delimiter
  = ","

function_call_arg
  = expression_strict
