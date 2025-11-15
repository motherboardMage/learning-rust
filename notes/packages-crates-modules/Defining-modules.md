# Defining Modules to Control Scope and Privacy

## Cheat sheet

The crate root is the file **`main.rs`** or **`lib.rs`** for library crates. We declare our *modules* here.

An example module vegetables which will be looked for in either the curly braces following **`mod vegetables`** or in **`src/vegetables.rs`** or **`src/vegetables/mod.rs`**.

```rust
// main.rs
mod thaali;

fn main() {
    // code
}
```

If we want to use the same code in all files within a crate, we can use the **`pub`** keyword before **`mod`**.

```rust
// main.rs
pub mod thaali;

fn main() {
    // code
}

// something_else.rs
use thaali::beverages::HotBeverage; // HotBeverage enum in  
                                    // beverages submodule

fn drink(pl: &mut Plate) {
    pl.garam().sip();   // Example method
}
```

The use statements create a "shortcut" to items within the modules so that we won't have to type in the complete path to them.

Any module defined in a file other than the root file of the crate is called a *submodule*. Say, for example, a submodule **`beverages`** defined in **`thaali/mod.rs`**.

---
