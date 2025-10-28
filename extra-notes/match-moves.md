# Match arms move value

In an example snippet for **`if let`** and **`else`**, [snippet](../notes/enums-pattern-matching/snippets/if-let-else.rs), there was code written in this way:

```rust
fn main() {
    // Regular match expression
    let picked_up = Drop::Armour(Material::Gold);
    match picked_up {
        Drop::Armour(material) => println!("Picked up armour of {:?}", material),
        _ => (),
    }

    // If let else
    if let Drop::Armour(material) = picked_up {
        println!("Picked up armour of {:?}", material);
    } else {
        println!("The picked up item was not armour");
    }
}
```

However, this would not compile because match arms move values into the variables they bind to. So in the first match expression, the first arm would cause **`Material::Gold`** from **`picked_up`** to move into the **`material`** variable. This would leave us unable to use **`picked_up`** further in code.

To fix this, we add the **`ref`** keyword before **`material`**, telling the compiler that it is a reference.

```rust
match picked_up {
    Drop::Armour(ref material) => println!("Picked up armour of {:?}", material),
    _ => (),
}
```

This fixes our code and let's us use **`picked_up`** further in the code.

---
