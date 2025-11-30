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

## Updating a HashMap

All keys have unique values in a HashMap. We can either update the value associated to a pre-existing key or insert a value only if the key isn't present in the map.

The **`insert()`** function does the first. We can chain **`.entry(K).or_insert(V)`** to insert a value if there is no key associated with it already. **`or_insert()`** method defined on **`entry()`** returns as mutable reference to the value if it exists.

We can use the return value of **`or_insert()`** to update a value based on the old value. For example, here is a program that counts the number of instances of a word in a sentence.

```rust
let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{map:?}");
```

---
