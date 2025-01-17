# Chapter 8 - Control Flow

## 8.2/3 - `if` statements

- Just use blocks for your if statements unless they're really really simple.
- Dangling `else` problem is caused by poor code styling - avoid nest `if` statements and if you do have to, use blocks to make it explicitly clear.
- Be super careful not to "terminate" if statements with a semi-colon, this is effectively a null statement and means whatever comes after will always execute instead.

## 8.5/6 `switch` stamements

- expression inside of `switch(...)` must evaluate to an integral type (bool or int), or an enum (covered later). Strings, floating point values can't be used here.
- Every statement after the first one is executed unless you specify a `return` or `break`. This is called "fallthrough"
- Stacking `case` labels together can be used instead of lots of OR (`||`) statements in an if statement.
- Avoid initialising variables within switch statements, and if you have to, use an explicit block for it.

## 8.6/7 - `goto` statements

Please just don't

## 8.11 - `break` and `continue`

- All the same behaviour and standards as python, nice.

## 8.12 - Halts

- `std::exit()` is how `main` terminates the program, and you can too!
- included via the `cstdlib` header
- local variables are *not* cleaned up by explicit calls to this.
- generally you don't wanna be tangling with these things anyway. But good to know that they exist!
