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

#[cfg_attr(not(doctest), doc = include_str!("../README.md"))]
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
