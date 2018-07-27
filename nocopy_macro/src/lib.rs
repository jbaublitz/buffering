extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_derive(NoCopy)]
pub fn no_copy(input: TokenStream) -> TokenStream {
    let mut ast: syn::DeriveInput = syn::parse(input).expect("Failed to parse input");

    let name = ast.ident;
    let attr = ast.attrs.swap_remove(0);
    match attr.style {
        syn::AttrStyle::Outer => (),
        _ => panic!("Outer attribute in the form #[name = \"UnionNameHere\"] required"),
    };
    let attrnamemeta = attr.interpret_meta();
    let attrname = match attrnamemeta {
        Some(syn::Meta::NameValue(syn::MetaNameValue { ident, eq_token: _, lit: syn::Lit::Str(s) })) => {
            if ident != syn::Ident::new("name", proc_macro2::Span::call_site()) {
                panic!("Unrecognized attribute");
            }
            syn::Ident::new(s.value().as_str(), proc_macro2::Span::call_site())
        },
        _ => panic!("Outer attribute in the form #[name = \"UnionNameHere\"] required"),
    };

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
        }
    };
    tokens.into()
}
