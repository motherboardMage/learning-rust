# References and Borrowing

## Reference

A *reference* is like a pointer that stores an address so we can access the value stored at the address without transferring ownership. Unlike a pointer, a *reference* is guaranteed to point to a valid value of a particular type for the *lifetime* of that *reference*.

Here's a **`calculate_length`** function with *references* instead,

```rust
fn main() {
    let s = String::from("Hello, World!");

    let len = calculate_length(&s);

    println!("String {s} is {len} characters long");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

This code works as **`s`** is dropped after the **`calculate_length`** function ends, but since **`s`** does not have *ownership* of the original **`String`**, it is not dropped after the function ends. This process of creating *references* is called *borrowing*.

---

## Mutable References

These allow other parts of code to modify a value without taking *ownership* of it. There can only be one *mutable* reference or any number of *immutable* references to a value at a given time in Rust. Here's an example,

```rust
fn main() {
    let mut s = String::from("Hello");

    add_world(&mut s);

    println!("{s}");
}

fn add_world(input: &mut String) {
    input.push_str(", world!");
}
```

These rules about *mutable* and *immutable* references prevent data races. They occur when,

1. Two or more pointers access the same data at the same time.
2. At least one of the pointers is being used to write to the data.
3. There's no mechanism being used to synchronize access to the data.

Rust prevents data races at compile time by simply refusing to compile in such conditions.

**Note: A reference's scope starts from where it is introduced and continues through till the last time it is used. For example:**

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{r1} and {r2}");
// Variables r1 and r2 will not be used after this point.

let r3 = &mut s; // no problem
println!("{r3}");
```

In the above code, since the references **`r1`** and **`r2`** aren't used after the first **`println!()`**, their scope ends there and so the third reference **`r3`** to the string is valid.

---

## Dangling References

The Rust compiler guarantees that there are no *dangling references*. A *dangling reference* would be a reference that would point to a freed memory location, or some place which does not have any data associated with it. The compiler ensures that the data would never go out of scope before the reference to the data does.

Example code,

```rust
fn main() {
    let s = dangle();

    println!("{s}");
}

fn dangle() -> &String {
    let s = String::from("Hello");

    &s;
}
```

This, on compiling gives,

```rust
error[E0106]: missing lifetime specifier
 --> dangling.rs:7:16
  |
7 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime, but this is uncommon unless you're returning a borrowed value from a `const` or a `static`
  |
7 | fn dangle() -> &'static String {
  |                 +++++++
help: instead, you are more likely to want to return an owned value
  |
7 - fn dangle() -> &String {
7 + fn dangle() -> String {
  |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0106`.
```

As you can see, the compiler detects that **`s`** contains a reference to a value that does not exist anymore (The other **`s`** in the separate function went out of scope and was dropped so its value does not exist in the memory anymore).

The solution here would be to return the **`String`** directly.

```rust
fn main() {
    let s = dangle();

    println!("{s}");
}

fn dangle() -> &String {
    let s = String::from("Hello");

    s;
}
```

And this works.

---
