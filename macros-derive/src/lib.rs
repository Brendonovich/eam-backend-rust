use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::ItemStruct;
use syn::PatPath;

#[proc_macro_attribute]
pub fn iter_object(arg: TokenStream, input: TokenStream) -> TokenStream {
    let arg = parse_macro_input!(arg as PatPath);
    let ast = parse_macro_input!(input as ItemStruct);

    impl_iter_object_macro(arg, ast)
}

fn impl_iter_object_macro(arg: PatPath, ast: ItemStruct) -> TokenStream {
    let ident_name = &ast.ident;
    let mut fileds = quote!();
    for f in ast.clone().fields {
        let field_name = f.ident.unwrap();
        fileds.extend(quote!(self.#field_name.map(#arg::#field_name::set), ))
    }
    let gen = quote! {
        #ast
        impl macros::IterObject<#arg::SetParam> for #ident_name {
            fn to_params(self) -> Vec<#arg::SetParam> {
                [#fileds].into_iter().flatten().collect()
            }
        }
    };
    gen.into()
}