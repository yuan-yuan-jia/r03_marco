use proc_macro::TokenStream;
use darling::ast::Data;
use darling::{FromDeriveInput, FromField};
use syn::DeriveInput;
use quote::quote;
#[derive(Debug, FromDeriveInput)]
struct AutoDebugInfo {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<(), AutoDebugFieldsInfo>
}

#[derive(Debug, FromField)]
#[darling(attributes(debug))]
struct AutoDebugFieldsInfo {
    ident: Option<syn::Ident>,
    #[darling(default)]
    skip: bool,
}

pub(crate) fn process_auto_debug(input: DeriveInput) -> TokenStream {

    let AutoDebugInfo {
        ident,
        generics,
        data: Data::Struct(fields)
    } = AutoDebugInfo::from_derive_input(&input).unwrap()
    else {
        panic!("AutoDebug only works on structs");
    };

    let fields = fields.iter().map(|field|{
        let ident = field.ident.as_ref().unwrap();
        let skip = field.skip;

        if skip {
            quote! {}
        }else {
            quote! {
                .field(stringify!(#ident), &self.#ident)
            }
        }
    });

    TokenStream::from(quote! {
        impl ::core::fmt::Debug for #ident #generics {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                f.debug_struct(stringify!(#ident))
                    #(#fields)*
                    .finish()
            }
        }
    })
}