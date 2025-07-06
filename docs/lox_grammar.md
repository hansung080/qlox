# Lox Grammar

This defines the Lox grammar written in the new notation created for Lox. 
```
expression -> binary
            | grouping
            | literal
            | unary ;
binary     -> expression operator expression ;
grouping   -> "(" expression ")" ;
literal    -> NUMBER | STRING | "true" | "false" | "nil" ;
operator   -> "+" | "-" | "*" | "/" | "==" | "!=" | "<" | "<=" | ">" | ">=" ;
unary      -> ( "-" | "!" ) expression ;
```