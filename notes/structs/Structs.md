# Structs

A struct, or structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group

## Defining and Instantiating Structs

We define structs by using the **`struct`** keyword followed by its name and then inside curly braces, we list the pieces of data by their name and types. These pieces are called the fields of the **`struct`**.

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    age: u8,
}
```

To instantiate a struct, we state its name and after the equals sign, write its type followed by curly braces which contain the values in **`key:value`** pairs.

```rust
let mut user1 = User {
    active: true,
    username: String::from("offensive_name"),
    email: String::from("something@somewhere.com"),
    age: 34
};
```

To access a value stored in a struct variable, we use the dot notation.

```rust
let mut user1 = User {
    active: true,
    username: String::from("offensive_name"),
    email: String::from("something@somewhere.com"),
    age: 34
};

user1.username = String::from("more_offensive_name");
```

We can also instantiate a struct by returning it from another function.

```rust
fn main() {
    let mut user1 = create_new_user();
}

fn create_new_user(username: String, email: String, age: u8) -> User {
    User {
        active: true,
        username: username,
        email: email,
        age: age
    }
}
```

If the field and function argument names match, we can also use the shorthand for initializing a struct,

```rust
fn create_new_user(username: String, email: String, age: u8) -> User {
    User {
        active: true,
        username,
        email,
        age
    }
}
```

It is also possible to initialize a struct variable from another one by using **`..`** if the variables are of the same struct type.

```rust
let user2 = User {
    username: String::from("someone_else"),
    ..user1
};
```

This creates user2 of the type **`User`** and copies all data from user1 other than its username which we have explicitly mentioned.

Remember, a compound type will be moved if any of its members implement the **`Copy`** trait. So, if we use the above syntax, user1 will be moved into user2 and will be inaccessible after the struct update but, if we gave user2 its own new *username* and *email* values and copied the rest of the values from user1, it would not be moved into user2.
