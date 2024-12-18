use syn::DeriveInput;
use proc_macro::TokenStream;
use darling::{FromDeriveInput, FromField, FromVariant};
use darling::ast::{Data, Style};
use quote::quote;

#[derive(Debug, FromDeriveInput)]
struct EnumFromDarling {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<EnumVariants, ()>,
}

#[derive(Debug, FromVariant)]
struct EnumVariants {
    ident: syn::Ident,
    fields: darling::ast::Fields<EnumVariantFields>,
}

#[derive(Debug, FromField)]
struct EnumVariantFields {
    ty: syn::Type,
}


pub(crate) fn process_enum_from_darling(input: DeriveInput) -> TokenStream {
    let EnumFromDarling {
        ident,
        generics,
        data: Data::Enum(data) }
        = FromDeriveInput::from_derive_input(&input).unwrap()
    else {
        panic!("EnumFromDarling only works on enums");
    };

    let from_impls = data.iter().map(|variant| {
        let var = &variant.ident;
        let style = &variant.fields.style;
        match style {
            Style::Tuple if variant.fields.len() == 1 => {
                let field = variant.fields.iter().next().expect("should have 1 field");
                let ty = &field.ty;
                quote! {
                    impl #generics From<#ty> for #ident #generics {
                        fn from(value: #ty) -> Self {
                            #ident::#var(value)
                        }
                    }
                }
            },
            _ => quote! {},
        }
    });



    TokenStream::from(quote! {
        #(#from_impls)*
    })
}
