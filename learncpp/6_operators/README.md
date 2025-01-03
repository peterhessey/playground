# Chapter 6 - Operators

## 6.1 - Operator Precedence

- Function arguments evaluate in an arbitrary order, do *not* write code that depends on the order of evaluation of function args.

## 6.3 - Remainder and exponentiation

- Prefer to compare the rest of the remainder operator (%) against 0 where possible to be robust to negative values.
- To do exponent operations, use the `<cmath>` header and `std::pow()` through that. However, this takes `double` inputs so use with caution when running exponentiation on integers (best to manually implement yourself apparently?)

## 6.4 - Increment/Decrement

- Prefix inc/decrement (e.g. `++x`) increments the operand and evaluates to the value of the operand.
- Postfix inc/decrement (e.g. `x++`) makes a copy of the operand, increments the original, and evaluates to the value of the copy (e.g. if x = 6 then y = x++ sets y equal to 6, and the value of x is now 7)
- As a result, prefer the prefix versions. They are more performant and less likely to cause surprises.
- Futhermore, don't use a variable that has a side effect applied to it more than once in a given statement (to avoid undefined behaviour, e.g. `x + ++x` is ambiguous)

## 6.6 - The conditional operator

- The only ternary operator in C++ (I think).
- Very low priority in operand evaluation, always wrap in parantheses if you want to avoid pain!
- Both expressions must have the same type (or a defined way of converting between them).
- Hard to read for the most part, only use in simple scenarios.

## 6.7 - Relational operators and floating-point comparison

- Relational operators work much the same as python, no problems there.
- Interesting section on floating point comparison:
  - Nearby floating point values (especially calculated ones) should be used with caution with relational operators.
  - Use relative and/or absolute epsilons to calculate *near enough* values instead.
