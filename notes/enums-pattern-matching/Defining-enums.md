# Defining Enums

## What are Enums?

Enums are custom data types that can have one of a possible set of values or *Variants* in Rust's case. For example,

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

Here, we are saying that the **`IpAddrKind`** type can have either one of two *variants*: **`V4`** or **`V6`**. *Variants* are the possible types that an **`Enum`** can be. In Rust, Enum variants can also hold values.

---

## Enum Values

This is how we instantiate **`Enums`**:

```rust
let ipv4 = IpAddrKind::V4
let ipv6 = IpAddrKind::V6
```

Note that the variants of and Enum are namespaced under their identifier. This is useful because both values are of the same type and we can create a function that takes any **`IpAddrKind`**.

```rust
fn route(ip_kind: IpAddrKind) {}
```

If we wanted to do this with a struct:

```rust
struct IpAddr {
    kind: IpAddrKind,
    addr: String,
}

home = IpAddr {
    kind: IpAddrKind::V4,
    addr: String::from("127.0.0.1"),
}
```

But the Enum syntax is more concise. The definition of the Enum will change to:

```rust
enum IpAddr {
    V4(String),
    V6(String),
}
```

Now, the enum can be instantiated this way,

```rust
let home = IpAddr::V4(String::from("127.0.0.1"));
```

Note that the name of each Enum variant is also the function that constructs it. The function **`IpAddr::V4()`** is a function that returns an instance of the Enum.

Another advantage that Enums provide is that each variant can have different types and numbers of values associated with them.

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
```

Like this, an address would be instantiated this way,

```rust
let ipv4 = IpAddr::V4(127, 0, 0, 1);
let ipv6 = IpAddr::V6("::1");
```

We can put any type of data inside enum variants. The standard Rust library has predefined types for IP V4 and IP V6 addresses.

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

See how the variants store Structs here. Another example for an enum:

```rust
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

We can also define *methods* on Enums as we did on Structs using **`impl`**.

```rust
impl Message {
    fn call(&self) {
        // something
    }
}

fn main() {
    let m = Message::Write(String::from("Something here"));
    m.call();
}
```

Here, **`self`** will be the value or instance the method is called on.

---

## The `Option` Enum

This is an enum defined by the standard library. It encodes the concept of a value being present or absent. Expressing this concept in terms of the type system means that the compiler can check if all the cases are being handled and prevent a multitude of bugs.

```rust
enum Option<T> {
    None,
    Some(T),
}
```

That **`<T>`** means that the **`Some`** variant is a *generic type* that is, it can hold one value of any types so,

```rust
let some_char = Some('c');
let some_num = Some(5);

let absent_number: Option<i32> = None;
```

The type of **`some_char`** is **`Option<char>`** and of **`some_num`** is **`Option<i32>`**. For **`absent_number`**, we need to annotate its type because it is not possible to infer it in compile time.

**`None`** is similar to **`NULL`** as in it represents the absence of data but since it is a pre-defined type in Rust, the compiler always checks for any value with **`Option<T>`** type that all cases have been handled as the value can also be **`None`**.

```rust
fn main() {
    let x: i8 = 4;
    let y: Option<i8> = Some(5);

    let sum = x + y;
}
```

Compiling this will give an error,

```rust
error[E0277]: cannot add `Option<i8>` to `i8`
 --> add.rs:5:17
  |
5 |     let sum = x + y;
  |                 ^ no implementation for `i8 + Option<i8>`
  |
  = help: the trait `Add<Option<i8>>` is not implemented for `i8`
  = help: the following other types implement trait `Add<Rhs>`:
            `&i8` implements `Add<i8>`
            `&i8` implements `Add`
            `i8` implements `Add<&i8>`
            `i8` implements `Add`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0277`.
```

As Rust does not know how to add an **`i8`** and an **`Option<i8>`** since it is known that a variable with the type **`i8`** will have a known value at compile time but **`Option<i8>`** may be **`None`**. This shows how the compiler ensures safety at compile time by verifying that logic is being performed on supported types only.

---
