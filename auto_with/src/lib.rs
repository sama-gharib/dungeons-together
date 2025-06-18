//! # Introduction
//! This crate contains the `with` procedural macro which enables to declare "with methodes" with fewer keystrokes. 
//! # Exemple
//! ```
//! # use auto_with::with;
//! #[derive(Default)]
//! struct Person { name: String, age: usize }
//! 
//! impl Person { with!{ name: String } with!{ age: usize } }
//!
//! # fn main() {
//! let claire = Person::default()
//!     .with_name(String::from("Claire"))
//!     .with_age(58);
//! # }
//! ```

use proc_macro::{
    TokenStream,
    TokenTree
};

#[proc_macro]
pub fn with(stream: TokenStream) -> TokenStream {
    let to_return;

    let mut stream = stream.into_iter();
    let tokens = (stream.next(), stream.next(), stream.next());

    if stream.next().is_some() {
        panic!("More than 3 tokens in argument.");
    }

    if let (
        Some(TokenTree::Ident (name)),
        Some(TokenTree::Punct (punct)),
        Some(typ)
    ) = tokens {
        if punct.to_string() == ":" {
            to_return = format!("pub fn with_{name}(mut self, {name}: {typ}) -> Self {{ self.{name} = {name}; self }}").parse().unwrap();
        } else {
            panic!("Expected a semicolon, found '{}'.", punct);
        }
    } else {
        panic!("Ill-formated argument, expected : `[attribute] : [type]`")
    }

    to_return
}
