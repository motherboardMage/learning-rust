# Method Syntax

Methods are similar to functions but are defined in the context of a **`struct`** or an **`enum`** or a **`trait`** object and their first parameter is always **`self`** which represents the instance of the aforementioned the method is being called on.

## Defining Methods

We can rewrite the [area snippet](snippets/area.rs) using a method implemented on the **`Rectangle`** struct.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 15,
        height: 5,
    };

    println!("The area of the rectangle is: {}", rect.area());
}
```

Here, an implementation starts with the **`impl`** keyword followed by the type of object it is being implemented on. Then, to write our *method* we define it like we normally define a function just keeping the first parameter as a reference to **`self`**. **`&self`** is actually shorthand for self: &Self. *Methods* can take ownership of **`self`**, borrow **`self`** mutably or immutably.

Method syntax lets us put all the capabilities of an instance of **`self`** in one **`impl`** block rather than spreading different functions that work on **`self`** all around our code. This makes it easier to discover, use and add functionality to the type rather than spending time looking for them.

---

## Getters

Often but not always, when we name a method the same as a struct's field, we intend it to simply get the value of that field. Methods like these are called *getters*. Rust does not implement them automatically. They are useful as we can keep the field private but the method public which allows read only access to the field.

Here's a getter for the **`Rectangle`** struct's width,

```rust
fn width(&self) -> u32 {
    self.width
}
```

---

## Methods With More Parameters

Methods can have multiple parameters. Since **`&self`** is always a parameter, it does not need to be added while calling the method.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 15,
        height: 5,
    };

    let rect1 = Rectangle {
        width: 16,
        height: 7,
    };

    println!("The area of the rectangle is: {}", rect.area());
    println!("Width of the rectangle is {}", rect.width());

    if rect.can_hold(&rect1) {
        println!("rect can hold rect1");
    } else {
        println!("rect cannot hold rect1");
    }
}
```

---

## Associated Functions

All functions defined after **`impl`** are called *associated functions*. Here the ones that have **`self`** as their first parameter are called methods but it isn't necessary for them to have it.

We've used such a function associated with **`String`**, the **`from()`** function. Such associated functions are often called *constructors* and are called with the double colon **`::`** syntax like **`String::from()`**.

Here's an associated function **`square()`** on **`Rectangle`**.

```rust
fn square(size: u32) -> Self {
    Rectangle {
        width: size,
        height: size,
    }
}
```

Every struct can also have multiple **`impl`** blocks.

---
