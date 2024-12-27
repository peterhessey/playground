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
