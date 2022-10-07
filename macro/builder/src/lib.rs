use proc_macro::{TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    // The name of the structure
    let name = &ast.ident;

    // The name of the builder for the structure
    let bname = format!("{}Builder", name);
    let bident = syn::Ident::new(&bname, name.span());

    // Get the fields of the structure
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!("We support only structs");
    };

    // The fields of the builder are the Option of the fields of the structure
    let optionized = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;

        quote! {
            #name: std::option::Option<#ty>
        }
    });

    // The setter methods in the builder.
    let setters = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;

        quote! {
            pub fn #name(&mut self, #name: #ty) -> &mut Self {
                self.#name = std::option::Option::Some(#name);
                self
            }
        }
    });

    let set_to_none = fields.iter().map(|f| {
        let name = &f.ident;

        quote! {
            #name: std::option::Option::None
        }
    });

    let set_to_bval = fields.iter().map(|f| {
        let name = &f.ident;

        quote! {
            #name: self.#name.clone().unwrap()
        }
    });


    let expanded = quote! {
        // The builder for #name structure
        pub struct #bident {
            #(#optionized,)*
        }

        impl #bident {
            #(#setters)*

            pub fn build(&mut self) -> std::result::Result<#name, std::boxed::Box<dyn std::error::Error>> {
                std::result::Result::Ok(#name {
                    #(#set_to_bval,)*
                })
            }
        }

        impl #name {
            pub fn builder() -> #bident {
                #bident {
                    #(#set_to_none,)*
                }
            }
        }
    };

    expanded.into()
}
