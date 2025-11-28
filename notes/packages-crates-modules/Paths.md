# Paths for referring to items in module trees

There are two types of paths:

1. **Absolute Path**: Starting from the **`crate root`**, they begin with the literal **`crate`**.

2. **Relative Path**: Start from the current module and begin with **`self`**, **`super`** or an identifier within the current module.

```rust
use thaali::{Plate, beverages::*, eatables::*};
```

Above is a path from the crate root bringing into scope a few items from the module **`thaali`**. It is a relative path as the module **`thaali`** was defined before.

Everything imported here should be public so that it is accessible to any callers. For *structs* and *enums*, their individual fields that need to be accessed need to be public or there should be an associated function that returns their private values.

Items in *parent modules* cannot access items in *child modules* by default but items in child modules can access items in their *ancestor modules*.

---

## Using `super` to start relative paths

We can use **`super`** at the start of a path to start a relative path from the parent module of the current module.

```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

In the above code, **`super`** allows the calling of **`deliver_order()`** which is outside the current module (in its parent module).

---

## The use keyword

It allows us to bring items from other modules into scope. It is kind of like creating a symbolic link in a filesystem.

Below is a code that **won't compile** as the function brought into scope is outside the *child module* **`customer`** so it is in a completely different scope.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
```

If a function is being imported using the **`use`** keyword, it is **idiomatic** to only include its parent module which makes it clear that it is not locally defined. However, for **`structs`**, **`enums`** and other items, it is idiomatic to include the full path to the item.

The only exception to this is when bringing two items of the same name into scope.

```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```

---

## Providing new names with the `as` keyword

We can use **`as`** after a **`use`** statement to give the imported items an alias. Below is another way to import the two **`Result`** types.

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

This and the other way are both considered idiomatic.

---
## Re-exporting names with `pub use`

When we being a name into scope using **`use`**, the name is private to the scope we imported it in. We use pub use to enable external code refer to the name as if it had been declared in that scope. For example, if we added **`pub`** to the restaurant code

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

External code can now call the function **`add_to_waitlist()`** using **`restaurant::hosting::add_to_waitlist()`** instead of the full path **`restaurant::front_of_house::hosting::add_to_waitlist()`**.

---
