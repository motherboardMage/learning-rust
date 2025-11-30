# Strings

Strings in Rust are a growable, mutable collection of UTF-8 encoded text. The **`String`** type is part of the Rust standard library. The only string type in the Rust core library is the **`string slice`** or **`str`** type.

Since Strings are implemented as a wrapper around a **`Vec<u8>`** with a few extra guarantees, we can use vector and vector like functions for strings like

```rust
fn main() {
    let mut str1 = String::from("Somebody");
    let str2 = String::from(" was here");
    let str3 = ", I guess...";

    str1.push_str(&str2);
    str1.push_str(str3);

    str1.push('\n');
    str1.push('W');
    str1.push('e');
    str1.push(' ');
    str1.push('c');
    str1.push('a');
    str1.push('n');
    str1.push(' ');
    str1.push('p');
    str1.push('u');
    str1.push('s');
    str1.push('h');
    str1.push(' ');
    str1.push('c');
    str1.push('h');
    str1.push('a');
    str1.push('r');
    str1.push('s');
    str1.push('!');

    println!("\nstr1 is {str1}");
    println!("\nstr2 is {str2}");
    println!("\nstr3 is {str3}");
}
```

---

## Concatenation with the `+` operator or the `format!` Macro

Strings can be concatenated easily using the `+` operator. It takes the original string and moves it into the left hand variable of the `=` operator with the string references (`&str`) on the right side.

```rust
let mut str1 = String::from("Is it");
let str2 = String::from("too late ");

str1 = str1 + " " + &str2;
println!("{}", str1);

let str3 = str1 + "to turn back now?";
println!("{}", str3);
// println!("{}", str1);
```

Uncommenting the last line will cause the compilation to fail as `str1` was moved into `str3`.

We can also use the **`format!`** macro to concatenate multiple strings. It can be use for much more than that though.

```rust
let s1 = String::from("Stardust");
let s2 = "In you and in me";
let s3 = "Fuse us...into unity";

let s = format!("{s1}...\n{s2}...\n{s3}");

println!("{s}");
```

---

## String indexing and slicing

String indexing is not allowed in Rust as Strings are encoded in **`UTF-8`** format and indexing based on bytes may lead us to a location within a multibyte character.

We can slice a string rather than index it by using a range within the **`[]`**

---
