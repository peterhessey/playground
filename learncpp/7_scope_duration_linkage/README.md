# Chapter 7 - Scope, Duration and Linkage

## 7.2 - User-Defined Namespaces

- `::` is the `scope resolution operator`, used for identifying things within namespaces (such as the `std` namespace)
  - using it with no name prefix searches in the global namespace, useful if operating inside a namespace that has a collision!
- gotta put namespaces inside of the header files too baby
- namespaces can be declared in multiple places and all get added together nicely.
- namespaces not used for small things, useful for shared code and larger projects though

## 7.3 - Local variables

- variable **scope** determines when that variable is accessible in the code. Local variables have **block scope**
- variable **duration** determines the rules for when a variable is created and destroyed. This is at the point of definition and the end of the block for local variables.
- variable **lifetime** is the actual time between a variable's creation and destruction.

## 7.4 - Global variables

- defined after the `#includes`, just like in python.
- should be `const`, but don't have to be.
- their advice on style is to prefix with `g_` to indicate globality.
- "global" refers to the file, not the whole program.

## 7.5 - Variable shadowing

- The same as in python, defining local variables in blocks that have the same name as variables in outer blocks that the current block has access to will temporarily "hide" the outer variables.
- This applies to global variables too.
- Highly discouraged (for obvious reasons).

## 7.6 - Internal Linkage

- A **translation unit** is the output from the preprocessor after it has processed a code file (this is what is passed to the compiler).
- Global variables and function identifiers can have "internal" or "external" linkage.
- Internal linkage means that the variable/function can only be seen and used within the translation unit in which it is defined.
- Variables and functions can be given internal linkage by defining them with the `static` keyword.
- `const` and `constexpr` globals have internal linkage by default, functions and global non-const variables do not.
- This seems similar to the concept of "private" functions / variables in Java/C#.

## 7.7 - External Linkage

- The inverse of the above, variables and functions with external linkage are accessible from other translation units.
- The linker will connect these together (after compilation ?).
- Functions and non-const variables have external linkage by default.
- `const` and `constexpr` can be given external linkage with the `extern` keyword.
- To use external variables or consts, a forward declaration must be made. This forward declaration must use the `extern` keyword but with no initialisation value:

```c++
// Global variable forward declarations (extern w/ no initializer):
extern int g_y;                 // forward declaration for non-constant global variable
extern const int g_y;           // forward declaration for const global variable
extern constexpr int g_y;       // not allowed: constexpr variables can't be forward declared

// External global variable definitions (no extern)
int g_x;                        // defines non-initialized external global variable (zero initialized by default)
int g_x { 1 };                  // defines initialized external global variable

// External const global variable definitions (extern w/ initializer)
extern const int g_x { 2 };     // defines initialized const external global variable
extern constexpr int g_x { 3 }; // defines initialized constexpr external global variable
```

## 7.11 - Static local variables

- This kinda crazy. Use the `static` keyword on local variables to change their duration to static as well (same as global variables) such that they are created and destroyed at program start and end respectively.
- Primarily can be used to avoid re-initialising expensive objects on every function call (handy!) without exposing them as globals.
- Avoid using this behaviour to control logical flow.
- Static variables are often prefixed with `s_`.

## 7.13 - `using namespace`

- Just don't, basically

## 7.14 - Unnamed and inline namespaces

- Unnamed namespaces put everything into the parent namespace.
- They also treat all identifiers within the namespace as if they have internal linkage, essentially making all functions `static`. This is useful if you want to make lots of related things static (but not for much else?)
- `inline` namespaces let you set a "default" namespace (the inline one), usually used for versioned stuff
