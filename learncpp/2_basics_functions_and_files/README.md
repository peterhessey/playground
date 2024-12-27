# Chapter 2 - Basics: Functions and Files

## 2.1 - Functions

NESTED FUNCTIONS ARE NOT LEGAL AW YEAH

### `int main()`

- `main()` cannot be called directly
- `main()` cannot have a non-`int` return type
- `main()` is not necessarily called first at the start of the program - global variables are initialised before `main()` is called, including any functions that the initialisation of those global variables call.

Some tips of the return types for `main()`:

```c++
#include <cstdlib> // for EXIT_SUCCESS and EXIT_FAILURE

int main()
{
    return EXIT_SUCCESS;  // can also be 0
    // or
    return EXIT_FAILURE;
}
```

### Not returning from returning functions

- This will cause undefined behaviour, although the code will still compile.
- Reminded that `-Werror` addresses this, however compilers may miss this in complicated scenarios!

## 2.7 - Forward Declarations

- Instead of re-ordering functions (e.g. always putting `main()` at the bottom of the file) one ca use *forward declarations*
- These tell the compiler about the existence of functions/variables before they're actually defined, allowing one to use the ordering one likes for functions (at the cost of extra maintenance!)
- For functions, this can also be called a "function prototype""

## 2.10 - Intro to the preprocessor

- The preprocessor runs before the compiler, making changes to the code such as removing comments
- Statements that start with a `#` are treated as *directives* which are handled by the preprocessor
  - e.g. the `#include` directive copies the contents of the included file into the file where the directive is placed.
- There's also the `#define` directive, used for text substitution
- `#ifdef` and `#ifndef` can be used to selectively compile or not block of code depending on if `#define` blocks

> Object-like macros (used for text substitution) are no longer necessary for defining global literals and should be avoided.

## 2.11 Header files

- These are how we avoid the repeated use of forward declarations, which do not scale at all.
- Essentially they are used to give the forward definitions of all the functions in a source file so that it can then be included without the forward declarations!
- the header file must be included in both the includer and the includee.
- for now, do not put functions definitions in header files, it will cause problems.
- `.cpp` files should not be included. While this will work, it is bad practice for many reasons such as namespace issues.

## 2.12 - Header Guards

- An interesting, hands-on mechanism for preventing things defined in header files being included more than once.
- Can be done with the `#define` directive (see example) or `#pragma once` directive (not in the C++ standard but commonly used by most modern compilers)
