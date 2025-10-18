# Control flow

## if, else if, else

In Rust, **`if`** and **`else if`** are expressions that evaluate to something. We'll see that later. There are no brackets around the conditions, unlike C.

```rust
if condition {
    code;
}
else if condition {
    code;
}
else condition {
    code;
}
```

Also, the condition inside **`if`** and **`else if`** must be a bool. Unlike C, just using a number there and expecting it to work based on if the number is positive or not won't work,

```rust
let num = 5;
if num {        // this is incorrect
    code;
} else {
    code;
}
```

Since **`if`** and **`else if`** are expressions, they can be used with **`let`** statements like this,

```rust
let condition = true;

let x = if condition {5} else {3};  // x = 5
```

It can go deep like,

```rust
let mut selector = 0;

while selector <= 3 {
    let x = if selector == 0 {
        2
    } else if selector == 1 {
        3
    } else if selector == 2 {
        4
    } else {
        5
    };

    println!("Value of x is: {x}");
    selector += 1;
}
```

All the types for the results of the evaluation of **`if, else if and else`** statements must be of the same type.

---

## Loops

There are **three** kinds of *loops* in Rust,

1. Using the **`loop`** keyword, this runs the loop indefinitely unless stopped explicitly.

```rust
loop {
    println!("Again!");
}
```

We may use **`break`** to break out of such a loop like,

```rust
let mut counter = 0;

let x = {
    loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
}
```

---

### Loop labels

We can use loop labels to disambiguate between different loops, say, when we are inside a nested loop. Loop labels start with **`'`** for example, while we are in a **`nested loop`**,

```rust
'outside loop {
    loop {
        if condition {
            break 'outside;
        }
    }
}
```

---

## While loop

You know the drill, nothing special.

```rust
while condition {
    code;
}
```

---

## For loop

Since they work with anything that implements the [iterator trait](https://doc.rust-lang.org/std/iter/trait.Iterator.html), there are a lot of ways to loop using the **`for`** loop in Rust.

We can use them with ranges (most common method),

```rust
// exclusive range
for i in 0..5 {
    println!("{i}");
}

// inclusive range
for i in 0..=5 {
    println!("{i}");
}

// reverse iteration
for i in (0..=5).rev() {
    println!("{i}");
}

// with step
for i in (0..8).step_by(2) {
    println!("{i}");
}
```

or, iterate directly over collections,

```rust
let a = [10, 20, 30, 40, 50];

for i in a {
    println!("{i}");
}
```

Iterating over strings requires special attention as they do not include the **`iterator`** trait.

```rust
let text = "Hello world";

for i in text {
    println!("{i}");
}
```

This gives an error message,

```zsh
error[E0599]: no method named `char` found for reference `&str` in the current scope
 --> string_iter.rs:4:20
  |
4 |     for ch in text.char() {
  |                    ^^^^
  |
help: there is a method `chars` with a similar name
  |
4 |     for ch in text.chars() {
  |                        +

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0599`.
```

To fix this, use the **`.chars()`** or **`.bytes()`** methods.

```rust
let text = "Hello World";

    for ch in text.chars() {
        println!("{ch}");
    }
```

This works as intended.

```zsh
H
e
l
l
o

W
o
r
l
d
```

There are more methods to use **`for`** loops but I'm sure we'll discover it on the way.

---
