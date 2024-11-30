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
use ::quote::quote;
use ::syn::{parse, Data, DeriveInput, Field, Fields, Ident, Variant};

#[proc_macro_derive(GetOrInsert)]
pub fn derive_get_or_insert(input: TokenStream) -> TokenStream {
    implementation(input).unwrap().into()
}

fn implementation(input: TokenStream) -> Result<TokenStream2, String> {
    let ast: DeriveInput = parse(input).unwrap();

    let data = match ast.data {
        Data::Enum(data) => Ok(data),
        _ => Err("GetOrInsert can only be derived for enums"),
    }?;

    let enum_ident: Ident = ast.ident.clone();

    let variants_data = data.variants.iter().collect::<Vec<_>>();

    let variants_to_tokens = variants_data
        .iter()
        .filter_map(|v| {
            let ident = v.ident.clone();
            VariantData::try_from_syn_variant(v, &enum_ident)
                .map_err(|_| {
                    format!(
                        "Error in {enum_ident}::{ident}: \
                        GetOrInsert only supports a tuple-like, single item enum variants."
                    )
                })
                .transpose()
        })
        .collect::<Result<Vec<_>, _>>()?;

    let variant_tokens = variants_to_tokens.iter().map(VariantData::to_tokens);

    Ok(quote! {
        #(#variant_tokens)*
    })
}

struct VariantData {
    ident: Ident,
    enum_ident: Ident,
    first_unnamed_field: Field,
}

impl VariantData {
    fn try_from_syn_variant(variant: &Variant, enum_ident: &Ident) -> Result<Option<Self>, ()> {
        let ident = variant.ident.clone();
        let enum_ident = enum_ident.clone();
        let first_unnamed_field = match &variant.fields {
            Fields::Unit => return Ok(None),
            Fields::Named(_) => return Err(()),
            Fields::Unnamed(ref fields_unnamed) => {
                let mut items_iter = fields_unnamed.unnamed.iter();
                let Some(first_field) = items_iter.next() else {
                    return Err(());
                };
                if items_iter.next().is_some() {
                    return Err(());
                }
                first_field.clone()
            }
        };
        Ok(Some(VariantData {
            ident,
            enum_ident,
            first_unnamed_field,
        }))
    }

    fn to_tokens(&self) -> TokenStream2 {
        let ident = &self.ident;
        let enum_ident = &self.enum_ident;
        let first_field = &self.first_unnamed_field;
        quote! {
            impl ::get_or_insert::GetOrInsert<#first_field> for #enum_ident {
                fn insert(&mut self, value: T) -> &mut T {
                    *self = #enum_ident::#ident(value);
                    match self {
                        #enum_ident::#ident(inner) => inner,
                        _ => unreachable!(),
                    }
                }
                fn get_or_insert_with<F: FnOnce() -> T>(&mut self, f: F) -> &mut T {
                    match self {
                        #enum_ident::#ident(inner) => inner,
                        _ => self.insert(f()),
                    }
                }
            }
        }
    }
}
