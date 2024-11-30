
A trait for enum types that support `.insert(x)` method which sets the enum variant to
something unique for `x` and returns a mutable reference to the value.

The most trivial example is `Option<T>`.

This operation is somehow not trivial to write in Rust (See the following sample generated code).
`#[derive(GetOrInsert)]` macro provides an implementation for enums
with every variant having a single distinct field types (or an unit type like `Option::None`).

# Example

If you write an enum like this:

```rust
use ::derive_insert::GetOrInsert;
#[derive(GetOrInsert)]
pub enum Foo {
   Bar(i32),
   Baz(String),
   AnEmptyVariant,
}
```

The following code will be generated:

```rust
# use ::derive_insert::GetOrInsert;
# pub enum Foo {
#    Bar(i32),
#    Baz(String),
#    AnEmptyVariant,
# }
impl GetOrInsert<i32> for Foo {
    fn insert(&mut self, value: i32) -> &mut i32 {
        *self = Self::Bar(value);
        match self {
            Self::Bar(ref mut x) => x,
            _ => unreachable!(),
        }
    }
    fn get_or_insert_with<F: FnOnce() -> i32>(&mut self, f: F) -> &mut i32 {
        match self {
            Self::Bar(ref mut x) => x,
            _ => self.insert(f()),
        }
    }
}
impl GetOrInsert<String> for Foo {
    // ... Same for Foo::Baz
    # fn insert(&mut self, value: String) -> &mut String { todo!() }
    # fn get_or_insert_with<F: FnOnce() -> String>(&mut self, f: F) -> &mut String { todo!() }
}
// Foo::AnEmptyVariant is skipped because it's an unit variant
```

# Limitations

Currently, this derive macro only supports the enum variants which are:
 - tuple-like, single field (`e.g. Option::Some(T)`),
 - or unit variants (`e.g. Option::None`).