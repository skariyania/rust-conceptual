use proc_macro::TokenStream;
use quote::quote;
use syn;

fn impl_whoami_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl WhoAmI for #name {
            fn whoami() {
                println!("@whoami type={}", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(WhoAmI)]
pub fn whoami_macro_derive(input: TokenStream) -> TokenStream {
    // construct representation of rust code as abstract syntax tree
    let ast = syn::parse(input).unwrap();

    //build trait implementaion
    impl_whoami_macro(&ast)
}
