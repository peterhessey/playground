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

### Global Variables

The grammar is now extended to the following to allowing declaring (global) variables:

```text
program        → declaration* EOF ;

declaration    → varDecl
               | statement ;

statement      → exprStmt
               | printStmt ;
```

Again using the same patterns as with the operators to handle precedence (I think this means that a `statement` has higher precendence than a `declaration`?)

> `declaration` falls through to `statement`

### Assignment Syntax

- The whole `a = "value";` thing!
- similar to binary operators but without the looping due to the fact that the assignment operator is right associative.
- the variable `a` is an "l-value", meaning it's not the value of `a` itself that is important here but the location where `a` is stored that is relevant

### Block statements

"behold the grammar":

```text
statement      → exprStmt
               | printStmt
               | block ;

block          → "{" declaration* "}" ;
```

## Questions

- ArrayList?
- Why the generics in `Stmt` type? Don't statements by definition have no return type?
- Who has higher precedence out of statements and declarations?
