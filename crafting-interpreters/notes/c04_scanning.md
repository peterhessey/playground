# [C4 - Scanning](https://craftinginterpreters.com/scanning.html)

## Notes

- REPL = Read, Eval, Print, Loop! Actually very simple turns out :)
- The number of keyword types is super small! All stored in [`TokenType.java`](../com/craftinginterpreters/lox/TokenType.java)
- Not using indentation (like python) makes the scanner very simple hey
- Maximal crunch - scanner/parser takes the word that has the largest number of matching characters, i.e. `orchid` matches rather than the keyword `or` when `or` is found

### End of first pass Qs

- difference between expression and statement? Still not clear on this
  - expressions return values, statements do not and instead change state. I.e. *print "hello world"* in lox does not evaluate to a value, but instead changes some state by printing stuff to stdout. Some languages do not have statements at all.
- how does that block `static {}` at the top of the Scanner object work? Why use that rather than just defining it on declaration?
  - this is a "static initialization block" and is run when Java loads the class (before any instances are created)
  - the `final` keyword means the reference is immutable, so `keywords` can't point to a different map, but the contents of the map are mutable and can change!
