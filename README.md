This crate provides the named_array derive macro, which allows you to access fields of a
struct as if they were elements of an array.
This provides an impl's of `Index` and `IndexMut`, which translates from a `usize` index to the fields, in the
order in which they appear.

The type of all the fields must be the same, and written identically.
For example, if one field is `Option<()>`, and another is `core::option::Option<()>`, the code
will be rejected.
This is because type information does not exist at the time of macro expansion, so there is no
way to confirm that the two refer to the same type.

Indexing will panic if the index is out of bounds.

# Example
```rust
#[derive(named_array)]
struct Example {
    a: u32,
    b: u32,
    c: u32,
}

let example = Example { a: 1, b: 2, c: 3 };
assert_eq!(example[0], example.a);
assert_eq!(example[1], example.b);
assert_eq!(example[2], example.c);
```

# Tuple structs

This can be used with tuple structs as well.
However, you may be better off using `struct Foo([u32; 3])` instead of `struct Foo(u32, u32, u32)`.

```rust
#[derive(named_array)]
struct Example(u32, u32, u32);
let example = Example(1, 2, 3);
assert_eq!(example[0], example.0);
assert_eq!(example[1], example.1);
assert_eq!(example[2], example.2);
```
