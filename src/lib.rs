#![doc = include_str!("../README.md")]

use proc_macro::{TokenStream, TokenTree, Literal, Ident, Punct, Spacing, Group, Delimiter, Span};
use litrs::StringLit;

trait ToCompileError {
    fn to_compile_error<S>(self, message: S) -> TokenStream
    where
        S: AsRef<str>;
}

impl ToCompileError for Span {
    fn to_compile_error<S>(self, message: S) -> TokenStream
        where
            S: AsRef<str>,
    {
        let token_trees = [
            TokenTree::from(Ident::new("compile_error", self)),
            TokenTree::from(Punct::new('!', Spacing::Alone)),
            TokenTree::from(Group::new(
                Delimiter::Parenthesis,
                TokenTree::from(Literal::string(message.as_ref())).into(),
            ))
        ];

        TokenStream::from_iter(
            token_trees.into_iter().map(|mut token| {
                token.set_span(self);
                token
            })
        )
    }
}

/// [`trim!`] can be used on any string literals to remove all blank lines and
/// trim each line's leading and trailing whitespace.  Under the hood `trim`
/// uses [str::trim], i.e. whatever that method considers as whitespace will
/// be trimmed by this macro.  Whether a line is blank is considered after it
/// has been trimmed, that is, if a line contains whitespace only, it will
/// removed.
///
/// # Example
///
/// ```
/// # use strim::trim;
/// let expected = r#"<a x="x"><b y="y"><c z="z">Hello, World!</c></b></a>"#;
/// let actual = trim!(r#"
///     <a x="x">
///         <b y="y">
///             <c z="z">
///                 Hello, World!
///             </c>
///         </b>
///     </a>
/// "#);
/// assert_eq!(actual, expected);
/// ```
#[proc_macro]
pub fn trim(stream: TokenStream) -> TokenStream {
    let macro_span = Span::call_site();
    let mut stream = stream.into_iter();
    let Some(literal) = stream.next() else {
        return macro_span.to_compile_error(
            "`trim` expects a single string literal, but got none",
        );
    };

    if let Some(redundant) = stream.next() {
        return redundant.span().to_compile_error(format!(
            "`trim` expects a single string literal, \
             but got another argument: `{redundant}`",
        ));
    }

    let literal_span = literal.span().clone();
    let literal = match StringLit::try_from(literal) {
        Ok(literal) => literal,
        Err(invalid_token) => return literal_span.to_compile_error(format!(
            "`trim` expects a single string literal, \
             but got: {invalid_token}",
        )),
    };

    let mut collected = String::new();
    for line in literal.value().lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            collected.push_str(trimmed);
        }
    }

    TokenTree::from(Literal::string(&collected)).into()
}
