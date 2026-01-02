# [C8 - Statements and State](https://craftinginterpreters.com/statements-and-state.html)

Time to add statements, yeah!

## Notes

The new top-level of the grammar will now be:

```text
program        → statement* EOF ;

statement      → exprStmt
               | printStmt ;

exprStmt       → expression ";" ;
printStmt      → "print" expression ";" ;
```

I.e. we are now constructing a program as a series of statements followed by an EOF, where there are two types of statements: expression statements and `print` statements.

- More types of statements will come in the following chapters.

## Questions

- ArrayList?
- Why the generics in `Stmnt` type? Don't statements by definition have no return type?
