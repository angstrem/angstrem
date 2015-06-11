
comment
  = comment:(comment_line / comment_block) { return angComment(comment) }

comment_line
  = "//" [^\n]* "\n"

comment_block
  = "/*" (!"*/" .)* "*/"
