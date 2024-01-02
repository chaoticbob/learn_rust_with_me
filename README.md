# Learn Rust With Me

# For C++ Programmers
This is a quick Rust guide for C++ programmers.

# Program Entry Point
Like C/C++ the entry point for a program is the `main` function:
```rust
fn main() {
    println!("hello");
}
```

# Variable Declaration
Variables are declared using the `let` keyword, analogous to C++'s auto keyword. Variable types can be explicitly stated or implicitly deduced from values, expressions, or functions.

## Explicit Type
```rust
let i : i32 = 1;   // i32
let u : u32 = 1;   // u32
let x : f32 = 1.0; // f32
let y : f64 = 1.0; // f64

// -- or --

let i = 1i32; // i32
let u = 1u32; // u32
let x = 1f32; // f32
let y = 1f64; // f64
```

## Deduced Type From Value
```rust
let i = 1;       // i32
let x = 1.0;     // f64
let s = "hello"; // &str
```

## Deduced Type From Expression
```rust
let x = 1.0 as f32; // f32
let y = x + 2.0;    // f32
```

## Deduced Type from Function
Variable types can be deduced from values, expressions, and functions:
```rust
fn add(a i32, b: 32) -> i32 {a + b}

let i = add(1, 2); // i32
```

# Type Strictness
Rust has very strong type requirements and there is no implicit conversion like C/C++. Mixing types will generally result in a compiler error in Rust.
```rust
let x = 1.0 + 2;        // ERROR: cannot add an integer to a float
let y = 1.0 + 2 as f32; // OK
let z = 1.0 + 2f32;     // OK
```

# Return Values
Return types are specified using the `->` operator at the end of a function signature:
```rust
fn add(a: i32, b: 32) -> i32 { a+b }
```

The use of the `return` keyword can be optional. The last statement in the function without a `;` is implicitly the return value. 
```rust
// Implicit return, no ; here
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Explicit return, must have ; when using return keyword
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
```

# Conditionals
`if` and `while` statements in Rust require curly braces but parentheses are optional:
```rust
if cond {}
if cond1 && cond2 {}
if cond1 {}
if (var == true) {}
if (var1 == true) || (var2 == true) {}
if ((var1 == true) || (var2 == true)) {}

while cond {}
while cond1 && cond2 {}
while cond1 {}
while (var == true) {}
while (var1 == true) || (var2 == true) {}
while ((var1 == true) || (var2 == true)) {}
```

# Ternary Operator
There is not a ternary operator, but the equivalent expression can be written using `if/else`:
```rust
let x = if cond { true } else { false }
```

Notice that there is not a `;` in the branches of `if/else`.
