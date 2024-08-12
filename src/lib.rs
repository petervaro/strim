#![doc = include_str!("../README.md")]

mod bytes;
mod string;
mod byte_string;
mod join_with;
mod arguments;

use std::mem;
use syn::parse_macro_input;
use proc_macro::{TokenStream, TokenTree, Literal};
use self::arguments::Arguments;

fn trimmed_string_joined_with_delimiter(
    string: &str,
    delimiter: &str,
) -> String {
    let mut collected = String::with_capacity(string.len());
    let mut lines = string::Lines::from(string);

    if let Some(line) = lines.next() {
        collected.push_str(line);

        for line in lines {
            collected.push_str(delimiter);
            collected.push_str(line);
        }
    }

    collected
}

fn trimmed_byte_string_joined_with_delimiter(
    byte_string: &[u8],
    delimiter: &[u8],
) -> Vec<u8> {
    let mut collected = Vec::with_capacity(byte_string.len());
    let mut lines = byte_string::Lines::from(byte_string);

    if let Some(line) = lines.next() {
        collected.extend(line);

        for line in lines {
            collected.extend(delimiter);
            collected.extend(line);
        }
    }

    collected
}

/// [`trim!`] can be used on any string or byte-string literals to remove all
/// blank lines and trim each line's leading and trailing whitespace.
///
/// When `trim` processes a string literal or a byte-string, under the hood it
/// uses [`str::trim`] or [`<[u8]>::trim_ascii`][ta] respectively.
///
/// When `trim` processes a byte-string literal, it considers either newline
/// characters (`\n`) or sequences of carriage return followed by a line feed
/// (`\r\n`) as line boundaries.  (This also means that carriage return (`\r`)
/// not immediately followed by a line feed (`\n`) is not considered a line
/// break.)
///
/// In both string and byte-string literals, whether a line is blank is
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
///
/// The `trim` macro also accepts a named parameter called `join_with` which can
/// be used to specify the _delimiter_ with which the lines are joined together.
/// If the input is a string literal, the delimiter is expected to be either a
/// character literal or a string literal.  If the input is a byte-string
/// literal, the delimiter is expected to be either a byte literal or a
/// byte-string literal.
///
/// # Example
///
/// ```
/// # use strim::trim;
/// let expected = "Alpha<br/>Beta and Gamma<br/>Delta, Epsilon, and Zeta";
/// let actual = trim!(
///     "Alpha
///      Beta and Gamma
///      Delta, Epsilon, and Zeta",
///     join_with = "<br/>",
/// );
/// assert_eq!(actual, expected);
/// ```
///
/// [ta]: https://doc.rust-lang.org/std/primitive.slice.html#method.trim_ascii
#[proc_macro]
pub fn trim(stream: TokenStream) -> TokenStream {
    let arguments = parse_macro_input!(stream as Arguments);

    let token_tree = match arguments {
        Arguments::String {
            input,
            delimiter: string::Delimiter::Character(delimiter),
        } => {
            let mut buffer = [0; mem::size_of::<char>()];
            let delimiter = delimiter.encode_utf8(&mut buffer);
            let string = trimmed_string_joined_with_delimiter(
                &input,
                delimiter,
            );

            TokenTree::from(Literal::string(&string))
        },
        Arguments::String {
            input,
            delimiter: string::Delimiter::String(delimiter),
        } => {
            let string = trimmed_string_joined_with_delimiter(
                &input,
                &delimiter,
            );

            TokenTree::from(Literal::string(&string))
        },
        Arguments::ByteString {
            input,
            delimiter: byte_string::Delimiter::Byte(delimiter),
        } => {
            let byte_string = trimmed_byte_string_joined_with_delimiter(
                &input,
                &[delimiter],
            );

            TokenTree::from(Literal::byte_string(&byte_string))
        },
        Arguments::ByteString {
            input,
            delimiter: byte_string::Delimiter::ByteString(delimiter),
        } => {
            let byte_string = trimmed_byte_string_joined_with_delimiter(
                &input,
                &delimiter,
            );

            TokenTree::from(Literal::byte_string(&byte_string))
        },
    };

    token_tree.into()
}
