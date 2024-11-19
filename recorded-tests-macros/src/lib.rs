// Copyright 2023 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use proc_macro::TokenStream;
use quote::quote;
use recorded_tests_core::TestMode;
use std::{env, sync::LazyLock};
use syn::{parse::Parser, spanned::Spanned, FnArg, PatType};

type AttributeArgs = syn::punctuated::Punctuated<syn::Meta, syn::Token![,]>;

// cspell:ignore asyncness
#[proc_macro_attribute]
pub fn recorded(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr: proc_macro2::TokenStream = attr.into();
    let Ok(args) = AttributeArgs::parse_terminated.parse2(attr.clone()) else {
        return syn::Error::new(attr.span(), "expected comma-separated arguments")
            .into_compile_error()
            .into();
    };
    let live_only = args
        .first()
        .and_then(|meta| meta.path().get_ident())
        .is_some_and(|ident| *ident == "live");

    let item: proc_macro2::TokenStream = item.into();
    let Ok(syn::ItemFn {
        attrs,
        vis,
        mut sig,
        block,
    }) = syn::parse2(item.clone())
    else {
        return syn::Error::new(item.span(), "only valid on functions")
            .into_compile_error()
            .into();
    };

    let mut out: proc_macro2::TokenStream = if sig.asyncness.is_some() {
        quote! { #[::tokio::test] }
    } else {
        quote! { #[::core::prelude::v1::test] }
    };

    let test_mode = TestModeTokens(*TEST_MODE);
    if live_only && test_mode.0 < TestMode::Live {
        out.extend(quote! {
            #[ignore = "skipping live tests"]
        });
    }

    let mut preamble = proc_macro2::TokenStream::new();
    if let Some(FnArg::Typed(PatType { pat, ty, .. })) = sig.inputs.first() {
        if !is_test_context(ty.as_ref()) {
            return syn::Error::new(item.span(), "expected `TestContext` parameter")
                .into_compile_error()
                .into();
        }

        let fn_name = &sig.ident;
        preamble = quote! {
            #[allow(unused_variables)]
            let #pat = #ty::new(#test_mode, ::std::module_path!(), ::std::file!(), stringify!(#fn_name));
        }
    }

    sig.inputs.clear();
    out.extend(quote! {
        #(#attrs)*
        #vis #sig {
            #preamble
            #block
        }
    });

    out.into()
}

static TEST_MODE: LazyLock<TestMode> = LazyLock::new(|| {
    // Okay to panic if AZURE_TEST_MODE is unsupported.
    env::var("AZURE_TEST_MODE").map_or_else(|_| TestMode::default(), |s| s.parse().unwrap())
});

#[derive(Clone, Debug)]
struct TestModeTokens(TestMode);

impl quote::ToTokens for TestModeTokens {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self.0 {
            TestMode::Playback => {
                tokens.extend(quote! { ::recorded_tests_core::TestMode::Playback })
            }
            TestMode::Record => tokens.extend(quote! { ::recorded_tests_core::TestMode::Record }),
            TestMode::Live => tokens.extend(quote! { ::recorded_tests_core::TestMode::Live }),
        }
    }
}

fn is_test_context(arg: &syn::Type) -> bool {
    let path = match arg {
        syn::Type::Path(syn::TypePath { path, .. }) => path,
        _ => return false,
    };

    if path.leading_colon.is_none()
        && path.segments.len() == 1
        && path.segments[0].ident == stringify!(TestContext)
    {
        return true;
    }

    path.segments.len() == 2
        && path.segments[0].ident == "recorded_tests_core"
        && path.segments[1].ident == "TestContext"
}
