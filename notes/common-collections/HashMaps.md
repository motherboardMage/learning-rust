# Storing keys and associated values in HashMaps

A **`HashMap<K, V>`** maps a **`key`** (K) to a **`value`** (V). The **`HashSet<K, ()>`** just keeps track of the key and stores an empty tuple for the value.

These are based on the Vector and so, also store their values on the heap.

---

## Accessing values in a HashMap

We ccan access values in a HashMap using the **`get()`** method.

```rust
let mut ratings = HashMap::new();

ratings.insert("Milton", 4.5);
ratings.insert("Pexpo", 4.2);

let milton_rating = ratings.get("Milton").copied().unwrap_or(0f64);

println!("{ratings:#?}");
println!("Rating for Milton: {milton_rating}");
```

**`get()`** returns an **`Option<&V>`** which needs to be handled extensively.

---

## HashMaps and Ownership

For keys that implement the **`Copy`** trait, they are simply copied into the HashMap but for types that don't (like Strings), they are moved into the HashMap during insertion.

```rust
let mut ratings = HashMap::new();

let milton = String::from("Milton");
let pexpo = String::from("Pexpo");

ratings.insert(milton, 4.5);
ratings.insert(pexpo, 4.2);

let milton_rating = ratings.get("Milton").copied().unwrap_or(0f64);

println!("{ratings:#?}");
println!("Rating for Milton: {milton_rating}");

println!("{}", milton); // error, value moved
```

---
