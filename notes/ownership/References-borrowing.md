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
