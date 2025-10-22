# The Slice Type

## Slices

Slices let us access a *contiguous* sequence of members in a collection. A slice is a kind of *reference* so, it does not take *Ownership*.

To demonstrate the importance of slices, here's a program to find the first word of a string only separated by spaces:

```rust
fn main() {
    let s = String::from("Hello world!");

    let first = find_first_word(&s);
    println!("First word ends at index {first}");
}

fn find_first_word(input: &String) -> usize {
    for (i, ch) in input.chars().enumerate() {
        if ch == ' ' {
            return i;
        }
    }
    input.len()
}
```

Let's break it down. The **`find_first_word`** function takes an input **`String`** reference and returns a **`usize`** which is usually a 64 bit unsigned integer. **`usize`** is Rust's standard for indices and sizes. Now the line:

```rust
for (i, ch) in input.chars().enumerate()
```

creates and matches a tuple **`(i, ch)`** to the result of **`input.chars().enumerate()`**. What happens here is, **`chars()`** is a method that returns an *iterator* on input.

In Rust, an *iterator* is a "lazy" structure that gives items one by one, whenever it is asked for a new one. Here we use a **`for`** loop to ask for new items. An iterator correctly iterates over a collection while keeping the size of individual elements of that collection in check. For collections like **`String`**, it is important as **`String`** elements, called *characters* are UTF-8 encoded in Rust which means their sizes can vary from 1 to 4 bytes. If not for **`chars()`**, we may interpret a string byte by byte which may lead to us falling in between a multi-byte character. **`chars()`** ensures reading correct characters.

Now **`enumerate()`** is an *iterator adapter*. It wraps an input iterator and modifies the result. Here, takes the iterator returned by **`chars()`** and bundles it with a counter, producing a tuple. We destructure the tuple using a pattern **`(i, ch)`** and match its values to the tuple produced to store our values.

This program has a flaw. The value in **`first`** is unrelated to the **`String`**. If anything were to happen to the **`String`**, it would still have its value even though the meaning of that value for the **`String`** may have changed. The index stored in **`first`** may not refer to the expected character if such a change occurs.

```rust
let mut s = String::from("Hello");

let first = find_first_word(&s);

s.clear();

println!("First word ends at index {first}");
```

Now here the program reports that the first word in the **`String`** ends at position 5 but in reality, there are no words in the **`String`** as it is now empty. Similarly, if the **`String`** had changed, there would be no guarantee that the index **`first`** would be referring to the end of the correct word or a word at all.

---

## String Slices

**`String slices`** are references to a *contiguous* sequence of characters in a **`String`**.

```rust
let s = String::from("Hello, World");

let h = &s[0..5];
let w = &s[6..11];

println!("--{h}--");
println!("--{w}--");
```
