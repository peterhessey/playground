# Chapter 0 Notes

## Compiling  

- Using `g++` to compile currently
- Two main options for compiling, "release" and "debug"
- Debug is for debugging (duh), best for testing while writing code but produces larger executables. Executed by adding the `-ggdb` flag when running `g++`
- Release is the opposite, produces lighter executables. Executed by adding `-O2` (the letter "O" for optimisation) and `-DNDEBUG` flags

### Compiler extensions

- These are extra options that compilers use to "improve" the language such as compatibility with other versions of the language.
- These can be confusing to new learners.
- Turn them off by adding the `-pedantic-errors` flag
