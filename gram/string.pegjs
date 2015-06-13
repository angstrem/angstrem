

string
  = string:(
    single_quoted
    /
    double_quoted
    ) { return ast.String(string) }

single_quoted
  = "'" (string_escape / (! "'" .))* "'"

double_quoted
  = '"' (string_escape / (! '"' .))* '"'

string_escape
  = "\\" string_escape_special

string_escape_special
  = "n"
  / "r"
  / "t"
  / "\\"
  / "'"
  / '"'
