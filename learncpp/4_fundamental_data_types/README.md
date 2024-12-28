# Chapter 4 - Fundamental Data Types

## 4.5 - Unsigned Integers

- You already know about these, the general advice is avoid them unless absolutely necessary!
- Instances where they *have* to be used:
  - Bit manipulation
  - Array indexing
  - Memory limited contexts

## 4.12 - Type conversion

- Implicit conversion happens where it is safe to do so (e.g. from int to double, but not the other way around).
- Cases where information would be lost or behaviour changed will result in warnings or errors.
- To do the type conversion explicitly, `static_cast<type>(variable)` can be used, e.g. `static_cast<int>(double)`
  - (parametrized types tend to sit between angled brackets like this in C++)
