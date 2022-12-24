use proc_macro::TokenStream;

#[proc_macro]
pub fn rustycan_ui(input: TokenStream) -> TokenStream {
    // let parsed = syn::parse(tokens);
    dbg!(input.to_string());
    // TokenStream::new()
    todo!()
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
