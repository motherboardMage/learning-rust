# Vectors

This data collection stores items in contiguous locations in memory. It is growable that is it can grow in size. 

A veector can hold only one type of values. However, this means if we want to store multiple types, we can use an enum type with variants of the different types and a match expression to handle all possible cases wherever operations to the vector items have to be performed.

Operations on vector items can be performed using various methods provided by the Vec struct. Some common operations include:

- Pushing elements onto the end of the vector using the `push` method.
- Removing elements from the end of the vector using the `pop` method.
- Accessing elements at a specific index using the `get` method.
- Iterating over the elements of the vector using the `iter` method.

---
