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

## Logical Operators

```text
expression     → assignment ;
assignment     → IDENTIFIER "=" assignment
               | logic_or ;
logic_or       → logic_and ( "or" logic_and )* ;
logic_and      → equality ( "and" equality )* ;
```

- Low priority, slotted between `assignment` and `equality`.
- `and` is higher prio than `or`
- Greedily evaluating `and`: we don't need to evaluate the right-hand expression if the left-hand  expression is `false`
  - Due to this short-circuiting, we'll add separate code instead of putting these into the binary operators to keep things cleanly separated
