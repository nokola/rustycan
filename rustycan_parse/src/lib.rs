use proc_macro::TokenStream;

#[proc_macro]
pub fn parse(input: TokenStream) -> TokenStream {
    input
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
