# field-visibility

Attribute to specify the visibility of all fields in a Rust `struct`.

## Usage

### Examples

The argument is the visibility mode (i.e. `pub` or `pub(crate)`).

To set all fields in the `struct` to public:

```rust
use field_visibility::visibility;

#[visibility(pub)] // Or `pub(crate)`, etc.
struct Widget {
    a: i32,
    b: i32,
}
```

This expands to:

```rust
use field_visibility::visibility;

#[visibility(pub)]
struct Widget {
    pub a: i32,
    pub b: i32,
}
```

---

When using this attribute, the visibility of individual fields can be set.

```rust
use field_visibility::visibility;

#[visibility(pub)]
struct SomeStruct {
    field1: i32,
    field2: i32,

    pub(crate) crate_field: i32,

    pub(self) private_field: i32, // Equivalent to private
}
```

The expansion of this is:

```rust
struct SomeStruct {
    pub field1: i32,
    pub field1: i32,

    pub(crate) crate_field: i32,

    pub(self) private_field: i32,
}
```
