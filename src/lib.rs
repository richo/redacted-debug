extern crate proc_macro;

use proc_macro2::{TokenStream, Ident};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Variant, Index};

#[proc_macro_derive(RedactedDebug, attributes(redacted))]
pub fn derive_redacted_debug(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let body = redacted_debug_body(&name, &input.data);

    #[cfg(feature = "std")]
    let (trayt, formatter) = (quote!(::std::fmt::Debug), quote!(::std::fmt::Formatter));

    #[cfg(not(feature = "std"))]
    let (trayt, formatter) = (quote!(::core::fmt::Debug), quote!(::core::fmt::Formatter));


    let expanded = quote! {
        impl #impl_generics #trayt for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut #formatter) -> ::std::fmt::Result {
                    #body
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

fn is_redacted(attrs: &Vec<syn::Attribute>) -> bool {
   for attr in attrs {
       if attr.path.is_ident("redacted") {
           return true
       }
   }
   false
}

fn redacted_debug_body(name: &Ident, data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        if is_redacted(&f.attrs) {
                            quote_spanned! {f.span()=>
                                .field(stringify!(#name), &"\"...\"")
                            }
                        } else {
                            quote_spanned! {f.span()=>
                                .field(stringify!(#name), &self.#name)
                            }
                        }
                    });
                    quote! {
                        f.debug_struct(stringify!(#name))
                        #(#recurse)*
                        .finish()
                    }
                }
                Fields::Unnamed(ref fields) => {
                    let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let index = Index::from(i);
                        if is_redacted(&f.attrs) {
                            quote_spanned! {f.span()=>
                                .field(&"\"...\"")
                            }
                        } else {
                            quote_spanned! {f.span()=>
                                .field(&self.#index)
                            }
                        }
                    });
                    quote! {
                        f.debug_tuple(stringify!(#name))
                        #(#recurse)*
                        .finish()
                    }
                }
                Fields::Unit => {
                    quote! {
                        write!(f, stringify!(#name))
                    }
                }
            }
        }
        Data::Enum(ref data) => {
            let variants = data.variants.iter().map(|v| {
                let name = &v.ident;
                match v.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        if is_redacted(&f.attrs) {
                            quote_spanned! {f.span()=>
                                .field(stringify!(#name), &"\"...\"")
                            }
                        } else {
                            quote_spanned! {f.span()=>
                                .field(stringify!(#name), &self.#name)
                            }
                        }
                    });
                    quote! {
                        f.debug_struct(stringify!(#name))
                        #(#recurse)*
                        .finish()
                    }
                }
                Fields::Unnamed(ref fields) => {
                    let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let index = Index::from(i);
                        if is_redacted(&f.attrs) {
                            quote_spanned! {f.span()=>
                                .field(&"\"...\"")
                            }
                        } else {
                            quote_spanned! {f.span()=>
                                .field(&self.#index)
                            }
                        }
                    });
                    quote! {
                        #name => {
                            f.debug_tuple(stringify!(#name))
                                #(#recurse)*
                                .finish()
                        }
                    }
                }
                Fields::Unit => {
                    quote! {
                        #name => {
                            write!(f, stringify!(#name))
                        }
                    }
                }
            }
            });
            quote! {
                match self {
                    #(#variants,)*
                }
            }
        }
        Data::Union(_) => panic!("this trait cannot be derived for unions"),
    }
}
