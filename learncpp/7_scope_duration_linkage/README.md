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
