// Copyright 2023 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn recorded(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = proc_macro2::TokenStream::from(item);
    quote! {
        #[::core::prelude::v1::test]
        #item
    }
    .into()
}
