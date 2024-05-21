#DECLARE
CHARS = digit, letter, symbol
TPRIMS = Keyword, Operator, Number, Ident, Symbol
GROUPS = Expression, Value

#CHAR_RULES
digit = "0123456789"
letter = "azertyuiopqsdfghjklmwxcvbnAZERTYUIOPQSDFGHJKLMWXCVBN"
symbol = "#=,&|()"

#TPRIM_RULES
Number = digit{END, "zz"} || (digit && Number)
Ident = letter || (letter && (digit && Ident || Ident || digit)) 
Keyword = letter || (letter && Keyword)  
Keyword in ["in", "SYMB", "GROUPS", "TPRIM"]
Symbol = symbol || digit || letter
Operator = symbol || (symbol && symbol)
Operator in ["||", "&&"]

#GROUP_RULES
Expression = (Value{END} && Operator && Expression) || (Symbol{"("} && Expression && Symbol{")", END} && Operator && Expression)     
Value = (Ident{END} && Symbol{"{"} && Symbol{"}"})