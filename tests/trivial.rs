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

use ::derive_insert::GetOrInsert;
use ::std::fmt::Debug;

#[derive(GetOrInsert)]
enum Foo {
    Bar1(i32),
    Bar2(String),
    #[allow(unused)]
    UnitVariantShouldBeSkipped,
}

fn check_get_or_insert<T: GetOrInsert<U>, U: Clone + PartialEq + Debug>(
    mut e: T,
    value1: U,
    value2: U,
) {
    assert_eq!(e.insert(value1.clone()), &value1);
    assert_eq!(e.get_or_insert(value2.clone()), &value1);
}

#[test]
fn test_bar1() {
    check_get_or_insert(Foo::Bar1(43), 44, 45);
}

#[test]
fn test_bar2() {
    check_get_or_insert(
        Foo::Bar2("hello".to_string()),
        "world".to_string(),
        "earth".to_string(),
    );
}
