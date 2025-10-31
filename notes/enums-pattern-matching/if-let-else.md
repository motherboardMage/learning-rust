# Concise Control Flow with if let and let else

**`if let`** allows us to handle cases where we want to match to only one variant anyway in a more concise manner.

Here's a match expression:

```rust
let picked_up = Drop::Armour(Material::Gold);
match picked_up {
    Drop::Armour(material) => println!("Picked up armour of {:?}", material),
    _ => (),
}
```

Written with **`if let`**,

```rust
if let Drop::Armour(material) {
    println!("Picked up armour of {:?}", material);
}
```

This looks much cleaner!

**`else`** can also be used with **`if let`**. It will behave the same way that the **`_`** arm would in a match expression.

```rust
if let Drop::Armour(material) {
    println!("Picked up armour of {:?}", material);
} else {
    println!("The picked up item was not armour");
}
```

---

## let else

**`let else`** can be used to check a failure case first.

```rust
fn calculate_reward(drop: Drop) -> u32 {
    let Drop::Card(rarity) = drop else {
//  ----------------------        ----
//            |                     |
//            |                     |-> execute the following
//            |                         code otherwise
//            |
//            |-> match to drop and bind value to rarity
        println!("Not a card!");
        return 0;
    };

    // control reaches here, meaning
    // the drop was indeed a card
    match rarity {
        Rarity::Common => 10,
        Rarity::Uncommon => 100,
        Rarity::Rare => 1000,
        Rarity::Epic => 10000,
        Rarity::Legendary => 100000,
    }
}
