// Copyright 2023 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

// cspell:ignore asyncness
use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn recorded(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item: proc_macro2::TokenStream = item.into();
    let Ok(syn::Item::Fn(func)) = syn::parse2(item.clone()) else {
        return syn::Error::new(item.span(), "only valid on functions")
            .into_compile_error()
            .into();
    };

    if func.sig.asyncness.is_some() {
        return quote! {
            #[::tokio::test]
            #func
        }
        .into();
    }

    quote! {
        #[::core::prelude::v1::test]
        #func
    }
    .into()
}
