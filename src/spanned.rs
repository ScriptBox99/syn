use proc_macro2::{Span, TokenStream};
use quote::{Tokens, ToTokens};

pub trait Spanned {
    /// Returns a `Span` covering the complete contents of this AST node, or
    /// `Span::call_site()` if this node is empty.
    fn span(&self) -> Span;
}

impl<T> Spanned for T
where
    T: ToTokens,
{
    fn span(&self) -> Span {
        let mut tokens = Tokens::new();
        self.to_tokens(&mut tokens);
        let token_stream = TokenStream::from(tokens);
        let mut iter = token_stream.into_iter();
        let mut span = match iter.next() {
            Some(tt) => tt.span,
            None => {
                return Span::call_site();
            }
        };
        for tt in iter {
            if let Some(joined) = span.join(tt.span) {
                span = joined;
            }
        }
        span
    }
}