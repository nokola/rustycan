use proc_macro2::{Ident, TokenStream, TokenTree};

#[proc_macro]
pub fn rustycan_ui(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // let parsed: Result<syn::token::Type, syn::Error> = syn::parse(input);
    let input = TokenStream::from(input); // convert to proc_macro2's TokenStream to allow using Ident in quote

    // let ident: Ident = Ident::new("asd", Span::new());
    // ident.span().unwrap().error("hoho").emit();

    print(input);
    proc_macro::TokenStream::new()
    // todo!()
}

fn print(input: TokenStream) {
    for t in input {
        if let TokenTree::Group(g) = t {
            println!("{:?}: open {:?}", g.span_open(), g.delimiter());
            print(g.stream());
            println!("{:?}: close {:?}", g.span_close(), g.delimiter());
        } else {
            // t.span().unwrap().error("sdf").emit();
            println!("{:?}: {}", t.span(), t.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        assert_eq!(4, 4);
    }
}
