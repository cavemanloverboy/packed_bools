use proc_macro::TokenStream;
use std::iter::FromIterator;
use syn::spanned::Spanned;

use proc_macro2::Ident;
use quote::quote;
use syn::__private::TokenStream2;
use syn::{Data, DeriveInput, Type};

pub fn packed_bools(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let struct_generics = &input.generics;

    let fields = match &input.data {
        Data::Struct(struct_data) => &struct_data.fields,
        _ => panic!("packed_bools can only be used on a struct"),
    };

    let getters_and_setters: Vec<TokenStream2> = fields.iter().map(|field| {

        // Initialize container with bools to pack
        let mut bools_to_pack = Vec::with_capacity(8);

        for attr in &field.attrs {

            // Get name of attribute and check that it is valid while gathering bools to pack
            let attr_name = attr.path.segments.first()
                .unwrap_or_else(|| panic!("packed_bools: Invalid path")).ident.to_string();

            match attr_name.as_str() {
                "pack_bools" => {

                    // Check that attr is on a BitPack
                    match field.ty {
                        Type::Path(ref p) => {
                            if p.path.segments.last().expect("can only use pack_bool on u8s").ident != "u8" {
                                return quote!{ compile_error!("can only use pack_bool on u8s" ) }
                            }
                        }
                        _ => return quote!{ compile_error!("can only use pack_bool on u8s") },
                    };

                    // Check for parenthesis
                    let attr_token_string = attr.tokens.to_string().clone();
                    if &attr_token_string[..1] != "(" {
                        return quote ! { compile_error!("packed_bools: Expected '(' after 'pack_bools'. Sample usage: #[pack_bools(active, admin)] for two bools active, admin.") };
                    }
                    if &attr_token_string[attr_token_string.len() - 1..] != ")" {
                        return quote! { compile_error!("packed_bools: Expected ')' to close 'pack_bools'. Sample usage: #[pack_bools(active, admin)] for two bools active, admin.") };
                    }
                    let arguments_string = &attr_token_string[1..attr_token_string.len() - 1];
                    let arguments: Vec<&str> = arguments_string.split(',').map(|s| s.trim()).collect();

                    if arguments.is_empty() || arguments.len() > 8 {
                        return quote! { compile_error!("Invalid number of attribute arguments. One u8 can pack 1-8 bools. Sample usage: #[pack_bools(active, admin)] for two bools active, admin.") };
                    }

                    // Gather all bools to pack
                    for (argument_index, argument) in arguments.iter().enumerate() {
                        bools_to_pack.push((field.ident.clone().expect("tuple structs or unit structs not supported"), quote! { #argument }, argument_index));
                    }
                }
                "doc" => {
                    // skip doc attributes
                }
                _ => {
                    panic!("packed_bools: Unhandled attribute '{}'. Only supported attribute is pack_bools", attr_name);
                }
            }
        };

        // Construct getter and setters for every bool to pack
        let getters_and_setters = bools_to_pack
            .into_iter()
            .map(|(field, ts, bit_idx)| {

                // Change type and split generics fo rimpl
                let bit_idx: u32 = bit_idx as u32;
                let (impl_generics, ty_generics, where_clause) = struct_generics.split_for_impl();

                // Getter name, made unique because every bool should have a unique name
                let getter_name = Ident::new(&format!("get_{}", ts.to_string().replace('\"', "")), ts.span());
                let getter = quote! {

                    impl #impl_generics #struct_name #ty_generics #where_clause {
                        pub fn #getter_name(&self) -> bool {
                            self.#field & 2_u8.pow(#bit_idx) == 2_u8.pow(#bit_idx)
                        }
                    }

                };
                // Setter name, made unique because every bool should have a unique name
                let setter_name = Ident::new(&format!("set_{}", ts.to_string().replace('\"', "")), ts.span());
                let setter = quote! {

                    impl #impl_generics #struct_name #ty_generics #where_clause  {
                        pub fn #setter_name(&mut self, value: bool) {

                            // Retrieve current bit value
                            let current_value: bool = self.#field & 2_u8.pow(#bit_idx) == 2_u8.pow(#bit_idx);

                            if value == current_value {
                                // If value equals the current value do nothing
                            } else if value {
                                // if they differ and value is true then current value is false so add
                                self.#field += 2_u8.pow(#bit_idx);
                            } else {
                                // if they differ and value is false then current value is true so subtract
                                self.#field -= 2_u8.pow(#bit_idx);
                            }
                        }
                    }
                };
                quote! {

                    #getter

                    #setter

                }
            });

        TokenStream2::from_iter(getters_and_setters)
    }).collect();

    TokenStream2::from_iter(getters_and_setters).into()
}
