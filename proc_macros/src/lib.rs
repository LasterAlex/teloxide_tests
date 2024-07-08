use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Changeable)]
pub fn changeable_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`
    let name = input.ident;

    // Generate an iterator over the fields
    let methods = if let Data::Struct(ref data) = input.data {
        match data.fields {
            Fields::Named(ref fields) => {
                fields.named.iter().map(|f| {
                    let field_name = &f.ident;
                    let field_type = &f.ty;

                    // Generate methods that handle Option and &str to String conversion
                    if let syn::Type::Path(type_path) = field_type {
                        let last_segment = type_path.path.segments.last().unwrap();
                        if last_segment.ident == "Option" {
                            // Idk wtf this does, but somehow i managed to make it work
                            let inner_type = if let syn::PathArguments::AngleBracketed(args) = &last_segment.arguments {
                                if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                                    inner_type
                                } else {
                                    panic!("Unsupported Option field type")
                                }
                            } else {
                                panic!("Unsupported Option field type")
                            };

                            quote! {
                                // Takes care of the string conversion for free 
                                // (and everything else that can Into for that matter)
                                pub fn #field_name<T: Into<#inner_type>>(mut self, value: T) -> Self {
                                    self.#field_name = Some(value.into());
                                    self
                                }
                            }
                        // Next is just a bunch of useful conversions, like &str to String, i64 to ChatId etc.
                        } else if last_segment.ident == "String" {
                            quote! {
                                pub fn #field_name<T: Into<String>>(mut self, value: T) -> Self {
                                    self.#field_name = value.into();
                                    self
                                }
                            }
                        } else if last_segment.ident == "ChatId" {
                            quote! {
                                pub fn #field_name(mut self, value: i64) -> Self {
                                    self.#field_name = ChatId(value);
                                    self
                                }
                            }
                        } else if last_segment.ident == "UserId" {
                            quote! {
                                pub fn #field_name(mut self, value: u64) -> Self {
                                    self.#field_name = UserId(value);
                                    self
                                }
                            }
                        } else {
                            quote! {
                                pub fn #field_name(mut self, value: #field_type) -> Self {
                                    self.#field_name = value;
                                    self
                                }
                            }
                        }
                    } else {
                        panic!("Unsupported field type")
                    }
                })
            }
            _ => panic!("Changeable macro only works on structs with named fields"),
        }
    } else {
        panic!("Changeable macro only works on structs");
    };

    // Build the output
    let expanded = quote! {
        impl #name {
            #(#methods)*
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
