// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub use ::derive_insert_impl::GetOrInsert;

/// A trait for types that support `.insert(x)` method which returns the reference to the inserted value.
/// The most trivial example is `Option<T>`.
/// This operation is somehow not trivial to write in Rust. `#[derive(GetOrInsert)]` macro provides
/// an implementation for enums with every variant having a single distinct field types (or an unit type like `Option::None`).
///
/// # Example
/// If you write an enum like this:
///
/// ```rust
/// use ::derive_insert::GetOrInsert;
/// #[derive(GetOrInsert)]
/// pub enum Foo {
///    Bar(i32),
///    Baz(String),
///    AnEmptyVariant,
/// }
/// ```
///
/// The following code will be generated:
///
/// ```rust
/// # use ::derive_insert::GetOrInsert;
/// # pub enum Foo {
/// #    Bar(i32),
/// #    Baz(String),
/// #    AnEmptyVariant,
/// # }
/// impl GetOrInsert<i32> for Foo {
///     fn insert(&mut self, value: i32) -> &mut i32 {
///         *self = Self::Bar(value);
///         match self {
///             Self::Bar(ref mut x) => x,
///             _ => unreachable!(),
///         }
///     }
///     fn get_or_insert_with<F: FnOnce() -> i32>(&mut self, f: F) -> &mut i32 {
///         match self {
///             Self::Bar(ref mut x) => x,
///             _ => self.insert(f()),
///         }
///     }
/// }
///
/// impl GetOrInsert<String> for Foo {
///     // ... Same for Foo::Baz
///     # fn insert(&mut self, value: String) -> &mut String { todo!() }
///     # fn get_or_insert_with<F: FnOnce() -> String>(&mut self, f: F) -> &mut String { todo!() }
/// }
///
/// // Foo::AnEmptyVariant is skipped because it's an unit variant
/// ```
pub trait GetOrInsert<T> {
    // Required methods
    fn insert(&mut self, value: T) -> &mut T;
    fn get_or_insert_with<F: FnOnce() -> T>(&mut self, f: F) -> &mut T;

    // Provided methods
    fn get_or_insert(&mut self, value: T) -> &mut T {
        self.get_or_insert_with(|| value)
    }
    fn get_or_insert_default(&mut self) -> &mut T
    where
        T: Default,
    {
        self.get_or_insert_with(Default::default)
    }
}

/// Provides a default implementation for `Option<T>`.
impl<T> GetOrInsert<T> for Option<T> {
    fn insert(&mut self, value: T) -> &mut T {
        <Option<T>>::insert(self, value)
    }

    fn get_or_insert_with<F: FnOnce() -> T>(&mut self, f: F) -> &mut T {
        <Option<T>>::get_or_insert_with(self, f)
    }
}
