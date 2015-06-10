


function_call
  = fn:(identifier spaces? "(" function_call_args ")") { return angFunctionCall(fn) }

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
