# Shadowing vs Reassigning

In a shadowing example, I tried to execute this code:

```rust
fn main() {
    let x = 5;
    const PI: f64 = 3.14;

    {
        let x = 7.6;
        let x *= 2;
        println!("Value of x in inner scope: {x}");

        println!("Assume pi is 4");
        const PI: u32 = 4;
        println!("Value of pi in inner scope is: {PI}");
    }

    println!("Value of x: {x}");
    println!("Value of pi is: {PI}");
}
```

However, this has many issues. Trying to compile the code, we get this error message:

```zsh
error: can't reassign to an uninitialized variable
 --> variables.rs:7:15
  |
7 |         let x *= 2;
  |               ^^
  |
  = help: if you meant to overwrite, remove the `let` binding
help: initialize the variable
  |
7 -         let x *= 2;
7 +         let x = 2;
  |

warning: unused variable: `x`
 --> variables.rs:6:13
  |
6 |         let x = 7.6;
  |             ^ help: if this is intentional, prefix it with an underscore: `_x`
  |
  = note: `#[warn(unused_variables)]` on by default

error: aborting due to 1 previous error; 1 warning emitted
```

Let's go over the problem. I wrote **`let x *= 2`**. This is a problem as **`*=`** is not used for *initialising* a variable (which, shadowing is) but to *reassign* an already *initialised* variable, a value of it multiplied by something else. To fix it, we remove the **`*=`** and replace it with **`= x * 2`**.

```rust
//x *= 2;
x = x * 2;
```

However this still gives us an error.

```zsh
error[E0277]: cannot multiply `{float}` by `{integer}`
 --> variables.rs:7:19
  |
7 |         let x = x * 2;
  |                   ^ no implementation for `{float} * {integer}`
  |
  = help: the trait `Mul<{integer}>` is not implemented for `{float}`
  = help: the following other types implement trait `Mul<Rhs>`:
            `&f128` implements `Mul<f128>`
            `&f128` implements `Mul`
            `&f16` implements `Mul<f16>`
            `&f16` implements `Mul`
            `&f32` implements `Mul<f32>`
            `&f32` implements `Mul`
            `&f64` implements `Mul<f64>`
            `&f64` implements `Mul`
          and 57 others

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0277`.
```

Now why does this happen? Well simple, the compiler explains it to us. We cannot multiply two different types here as multiplication traits for different number types are seemingly defined differently.

So we either need to explicitly typecast variables or just convert the constant to a float.

```rust
let x = x * 2f64;   // Now this runs without issues
```

---
