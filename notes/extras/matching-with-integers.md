# Matching with integers

## Some things to keep in mind

Here's a code:

```rust
fn main() {
    let mut count = 0;

    while count < 5 {
        let x = match count {
            0 => 1,
            1 => 2,
            2 => 3,
            3 => 4,
            4 => 5,
        };

        println!("Value of x is: {x}");
        count += 1;
    }
}
```

Running the above code will give us an error,

```zsh
error[E0004]: non-exhaustive patterns: `i32::MIN..=-1_i32` and `5_i32..=i32::MAX` not covered
 --> let-match.rs:5:23
  |
5 |         let x = match count {
  |                       ^^^^^ patterns `i32::MIN..=-1_i32` and `5_i32..=i32::MAX` not covered
  |
  = note: the matched value is of type `i32`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern, a match arm with multiple or-patterns as shown, or multiple match arms
  |
10~             4 => 5,
11~             i32::MIN..=-1_i32 | 5_i32..=i32::MAX => todo!(),
  |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0004`.
```

This is because a **`match`** statement checks that its arms cover all the possible cases that the thing being matched can have so, since our code won't encounter anything values from 0 to 4, we can just use a **`match all (_)`** to match any remaining value and avoid the error. The compiler suggests a much more exhaustive solution but that is not needed here.

```rust
fn main() {
    let mut count = 0;

    while count < 5 {
        let x = match count {
            0 => 1,
            1 => 2,
            2 => 3,
            3 => 4,
            4 => 5,
            _ => 5,
        };

        println!("Value of x is: {x}");
        count += 1;
    }
}
```

This executes as we expect it to.

```zsh
Value of x is: 1
Value of x is: 2
Value of x is: 3
Value of x is: 4
Value of x is: 5
```

---
