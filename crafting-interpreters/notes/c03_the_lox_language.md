# [C3 - The Lox Language](https://craftinginterpreters.com/the-lox-language.htmhttps://craftinginterpreters.com/the-lox-language.htmll)

## Challenges

> This informal introduction leaves a lot unspecified. List several open questions you have about the language’s syntax and semantics. What do you think the answers should be?

- keyword arguments? not implementing those at all I suppose
- what are the downsides of having the primitive types not implemented as classes?
- importing things from other files? modules etc.?
  - this seems to be fairly uniform across the c-style languages I've encountered, perhaps just sticking with python's absolute imports to keep things minimal?
- no switch statement stuff - I like this for now, as I personally don't find they add much value against lots of `if` statements
