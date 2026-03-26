# Recoverable errors with result

Most errors aren't serious enough to cause the program to crash. **`Result`** is an enum that helps us handle them. **`Result`** enum is defined with 2 variants **`Ok`** and **`Err`**

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

**`T`** and **`E`** are generic types. **`Ok`** contains the result value for a successful run and **`Err`** returns an error value that will be returned in a failure case.

Here's an example of file handling with error handling using **`Result`**

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}
```

The type of value inside the **`Err`** variant of the Result enum is a struct **`io::Error`** which has a method **`.kind()`** that returns the type of the error in the form of a variant of the **`io::ErrorKind`** enum. We can use a match statement to handle this error.

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
}
```

---

## Propogating errors
