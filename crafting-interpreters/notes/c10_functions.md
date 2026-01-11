# [C10 - Functions](https://craftinginterpreters.com/functions.html)

## Let's go

- It's parentheses "()" that trigger a function call - not the name of the function itself!
- Think of `(` as a post-fix operator with high precedence:

```text
unary          → ( "!" | "-" ) unary | call ;
call           → primary ( "(" arguments? ")" )* ;
```

with

```text
arguments      → expression ( "," expression )* ;
```

## Function declarations

- like variables, function declarations bind a new name

```text
declaration    → funDecl
               | varDecl
               | statement ;
```

with these new rules:

```text
funDecl        → "fun" function ;
function       → IDENTIFIER "(" parameters? ")" block ;
parameters     → IDENTIFIER ( "," IDENTIFIER )* ;
```
