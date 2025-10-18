# Ownership

Rust's ownership system guarantees memory safety without a garbage collector. It's a set of rules that the compiler follows. It's main purpose is to manage **`heap`** data.

Here are the rules of ownership:

1. Each value in Rust has an owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value is dropped

To understand ownership, we will use the **`String`** type as it is not a type with a fixed size so, unlike some other data types, it cannot simply be pushed onto and popped off the **`stack`**. A **`String`** is stored on the **`heap`** so it will help us to understand how Rust manages and cleans up that data.

```rust
let mut s = String::from("Hello");  // :: allows us to namespace the "from"
                                    // function under the String type

s.push_str(", world!");     // This function concatenates ", world!" 
                            // to the end of the string

println!("{s}");
```

See how a variable can be mutated? If we just had a string literal, it could not be mutated like that. So why not? The difference is in how and where they are allocated in memory.

In case of a string literal, it is hardcoded directly into the final binary so, it is fast and efficient but the **`String`** type used here is a growable piece of text so it has to allocated memory on the **`heap`** unknown at compile time.

In Rust, any memory owned by a variable is freed the instant it goes out of **`scope`**.

```rust
{
    let mut s = String::from("Hello, World!"); // s is valid from here on

    // can do things with s
}   // s goes out of scope, memory is freed
```

When a variable goes out of scope, Rust calls a special function **`drop`**. This function returns the memory that was needed by the String back to the **`allocator`**.

---

## Variables and data interacting with `move`

Look at this code,

```rust
let x = 5;
let y = x;
```

This code pushes the variables onto the **`stack`** since integers are simple data types with a fixed size.

Here's a similar looking code with **`Strings`**,

```rust
let s1 = String::from("Hello");
let _s2 = s1;   // underscore to remove unused variable warning
```

Now this code does something different. **`Strings`** are complex data types. They are made of three parts,

1. A pointer to the memory location in the **`heap`** where the contents of the string are stored
2. Its current length
3. Its capacity

These three are stored on the **`stack`** while the actual **`String`** is stored on the **`heap`**. In the above code, we actually copy the data stored on the **`stack`** into **`s2`**. This means that **`s2`** just becomes a pointer to the string content on the **`heap`**. Although, if we run the code, we get this error,

```zsh
error[E0382]: borrow of moved value: `s1`
 --> ownership.rs:5:16
  |
2 |     let s1 = String::from("Hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let _s2 = s1;
  |               -- value moved here
4 |
5 |     println!("{s1}");
  |                ^^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let _s2 = s1.clone();
  |                 ++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0382`.
```

If s1 and s2 both point to the same data and both of them go out of scope, Rust will have to call **`drop`** on both the **`Strings`**. This would have caused a **`double free`** which is a memory corruption bug.

When only the **`stack`** data of a variable is copied and not the **`heap`** data, it is called a **`shallow copy`**. When we try to shallow copy a **`String`**, **`Rust`** invalidates the old variable s1 that is, the **`ownership`** of the value stored in **`s1`** is moved to **`s2`** (we say, **`s1`** moved to **`s2`**). So instead of being called a shallow copy, it is called a **`move`**. The resultant error is because of the second rule of ownership in Rust, that there can be only one owner of a value at a time. Since **`s1`** does not own the value it did anymore and **`println!()`** tries to **`borrow`** the value from it.

A **`borrow`** is a reference to a value that doesn't take **`ownership`** of it. If we instead had written the code like this,

```rust
let s1 = String::from("Hello");
let s2 = &s1;

let s1 = String::from("Nooooo");

println!("{s2}");
println!("{s1}");
```

Then the compiler would not throw errors and the code will run successfully as adding an **`&`** before **`s1`** makes the it an **`immutable borrow`**. This gives **`s2`** read only access to the value stored in **`s1`**. There can be any number of **`immutable borrows`** of a value at a time but only one **`mutable borrow`**.

```rust
let s2 = &mut s1;   // a mutable borrow
```

This way, **`s2`** can modify the data stored at **`s1`**.

---

## Scope and Assignment

The above rules also apply to the values being stored. Look at this code,

```rust
let mut s1 = String::from("Hello");
s1 = String::from("Nooooo");

