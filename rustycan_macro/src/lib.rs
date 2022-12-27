mod parse;

use parse::Elem;
use proc_macro2::{TokenStream, TokenTree};
use syn::parse_macro_input;

use crate::parse::ElemParam;

#[proc_macro]
pub fn rustycan_ui(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if input.is_empty() {
        return proc_macro::TokenStream::new();
    }

    // let parsed: Result<syn::token::Type, syn::Error> = syn::parse(input);
    let parsed: Elem = parse_macro_input!(input as Elem);
    // let ident: Ident = Ident::new("asd", Span::new());
    // ident.span().unwrap().error("hoho").emit();

    // let input = TokenStream::from(input); // convert to proc_macro2's TokenStream to allow using Ident in quote
    // print(input.into());
    print_elem(parsed);
    proc_macro::TokenStream::new()
    // todo!()
}

fn print_elem(elem: Elem) {
    println!("name: {name}", name = elem.name.to_string());
    for param in elem.params {
        match param {
            ElemParam::Property(prop) => {
                println!("prop {name} = {value}", name = prop.name, value = "[Expr]");
            }
            ElemParam::ChildElem(elem) => print_elem(elem),
        }
    }
}
fn _print(input: TokenStream) {
    for t in input {
        if let TokenTree::Group(g) = t {
            println!("{:?}: open {:?}", g.span_open(), g.delimiter());
            _print(g.stream());
            println!("{:?}: close {:?}", g.span_close(), g.delimiter());
        } else {
            // t.span().unwrap().error("sdf").emit();
            println!("{:?}: {}", t.span(), t.to_string());
        }
    }
}
