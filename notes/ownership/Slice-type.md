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
let w = &s[7..12];

println!("--{h}--");
println!("--{w}--");
```

This program will output:

```zsh
--Hello--
--world--
```

So the syntax for **`string slices`** is something like this,

```rust
&string_name[starting_of_slice..end_of_slice];
```

The **`end_of_slice`** is the index where the **`String`** will be sliced so, it must always be 1 more than the last index of the piece what we wish to separate.

Rather than referring to the entire **`String`**, a **`string slice`** has two fields:

1. A reference to the start of the *slice*
2. The length of the slice

If we want the start or end of a **`String`** to be included in our slice, we can remove the corresponding value around the two dots **`..`**.

```rust
&s[..i]     // Slices from first to index i
&s[i..]     // Slices from index i to last
&s[i..k]    // Slices from index i to k
&s[..]      // Slices the entire string
```

Now, if we write the same first word program using *string slice*, we get

```rust
fn main() {
    let mut s = String::from("Hello world!");

    let first = find_first_word(&s);

    s.clear();

    println!("First word was {first}");
}

fn find_first_word(input: &str) -> &str {
    for (i, ch) in input.chars().enumerate() {
        if ch == ' ' {
            return &input[..i];
        }
    }
    &input[..]
}
```

The previous program with the indexing approach also had a logical issue that the index was invalid after the **`String`** was cleared but we didn't get an error. If that issue created a bug later on in a larger program, it would be very difficult to track. Here although, if we compile the program, we get an error,

```zsh
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> first-slice.rs:6:5
  |
4 |     let first = find_first_word(&s);
  |                                 -- immutable borrow occurs here
5 |
6 |     s.clear();
  |     ^^^^^^^^^ mutable borrow occurs here
7 |
8 |     println!("First word was {first}");
  |                               ----- immutable borrow later used here

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0502`.
```

Clearly, this says that we cannot take a mutable borrow of **`s`** as it has been borrowed before in **`find_first_word`**. This makes it much easier to debug our code. Now this program can be fixed easily by moving the **`println!()`** line above the **`s.clear()`** line.

---

## String Literals as Slices

We know **`String literals`** are stored directly inside the binary. The type of a **`String literal`** is **`&str`**. It is a slice pointing to a specific location in the binary. This is also why they are *immutable* as **`&str`** is an *immutable reference*.

---

## Other Slices

Just like **`string slices`**, there is the more general **`slice`** type too. Here is it applied to an array:

```rust
let arr = [1, 2, 3, 4, 5];

let arr_slice = &arr[..3];

assert_eq!(slice, &[1, 2, 3]);
```

This **`slice`** has the type &[i32] and works the same way as other slices by storing a reference to the first element of the **`slice`** and its length. Here, the last element is excluded. So the **`slice`** contains the elements [1, 2, 3].

The last line just verifies if the elements in the **`slice`** are what we expect by creating a temporary array of elements [1, 2, 3] and comparing it to the **`slice`** we created.

---
