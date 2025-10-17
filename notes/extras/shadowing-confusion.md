# Shadowing confusion

Look at this code,

```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = &s1;

    let s1 = String::from("Nooooo");

    println!("{s2}");
    println!("{s1}");
}
```

The output of this program is,

```zsh
Hello
Nooooo
```

But I assumed that since **`s2`** has borrowed **`s1`**, when we changed **`s1`** after the borrow, **`s2`** should have had "Nooooo" instead of "Hello". But the line

```rust
let s1 = String::from("Nooooo");
```

shadows **`s1`** that is, creates a new variable with the same name and data that remains the owner of the data "Nooooo" till the end of its scope (the function **`main()`**) but the old **`s1`** still exists and **`s2`** still has reference to it. Therefore when we try to print **`s2`**, we get "Hello" and "Nooooo" if we try to print **`s1`**.

---
