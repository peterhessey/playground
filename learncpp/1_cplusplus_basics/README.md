# Chapter 1 - C++ Basics

## Important author's note

> One more thing: You may be thinking, “C++ has so many rules and concepts. How do I remember all of this stuff?”.
>
> Short answer: You don’t. C++ is one part using what you know, and two parts looking up how to do the rest.
>
> As you read through this site for the first time, focus less on memorizing specifics, and more on understanding what’s possible. Then, when you have a need to implement something in a program you’re writing, you can come back here (or to a reference site) and refresh yourself on how to do so.
>
## Initialisation

Who knew there were so many options?

```c++
int a;         // default-initialization (no initializer)

// Traditional initialization forms:
int b = 5;     // copy-initialization (initial value after equals sign)
int c ( 6 );   // direct-initialization (initial value in parenthesis)

// Modern initialization forms (preferred):
int d { 7 };   // direct-list-initialization (initial value in braces)
int e {};      // value-initialization (empty braces)`
```

> direct-list-initialization is the preferred option for modern C++ (although it is very similar to copy-initialization for C++17 and after)

### list-initialization

```c++
int width { 5 };    // direct-list-initialization of initial value 5 into variable width (preferred)
int height = { 6 }; // copy-list-initialization of initial value 6 into variable height (rarely used)
```

This form of initialization disallows narrowing:

```c++
int main()
{
    // An integer can only hold non-fractional values
    int w1 { 4.5 }; // compile error: list init does not allow narrowing conversion of 4.5 to 4

    int w2 = 4.5;   // compiles: copy-init initializes width with 4
    int w3 (4.5);    // compiles: direct-init initializes width with 4

    return 0;
}
```

> Best practice: initialize on creation.

## `iostream`

- `cout` = "character out"
- outputs

### endline

- `std::endl` also flushes the output buffer, which can be slower / less efficient than allowing c++ to flush the buffer in its own time, which can be done by using `\n` instead :)

### characters

- single quotes are used for chars, double for strings
- that being said, authors reccomend using double quotes for all output text (technical reasons they explain later)

## Undefined behaviour

- Uninitialized variables don't not have values, they just haven't been *assigned* values!!
- They'll still get a memory address and initially have the value of whatever is being stored in memory at that address currently.
- Don't do this :)
