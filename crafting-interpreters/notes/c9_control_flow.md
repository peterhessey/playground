# [C9 - Control Flow](https://craftinginterpreters.com/control-flow.html)

## If statements let's go

```text
statement      → exprStmt
               | ifStmt
               | printStmt
               | block ;

ifStmt         → "if" "(" expression ")" statement
               ( "else" statement )? ;
```
