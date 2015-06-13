
comment
  = comment:(comment_line / comment_block) { return ast.Comment(comment) }

comment_line
  = "//" [^\n]* "\n"

comment_block
  = "/*" (!"*/" .)* "*/"
