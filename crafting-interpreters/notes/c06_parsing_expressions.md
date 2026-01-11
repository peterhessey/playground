# [C6 - Parsing Expressions](https://craftinginterpreters.com/parsing-expressions.html)

## Notes

- *Recusrive descent parsing* - the name of the parser we implemented in Java
  - Considered a top-down parser
  - These have problems with left-recursive grammar rules, as they get stuck in infinite loops. The solution for this is to replace left-recusrive rules with iterative rules, e.g.:
  - Other parsing techniques (bottom-up parsers) don't have this problem as they consume in the opposite direction.

```text
factor         → factor ( "/" | "*" ) unary
               | unary ;
```

Becomes:

```text
factor         → unary ( ( "/" | "*" ) unary )* ;
```

## Questions

- Why is left-recursion a problem? - explained above
- What is a "simple sentinel class"? - just a term for a class/object that replaces generic errors and/or `null` / `None` values
