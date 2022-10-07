use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

fn ty_is_option<'a>(ty: &'a syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(ref p) = ty {
        if p.path.segments.len() == 1 && p.path.segments[0].ident == "Option" {
            if let syn::PathArguments::AngleBracketed(ref inner_ty) = p.path.segments[0].arguments {
                if inner_ty.args.len() == 1 {
                    let inner_ty = inner_ty.args.first().unwrap();
                    if let syn::GenericArgument::Type(ref ty) = inner_ty {
                        return Some(ty)
                    }
                }
            }
        }
    }

    None
}

fn ty_is_vec<'a>(ty: &'a syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(ref p) = ty {
        if p.path.segments.len() == 1 && p.path.segments[0].ident == "Vec" {
            if let syn::PathArguments::AngleBracketed(ref inner_ty) = p.path.segments[0].arguments {
                if inner_ty.args.len() == 1 {
                    let inner_ty = inner_ty.args.first().unwrap();
                    if let syn::GenericArgument::Type(ref ty) = inner_ty {
                        return Some(ty)
                    }
                }
            }
        }
    }

    None
}

#[proc_macro_derive(Builder, attributes(builder))]
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

        if ty_is_option(ty).is_some() {
            quote! {
                #name: #ty
            }
        } else {
            quote! {
                #name: std::option::Option<#ty>
            }
        }
    });

    // The setter methods in the builder.
    let setters = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;

        if let Some(inner_ty) = ty_is_option(ty) {
            quote! {
                pub fn #name(&mut self, #name: #inner_ty) -> &mut Self {
                    self.#name = Some(#name);
                    self
                }
            }
        } else {
            quote! {
                pub fn #name(&mut self, #name: #ty) -> &mut Self {
                    self.#name = std::option::Option::Some(#name);
                    self
                }
            }
        }
    });

    let extend_setters = fields.iter().filter_map(|f| {

        let ty = &f.ty;

        if let Some(inner_ty) = ty_is_vec(ty) {
            for attr in &f.attrs {
                if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "builder" {
    
                    if let Some(proc_macro2::TokenTree::Group(g)) = attr.tokens.clone().into_iter().next() {
                        let mut tokens = g.stream().into_iter();
                        match tokens.next().unwrap() {
                            TokenTree::Ident(ref _i) => (),
                            _tt => panic!("Not each")
                        }
    
                        match tokens.next().unwrap() {
                            TokenTree::Punct(ref _p) => (),
                            _tt => panic!("Not =")
                        }
    
                        match tokens.next().unwrap() {
                            TokenTree::Literal(l) => {
                                match syn::Lit::new(l) {
                                    syn::Lit::Str(s) => {
                                        let arg = syn::Ident::new(&s.value(), s.span());
                                        
                                        return Some(quote! {
                                            pub fn #arg(&mut self, value: #inner_ty) -> &mut Self {
                                                self
                                            }
                                        });
                                    },
                                    _l => panic!("Not a string literal")
                                }
                            },
                            _tt => panic!("Not a literal")
                        }
                    }
                }
            }
        }

        None
    });

    let set_to_none = fields.iter().map(|f| {
        let name = &f.ident;

        quote! {
            #name: std::option::Option::None
        }
    });

    let build_fields = fields.iter().map(|f| {
        let name = &f.ident;
        if ty_is_option(&f.ty).is_some() {
            quote! {
                #name: self.#name.clone()
            }
        } else {
            quote! {
                #name: self.#name.clone().ok_or(concat!(stringify!(#name), " is not set"))?
            }
        }
    });

    let expanded = quote! {
        // The builder for #name structure
        pub struct #bident {
            #(#optionized,)*
        }

        impl #bident {
            #(#setters)*

            #(#extend_setters)*

            pub fn build(&mut self) -> std::result::Result<#name, std::boxed::Box<dyn std::error::Error>> {
                std::result::Result::Ok(#name {
                    #(#build_fields,)*
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
