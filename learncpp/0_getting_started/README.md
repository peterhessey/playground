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

### Warnings

- Turn warning levels up while learning c++
- With clang, this can be done by adding `-Wall -Weffc++ -Wextra -Wconversion -Wsign-conversion` flags to all commands
- To treat warnings as errors in all cases, add the following flag: `-Werror`

### C+= version

- To configure C++ version using `g++`, add the `-std=c++<version>` flag when compiling.
- The tutorials recommend C++17 so I'll set up an alias for that
