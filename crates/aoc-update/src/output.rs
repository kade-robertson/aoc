use proc_macro2::TokenStream;
use quote::quote;

/// Modified from: https://github.com/nu11ptr/flexgen
pub fn question_comment(comment: &str) -> TokenStream {
    let mut buffer = String::new();

    for line in comment.lines() {
        if !line.is_empty() {
            buffer.push(' ');
        }
        buffer.push_str(line);
        buffer.push('\n');
    }

    let doc_comment: Vec<_> = buffer.lines().collect();
    quote! { #( #[doc = #doc_comment] )* }
}
