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

## While loops

```text
statement      → exprStmt
               | ifStmt
               | printStmt
               | whileStmt
               | block ;

whileStmt      → "while" "(" expression ")" statement ;
```

- same syntax as C

## For loops

```text
statement      → exprStmt
               | forStmt
               | ifStmt
               | printStmt
               | whileStmt
               | block ;

forStmt        → "for" "(" ( varDecl | exprStmt | ";" )
                 expression? ";"
                 expression? ")" statement ;
```

- `for` statements are really just syntactic sugar - not offering any new capabilities over what we already have.
- i.e., the following statement `for (var i = 0; i < 10; i = i + 1) print i;` can be written as:

```text
{
  var i = 0;
  while (i < 10) {
    print i;
    i = i + 1;
  }
}
```

- As such, we will decompose `for` loops into their fundamentals. This doesn't really save any cost right now but could in more complex examples of syntactic sugaring
