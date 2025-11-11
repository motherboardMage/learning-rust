# Packages and Crates

A **`crate`** is the smallest piece of code that the Rust compiler considers at a time. Be it a single source file or a module that gets defined in other files.

Crates are of two types: **`Binary crates`** and **`Library crates`**. A binary crate has a **`main`** function and it compiles into an executable while a library crate has other functionality that other crates or programs may use. Library crates do not have a **`main`** function. Usually, rustaceans use **`crate`** interchangably with library crates or the general programming concept of a **`Library`**.

A **`package`** is a bundle of one or more crates that provides a set of functionality. Cargo is also a package that contains a binary crate for the command line tool and a library crate that the binary crate depends on.

A package must contain atleast one crate and there can be many binary crates in a package but only one library crate. A package contains a **`Cargo.toml`** file that discribes how to build those crates.

When we run **`cargo new <project-name>`**, it creates a new package.

---
