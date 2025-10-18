# Compound Types in Rust

## Tuples

This is a compound data type that can store multiple values of different types. Tuples are of fixed size. There are several ways of declaring and using them.

```rust
// We can use type annotations
let tup = (500, "has one ball", 4.5);
let tup: (u32, &'static str, f32) = (500, "has one ball", 4.5);
```

To use the values stored in it, we can either declare and initialise variables in one line:

```rust
let (x, y, z) = tup;
```

or do it on separate lines,

```rust
let x = tup.0;
let y = tup.1;
let z = tup.2;
```

Mutable tuples can also have their values changed.

---

## Arrays

We know what arrays are. They allocate contiguous blocks of memory to same type of elements on the **stack**. They can be declared and initialised in a couple of ways in rust;

```rust
let a = {1, 2, 3, 4, 5};

// With type annotation
let a: [i32; 5] = {1, 2, 3, 4, 5};

// Declare and initialse array with "size" of "value"
// let identifier = [value; size];
let a = [3; 5];
```

---
