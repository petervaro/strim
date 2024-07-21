#![doc = include_str!("../README.md")]

mod bytes;

use proc_macro::{
    self,
    TokenStream,
    TokenTree,
    Ident,
    Punct,
    Spacing,
    Group,
    Delimiter,
    Span,
};
use litrs::{
    self,
    StringLit,
    ByteStringLit,
    Buffer,
};
use self::bytes::Bytes as _;

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
                {
                    let literal = proc_macro::Literal::string(message.as_ref());
                    TokenTree::from(literal).into()
                },
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

fn trimmed_string_literal<B>(literal: StringLit<B>) -> TokenTree
where
    B: Buffer,
{
    let mut collected = String::new();
    for line in literal.value().lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            collected.push_str(trimmed);
        }
    }

    TokenTree::from(proc_macro::Literal::string(&collected))
}

fn trimmed_byte_string_literal<B>(literal: ByteStringLit<B>) -> TokenTree
where
    B: Buffer,
{
    let mut collected = Vec::new();
    for line in literal.value().lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            collected.extend_from_slice(trimmed);
        }
    }

    TokenTree::from(proc_macro::Literal::byte_string(&collected))
}

/// [`trim!`] can be used on any string or byte-string literals to remove all
/// blank lines and trim each line's leading and trailing whitespace.
///
/// When `trim` processes a string literal, under the hood it  uses [str::trim],
/// i.e. whatever that method considers as whitespace will be trimmed by this
/// macro.
///
/// When `trim` processes a byte-string literal, it considers either newline
/// characters (`\n`) or sequences of carriage return followed by a line feed
/// (`\r\n`) as line boundaries.  (This also means that carriage return (`\r`)
/// not immediately followed by a line feed (`\n`) is not considered a line
/// break.)  `trim` considers space (` `), horizontal tab (`\t`), vertical
/// tab (`\v`), and form feed (`\f`) characters as whitespace.
///
/// In both string and byte-string literals, Whether a line is blank is
/// considered after it has been trimmed, that is, if a line contains whitespace
/// only, it will removed.
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
    let span = Span::call_site();
    let mut stream = stream.into_iter();
    let Some(literal) = stream.next() else {
        return span.to_compile_error(
            "`trim` expects a single string or byte-string literal, \
             but got none",
        );
    };

    if let Some(redundant) = stream.next() {
        return redundant.span().to_compile_error(format!(
            "`trim` expects a single string or byte-string literal, \
             but got another argument: `{redundant}`",
        ));
    }

    let span = literal.span().clone();
    // HACK: Unfortunately this is really, really quirky here -- there is no way
    //       to get the source as a string-slice, which is unfortunately needed
    //       by `litrs::Literal::parse` (which requires either `String` or
    //       `&str`), so we rely on `TokenTree`'s `Display` implementation which
    //       gives us `ToString::to_string`.  Even `Span::source_text` is
    //       admittedly just a "best effort" based on the documentation so we
    //       cannot use that either.  Hopefully either `litrs` will evolve or
    //       the horrific `proc_macro` interface will.  Regardless whicever
    //       happens first, we will be able to do something more reliable here
    // SEE: https://doc.rust-lang.org/proc_macro/enum.TokenTree.html#impl-Display-for-TokenTree
    // SEE: https://doc.rust-lang.org/proc_macro/struct.Span.html#method.source_text
    let literal = literal.to_string();

    let literal = match litrs::Literal::parse(literal.as_str()) {
        Ok(literal) => literal,
        // NOTE: For the time being we do not use `error.span()` because that is
        //       returned as a `Range` instead of `proc_macro::Span` and the
        //       latter cannot be constructed from the former.  When `litrs`
        //       improves its ergonomics, we should be able to provide better
        //       error location.  (That being said, I don't quite see how we
        //       would hit this error, considering that `literal` is turned into
        //       string from an already parsed `TokenTree`)
        Err(error) => return span.to_compile_error(format!(
            "`trim` cannot parse literal: {error}"
        ))
    };

    let token_tree = match literal {
        litrs::Literal::String(string) => trimmed_string_literal(string),
        litrs::Literal::ByteString(bytes) => trimmed_byte_string_literal(bytes),
        _ => return span.to_compile_error(format!(
            "`trim` expects a single string or byte-string literal, \
             but got: {literal}",
        ))
    };

    token_tree.into()
}
