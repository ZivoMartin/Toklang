#DEFINE
SYMB = digit, letter, symbol
TPRIM =  Keyword, Operator, Number, Ident, Symbol
GROUPS =  Expression, Value

#CHAR_RULES
digit = "0123456789"
letter = "azertyuiopqsdfghjklmwxcvbnAZERTYUIOPQSDFGHJKLMWXCVBN"
symbol = _


#TPRIM_RULES
Number = digit || (digit && Number)
Ident = letter || (digit && Ident || letter && Ident || letter && digit)
Keyword = letter || (letter && Keyword)  
Keyword in ["in", "SYMB", "GROUPS", "TPRIM"]
Symbol = symbol || digit || letter
Operator = symbol || (symbol && symbol)
Operator in ["||", "&&"]

#GROUP_RULES
Expression = (Value{END} && Operator && Expression) || (Symbol{"("} && Expression && Symbol{")", END} && Operator && Expression)     
Value = (Ident{END} && Symbol{"{"} &&  Symbol{"}"})