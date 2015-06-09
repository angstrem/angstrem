
program
  = identifier

identifier
  = identifier_first identifier_succedent *

identifier_succedent
  = [01-9]
  / identifier_almost_any

identifier_first
  = identifier_almost_any

identifier_almost_any
  = ([a-z]i / "@" / "$" / "_" / "?")
