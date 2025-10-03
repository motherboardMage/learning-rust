
## Prelude:

The Rust *prelude* is a collection of *standard library components* that are included automatically in every Rust program or module. They include things like **`Option, Clone, Vec, Result`** etc. so that we do not need to include these components explicitly, reducing verbosity.

---
## Libraries and "use":

If we want to include libraries and components that are not included in the prelude, we use the **`use`** keyword to include them explicitly in the program.

### Examples:

```rust
use rand::Rng;
use std::{
	cmp::Ordering,
	io::{self, Write},
};

std::io::{self, Write}; // Library::module::{module_iteslf, trait};
```

---
## Printing to console:

There are many ways of printing to console. Two of them used in this project are:

```rust
println!("Guess the number!");
```

and

```rust
print!("Enter your guess: ");
io::stdout().flush().expect("Could not flush stdout!");
```

Notice that for **`println!()`** we did not need any extra code but for **`print!()`** we needed the extra line. This is because **`println!()`** *macro* prints text on *stdout* and also flushes the output while printing a newline. Since I want the input to be taken on the same line as the `Enter your guess` message, I used the **`print!()`** macro which does not print a newline.

However, this causes the input line, which comes after it to execute first as the output produced by **`print!()`** is buffered and is not displayed unless the output stream is flushed which is achieved by **`io::stdout().flush()`**.

---
## Declaring and initialising a string:

In the code,

```rust
let mut guess = String.new();
```

is used instead of

```rust
let mut guess: String;
```

as **`String.new()`** is a function that creates a new, empty instance of a **`String`** which is a type provided by the *standard library* which is a growable, [[UTF-8]] encoded string whereas **`guess: String`** would only declare an uninitialised string which could lead to any random memory location and thus, is memory unsafe. You could also just declare an uninitialised string and initialise it on the next line or somewhere else before using but it is not worth the risk.

---
## User input:

We imported the **`io`** module from the *standard library* **`std`**. Now to take user input, we use **`read_line()`** method on the *standard input handle* **`stdin()`** provided by the *Rust Standard Library*.

The code to this looks something like this:

```rust
io::stdin().read_line(&mut guess);
```

Looking closer, the **`read_line(&mut buff)`** method reads a line from the *standard input stream* and puts it into a mutable variable **`buff`**. Now, Rust needs robust error handling so just writing this much code would lead to a warning:

```zsh
warning: unused `Result` that must be used
  --> projects/guessing_game/src/main.rs:16:5
   |
16 |     io::stdin().read_line(&mut guess);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: this `Result` may be an `Err` variant, which should be handled
   = note: `#[warn(unused_must_use)]` on by default
help: use `let _ = ...` to ignore the resulting value
   |
16 |     let _ = io::stdin().read_line(&mut guess);
   |     +++++++
```

which brings us to the **`Result`** type:

### Result:

It is an **`Enum`**, short for *Enumeration* which is a type that can be in one of multiple possible states, each called a **`Variant`**. Its variants are **`Ok`** and **`Err`**. It being **`Ok`** means that the **`read_line()`** function worked without issues but **`Err`** means that there was or were some issues due to which **`read_line()`** specifically couldn't read input. Now for safety and completeness of a program, there should be *error handling code* that decides what happens in the situation where there is a problem but, if we just want the process to crash and print an error message, we can use the **`expect(self, msg: &str)`** method.

Now the code looks like this:

```rust
io::stdin().read_line(&mut guess).expect("Could not read input");
```

Though, all this on the same line looks cluttered so we move the methods onto separate lines, taking advantage of the fact that **`;`** defines a new line.

```rust
io::stdin()
	.read_line(&mut guess)
	.expect("Could not read input");
```

---
## Crates and Cargo:

**`Crates`** are a collection of Rust source code files. They are of two types:

1. Binary crates: These are supposed to produce an executable
2. Library crates: These are supposed to be used in other files / projects such as this one

**`Cargo`** is Rust's build system and package manager. For crates, the ones mentioned in **`cargo.toml`** under `[dependencies]` and the latest versions of everything a specific dependency needs are fetched by **`Cargo`** from the *registry* which is a copy of the data at Crates.io.

### Cargo.lock file:

It contains all the versions of the dependencies that were used to build a project. This is done to ensure that a successful build is reproducible i.e. it can run everywhere without worrying if a dependency updates and breaks our code as cargo will not upgrade it and use the same versions mentioned in the **`Cargo.lock`** file unless we specifically upgrade the dependencies.

---
## Random number generation:

We use the 