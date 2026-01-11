# [C10 - Functions](https://craftinginterpreters.com/functions.html)

## Let's go

- It's parentheses "()" that trigger a function call - not the name of the function itself!
- Think of `(` as a post-fix operator with high precedence:

```text
unary          → ( "!" | "-" ) unary | call ;
call           → primary ( "(" arguments? ")" )* ;
```

with

```
arguments      → expression ( "," expression )* ;
```
