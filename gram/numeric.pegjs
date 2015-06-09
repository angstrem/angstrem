
number
  = number_integer

number_integer
  = number:(number_sign? number_digit+) { return angNumber(number) }

number_sign
  = [+-]

number_digit
  = [01-9]
