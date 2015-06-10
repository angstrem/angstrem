


function_call
  = identifier spaces? "(" function_call_args ")"

function_call_args
  = function_call_arglist
  / function_call_arg
  / spaces
  / nothing

function_call_arglist
  = (spaces? function_call_arg spaces? function_call_arg_delimiter)* spaces? function_call_arg

function_call_arg_delimiter
  = ","

function_call_arg
  = expression_strict
