use {
    darling::FromDeriveInput,
    proc_macro::TokenStream,
    quote::{format_ident, quote},
    syn::{parse_macro_input, DeriveInput},
};

#[derive(FromDeriveInput)]
#[darling(attributes(kind))]
struct Opts {
    class: String,
}

#[proc_macro_derive(Kind, attributes(kind))]
pub fn kind_macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = Opts::from_derive_input(&input).expect("Wrong options");
    let class = opts.class;
    if class.is_empty() {
        panic!("kind class can't be empty")
    }
    for c in class.chars() {
        if !c.is_ascii_alphanumeric() {
            panic!("Invalid character {c:?} in kind class {class:?}");
        }
    }
    let class_const = format_ident!("TYPID_CLASS_{}", class);
    let DeriveInput { ident, .. } = input;
    let gen = quote! {
        pub static #class_const: IdClass = IdClass::new(#class);
        impl Identifiable for #ident {
            fn class() -> IdClass {
                #class_const
            }
        }
    };
    gen.into()
}