println!("{s1}");
```

What happens in this case is that after we modify **`s1`** to contain "Nooooo", there is no reference to the old value on the heap, so Rust immediately calls **`drop`** on the old value as it is considered out of scope.

---

## Variables and Data Interacting with Clone

If we instead want to make a **`deep copy`** of the string, we must call the **`clone()`** method on **`s1`**.

```rust
let s1 = String::from("Hello");
let s2 = s1.clone();

println!("{s1}");
println!("{s2}");
```

This works as expected as **`s2`** has now copied both the stack data and heap data of **`s1`**.

---

## Stack only data

Variables that are stored on the stack that is, which have known sizes at compile time implements the **`Copy`** trait. This means, the data in these variables can be copied from one to the other while keeping the original intact. Rust won't let us annotate a type with **`Copy`** if it or, any of its parts implement the **`Drop`** trait.

---

## Ownership and Functions

Look at this program

```rust
fn main() {
    let s1 = String::from("Hello");

    println!("{s1}");

    take_ownership(s1); // Ownership of s1 is passed to the function
}

fn take_ownership(some_string: String) {
    println!("Took ownership of {some_string}");
} // some_string is dropped
```

This code gives an error,

```zsh
error[E0382]: borrow of moved value: `s1`
  --> ownership.rs:15:16
   |
 2 |     let s1 = String::from("Hello");
   |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
...
10 |     takes_ownership(s1);
   |                     -- value moved here
...
15 |     println!("{s1}");
   |                ^^ value borrowed here after move
   |
note: consider changing this parameter type in function `takes_ownership` to borrow instead if owning the value isn't necessary
  --> ownership.rs:19:31
   |
19 | fn takes_ownership(something: String) {
   |    ---------------            ^^^^^^ this parameter takes ownership of the value
   |    |
   |    in this function
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
   |
10 |     takes_ownership(s1.clone());
   |                       ++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0382`.
```

This is because **`take_ownership(s1)`** moves **`s1`** to the function and after the function finishes executing, **`s1`** is dropped and cannot be borrowed by **`println!()`**. This can be fixed by instead borrowing **`s1`**.

```rust
fn main() {
    let s1 = String::from("Hello");

    println!("{s1}");

    does_not_take_ownership(&s1); // s1 is borrowed by the function
}

fn does_not_take_ownership(some_string: &String) {
    println!("Didn't take ownership of {some_string}");
} // some_string is not dropped as it was not moved
```

Now, the code compiles to

```zsh
Hello
Didn't take ownership of Hello
```

A similar program with integers works like this

```rust
fn main() {
    let x = 5; // x comes into scope

    println!("{x}");

    copy_variable(x); // x is borrowed by copy_variable()

    println!("{x}");
}

fn copy_variable(x: i32) {
    println!("Copied {x}");
} // x is not dropped
```

This program outputs,

```zsh
5
Copied 5
5
```

because i32 implements the Copt trait, x can be borrowed by the **`copy_variable()`** function without moving **`ownership`**

---

## Return values and scope

Returned values can also transfer ownership. For example,

```rust
fn main() {
    let s = gives_ownership();

    println!("{s}");
}

fn gives_ownership() -> String {
    let some_string = String::from("Owned by s");

    some_string
}
```

Here, the function **`gives_ownership()`** creates the string **`some_string`** and moves it to **`s`**.

Another example,

```rust
fn main() {
    let s = String::from("Hello");

    let s1 = takes_and_gives(s);
}

fn takes_and_gives(input_str: String) -> String {
    input_str
}
```

Here, function **`takes_and_gives()`** moves **`s`** and then *returns* it to **`s1`**. This finally *moves* **`s`** to **`s1`**.

Functions can also return multiple values as a tuple,

```rust
fn main() {
    let s = String::from("Hello");

    let (s1, len) = calculate_length(s1);

    println!("Length of s1 is: {len}");
}

fn calculate_len(string: String) -> (String, usize) {
    let length = string.len();

    (string, length)
}
```

---
