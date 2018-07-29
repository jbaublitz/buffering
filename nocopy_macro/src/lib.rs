extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_derive(NoCopy)]
pub fn no_copy(input: TokenStream) -> TokenStream {
    enum Endian {
        Big,
        Little,
    }

    let ast: syn::DeriveInput = syn::parse(input).expect("Failed to parse input");

    let name = ast.ident;
    let mut attrname = syn::Ident::new(format!("{}Buffer", name).as_str(), proc_macro2::Span::call_site());
    let mut endian = Endian::Big;
    for attr in ast.attrs {
        match attr.style {
            syn::AttrStyle::Outer => (),
            _ => panic!("Only outer attributes allowed here"),
        };
        let attrnamemeta = attr.interpret_meta();
        match attrnamemeta {
            Some(syn::Meta::NameValue(syn::MetaNameValue { ident, eq_token: _, lit: syn::Lit::Str(s) })) => {
                if ident == syn::Ident::new("name", proc_macro2::Span::call_site()) {
                    attrname = syn::Ident::new(s.value().as_str(), proc_macro2::Span::call_site());
                }
                if ident == syn::Ident::new("endian", proc_macro2::Span::call_site()) {
                    match s.value().as_str() {
                        "big" => (),
                        "little" => { endian = Endian::Little; },
                        _ => panic!("Unrecognized \"endian\" option"),
                    }
                }
            },
            _ => panic!("Outer attribute must be in the form #[key = \"value\"]"),
        };
    }

    let fields = match ast.data {
        syn::Data::Struct(structure) => structure.fields,
        _ => panic!("This macro only supports structs"),
    };
    let field_pairs = match fields {
        syn::Fields::Named(named) => named.named,
        _ => panic!("This macro only supports structs with named fields"),
    };

    let mut funcs_vec = Vec::new();
    for named_field in field_pairs {
        let ident = &named_field.ident;
        let get_ident = syn::Ident::new(format!("get_{}", named_field.ident.as_ref().expect("All fields must be named")).as_str(), proc_macro2::Span::call_site());
        let set_ident = syn::Ident::new(format!("set_{}", named_field.ident.as_ref().expect("All fields must be named")).as_str(), proc_macro2::Span::call_site());
        let ty = named_field.ty;
        funcs_vec.push(
            quote! {
                pub fn #get_ident(&self) -> #ty {
                    unsafe { self.structure.#ident }
                }

                pub fn #set_ident(&mut self, v: #ty) {
                    unsafe { self.structure.#ident = v; }
                }
            }
        );
    }

    let tokens = quote! {
        use std::mem;

        #[derive(Copy,Clone)]
        #[repr(C)]
        pub union #attrname {
            structure: #name,
            buffer: [u8; mem::size_of::<#name>()]
        }

        impl #attrname {
            pub fn new_buffer(buffer: [u8; mem::size_of::<#name>()]) -> Self {
                #attrname { buffer }
            }

            pub fn as_buffer(&self) -> &[u8] {
                unsafe { &self.buffer }
            }

            #(
                #funcs_vec
            )*
        }
    };
    tokens.into()
}
