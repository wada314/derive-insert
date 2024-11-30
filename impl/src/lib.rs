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

use ::proc_macro::TokenStream;
use ::proc_macro2::TokenStream as TokenStream2;
use ::syn::{parse, Data, DeriveInput, Fields, Ident};

#[proc_macro_derive(GetOrInsert)]
pub fn derive_get_or_insert(input: TokenStream) -> TokenStream {
    implementation(input).unwrap()
}

fn implementation(input: TokenStream) -> Result<TokenStream, &'static str> {
    let ast: DeriveInput = parse(input).unwrap();

    let data = match ast.data {
        Data::Enum(data) => Ok(data),
        _ => Err("GetOrInsert can only be derived for enums"),
    }?;

    let enum_name = &ast.ident;

    let variants_data = data.variants.iter().collect::<Vec<_>>();

    let variant_types = variants_data
        .iter()
        .filter_map(|v| {
            let fields_unnamed = match v.fields {
                Fields::Unit => return None,
                Fields::Named(fields_named) => {
                    return Some(Err("GetOrInsert only supports a tuple-like enum variants."))
                }
                Fields::Unnamed(fields_unnamed) => Some(Ok(fields_unnamed)),
            };
            let ident = v.ident.clone();
            Some(Ok(VariantData {
                ident,
                fields_unnamed,
            }))
        })
        .collect::<Result<Vec<_>>>()?;
    todo!()
}

struct VariantData {
    ident: Ident,
    fields_unnamed: TokenStream2,
}
