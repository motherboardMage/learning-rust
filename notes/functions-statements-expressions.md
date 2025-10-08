# Functions, Statements and Expressions

This is a statement

```rust
let x = 4;
```

and these are expressions,

```rust
x = 5
8
```

So what is the difference?

**Statements** are instructions that perform some action and do not return a value.
**Expressions** evaluate to a resultant value.

```rust
                        // This returns an error since "let y = 6"
let x = (let y = 6);    // is a statement and does not return a value
                        // hence, there is nothing for x to bind to
```

In the above code, **`6`** is an expression in  **`let y = 6`** that evaluates to the value to **`6`** which is bound to **`y`**.

Another case of an expression is,

```rust
fn main() {
    let y = {
        let x = 5;
        x + 1
    };
}
```

Here, **`y`** is bound to the value **`6`**, as in the inner scope,

```rust
{
    let x = 5;
    x + 1
}
```

**`x + 1`** is just an expression that is evaluated to the value **`6`** and is returned as the scope ends. **Notice that expressions do not end in semicolons**.

## Functions that return a value

In Rust, **`functions`** return the last expression by default but we can return early by using the **`return`** keyword and specifiying the value. **`Functions`** that return a value must also have a **`type specifier`** after an arrow **`->`**. For example,

```rust
fn five() -> u8 {
    5
}
```
