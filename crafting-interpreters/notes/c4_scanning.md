# [C4 - Scanning](https://craftinginterpreters.com/scanning.html)

## Notes

- REPL = Read, Eval, Print, Loop! Actually very simple turns out :)
- The number of keyword types is super small! All stored in [`TokenType.java`](../com/craftinginterpreters/lox/TokenType.java)
- Not using indentation (like python) makes the scanner very simple hey
- Maximal crunch - scanner/parser takes the word that has the largest number of matching characters, i.e. `orchid` matches rather than the keyword `or` when `or` is found

### End of first pass Qs

- difference between expression and statement? Still not clear on this
- how does that block `static {}` at the top of the object work? Why use that rather than just defining it on declaration?
