use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::ops::{Deref, DerefMut};
use syn::{
    parse_macro_input, punctuated::Punctuated, spanned::Spanned, Data, DataEnum, DataStruct,
    DeriveInput, Error, Fields, FieldsNamed, Result,
};

const CRATE_NAME: &'static str = "tamanegi-error";
const CRATE_USE_NAME: &'static str = "tamanegi_error";

#[proc_macro_derive(TamanegiError)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    match derive_builder(input) {
        Ok(token) => TokenStream::from(token),
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}

fn derive_builder(input: DeriveInput) -> Result<TokenStream2> {
    // - fmt::Debugを作る
    // - location, sourceメンバが存在することを確認
    // - traitの実装確認(専用traitが必要?)
    //      - AsErrorSourceが実装されているか確認
    //      - Displayが実装されているか確認

    let ident = input.ident.clone();

    match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) => struct_derive_builder(&ident, &named),
        Data::Enum(DataEnum { variants, .. }) => enum_derive_builder(&ident, &variants),
        _ => Err(Error::new(input.span(), "not supported")),
    }
}

fn has_ident(fields: &Punctuated<syn::Field, syn::token::Comma>, ident: &str) -> bool {
    fields
        .iter()
        .find(|f| f.ident.as_ref().is_some_and(|x| x == ident))
        .is_some()
}

fn crate_name() -> proc_macro2::Ident {
    let crate_name = proc_macro_crate::crate_name(CRATE_NAME).unwrap();
    let crate_name = match crate_name {
        proc_macro_crate::FoundCrate::Itself => {
            proc_macro2::Ident::new(CRATE_USE_NAME, proc_macro2::Span::call_site())
        }
        proc_macro_crate::FoundCrate::Name(name) => {
            proc_macro2::Ident::new(&name, proc_macro2::Span::call_site())
        }
    };
    crate_name
}

fn struct_derive_builder(
    ident: &syn::Ident,
    input: &Punctuated<syn::Field, syn::token::Comma>,
) -> Result<TokenStream2> {
    if !has_ident(input, "location") {
        return Err(syn::Error::new(
            input.span(),
            "location field must be exist",
        ));
    }

    let crate_name = crate_name();

    let debug_impl = quote! {
        const _: () = {
            extern crate #crate_name as _tamanegi_error;
            impl _tamanegi_error::TamanegiTrait for #ident {}

            impl ::core::fmt::Debug for #ident {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
                    where Self: _tamanegi_error::TamanegiTrait
                {
                    use ::snafu::ErrorCompat;
                    // show source error if exists
                    write!(f, "{}: {}, at {:?}\n", self.iter_chain().count() - 1, self, self.location)?;

                    if let Some(e) = self.iter_chain().nth(1) {
                        ::core::fmt::Debug::fmt(e, f)?;
                    }

                    Ok(())
                }
            }
        };
    };

    Ok(TokenStream2::from(debug_impl))
}

#[derive(Debug)]
struct EnumVariant {
    ident: syn::Ident,
    has_location: bool,
}

#[derive(Debug)]
struct EnumVariants(Vec<EnumVariant>);

impl EnumVariants {
    fn new() -> Self {
        Self(Vec::new())
    }
}

impl Deref for EnumVariants {
    type Target = Vec<EnumVariant>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EnumVariants {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn enum_derive_builder(
    ident: &syn::Ident,
    input: &Punctuated<syn::Variant, syn::token::Comma>,
) -> Result<TokenStream2> {
    // - fmt::Debugを作る
    // - locationメンバが存在することを確認
    // - traitの実装確認(専用traitが必要?)
    //      - AsErrorSourceが実装されているか確認
    //      - Displayが実装されているか確認

    let mut variants = EnumVariants::new();

    // enumのすべてのVariant内にlocationフィールドが存在することを確認
    // sourceはleafの場合はないこともある
    // TODO: syn::Parseにまとめる
    for v in input.iter() {
        if match &v.fields {
            Fields::Named(FieldsNamed { named, .. }) => has_ident(named, "location"),
            _ => {
                // error: non member variant
                false
            }
        } {
            Ok(())
        } else {
            Err(syn::Error::new(
                v.ident.span(),
                "location field must be exist",
            ))
        }?;

        variants.push(EnumVariant {
            ident: v.ident.clone(),
            has_location: true,
        });
    }

    let debug_impl = variants.iter().map(|v| {
        let variant_ident = v.ident.clone();

        if v.has_location {
            quote! {
                #ident::#variant_ident { location, .. } => {
                    write!(f, "{:?}", location)?;
                }
            }
        } else {
            quote! {
                #ident::#variant_ident { .. } => {
                    write!(f, "Unknown")?;
                }
            }
        }
    });

    let crate_name = crate_name();

    // impl Drbug
    // TODO: support generics
    let debug_impl = quote! {
        const _: () = {
            extern crate #crate_name as _tamanegi_error;

            impl _tamanegi_error::TamanegiTrait for #ident {}

            impl ::core::fmt::Debug for #ident {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
                    where Self: _tamanegi_error::TamanegiTrait
                {
                    use ::snafu::ErrorCompat;
                    // show source error if exists
                    write!(f, "{}: {}, at ", self.iter_chain().count() - 1, self)?;

                    match self {
                        #(#debug_impl),*
                    }

                    write!(f, "\n")?;

                    if let Some(e) = self.iter_chain().nth(1) {
                        ::core::fmt::Debug::fmt(e, f)?;
                    }

                    Ok(())
                }
            }
        };
    };

    Ok(TokenStream2::from(debug_impl))
}
