# Some Common Programming Concepts

## Variables and mutability

Variables are immutable **by default**

```rust
// This will give a compiler error
let x = 5;
x = 6;

// This works 
let mut y = 5;
y = 6;
```

For values that will stay constant, use **`const`**. Constants must have their types annotated firsthand.

```rust
const PI = 3.14;                   // wrong, data type must be annotated
const KGS_IN_A_TON: u16 = 1000;    // this is correct
```

---

## Shadowing

This allows another variable to take any uses of a variable's name to itself while in the scope. Constants can also be shadowed but use of **`const`** is necessary each time.

```rust
fn main() {
    let x = 5;
    const PI: f64 = 3.14;

    {
        let x = 7;              // value of x in here is 7
        let x = x * 2;          // value if x is 14 now
        const PI: f64 = 4.0;    // value of PI in here is 4.0
    }

    // both values return to their previous states
}
```

Types can be shadowed too.

```rust
let x = 5;      // x is i32
let x = 7.6;    // x is f64

const PI: f64 = 3.14;   // PI is f64
const PI: u32 = 4;      // PI is u32
```

Types of mutable variables cannot be mutated!

```rust
// This is fine
let spaces = "    ";        // spaces is a string
let spaces = spaces.len()   // spaces is a pointed-sized, unsigned integer (u32)
                            // containing the original string's length

// This is not
let mut spaces = "    ";
let spaces = spaces.len();
```

Be careful while shadowing as there may be confusion between shadowing and assigning sometimes. See [[reassigning-confusion]], **it is very important**.
