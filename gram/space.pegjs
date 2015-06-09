
//= ./comment.pegjs

spaces
  = space+

space
  = comment
  / space_symbols

space_symbols
  = [ \n\r\t]

nothing
  = ""
