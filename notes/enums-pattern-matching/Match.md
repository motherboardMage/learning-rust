# The match Control Flow Construct

A **`match`** statement compares a value against a set of patterns and executes code based on which pattern matches.

The code that is executed depends upon the **first** pattern that matches the value. The rest of the code is ignored.

```rust
enum Denom {
    Das,
    Sau,
    Do_sau,
    Paach_sau,
    Do_hazaar,
}

fn value_in_rupees(note: Denom) -> u16 {
    match note {
        Denom::Das => 10,
        Denom::Sau => 100,
        Denom::DoSau => 200,
        Denom::PaachSau => 500,
        Denom::DoHazaar => 2000,
    }
}
```

There really can be code inside a **`match`** arm.

```rust
match expression {
    arm1 => code,
    arm2 => {
        multi;
        line;
        code;
    },
}
```

---

## Patterns that Bind to Values

Enums variants can store values. Using **`match`**, we can *destructure* these variants to extract values from them while we are pattern matching. For example,

```rust
#[derive(Debug)]
enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

#[derive(Debug)]
enum Material {
    Leather,
    Iron,
    Gold,
    Diamond,
}

#[derive(Debug)]
enum Drop {
    Card(Rarity),
    Armour(Material),
}

fn reward_player(reward: Drop) {
    match reward {
        Drop::Card(rarity @ (Rarity::Uncommon | Rarity::Epic)) => {
            println!("An {:?} card!", rarity);
        }
        Drop::Card(rarity) => println!("An {:?} card!", rarity),
        Drop::Armour(material) => println!("An armour of {:?}", material),
    }
}
```

It outputs:

```zsh
An Epic card!
```

In the arms **`Drop::Card(rarity) => println!("An {:?} card!", rarity)`** and the one below it, **`match`** matches the pattern on the left to reward and binds the values those variants hold into variables **`rarity`** and **`material`**. In the first arm though, we use the **`@`** pattern to bind the value from multiple patterns of **`reward`** to **`rarity`**.

---

## The **`Option<T>`** enum

The **`Option<T>`** enum looks like:

```rust
pub enum Option<T> {
    None,
    Some(T),
}
```

**`<T>`** means here any type so it can be matched with value of any type as long as it exists.

```rust
fn add_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(value) => Some(value + 1),
    }
}

fn main() {
    let five = Some(5);

    let six = match add_one(five) {
        None => panic!(),
        Some(value) => value,
    };

    println!("{}", six)
}
```

Above code takes an **`Option<i32>`**, adds one to the value stored in it and converts it to an **`i32`** to then be printed.

Matches are exhaustive so in a match statement, one must cover all variants an Enum can be or the code would not compile.
