# Unrecoverable errors with `panic!`

**`panic!`** makes the program print a failure message, unwind, clean up the stack and quit. Using an environment variable, it is also possible to print the call stack to help track the source of the panic.

If we want to keep our binary as small as possible, we can make panics just abort the program without all the unwinding and cleaning up (which will still have to be done by the OS) by adding this to the project's **`Cargo.toml`**

```rust
[profile.release]
panic = 'abort'
```

---
