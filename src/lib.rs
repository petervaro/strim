#![doc = include_str!("../README.md")]

mod bytes;
mod string;
mod c_string;
mod byte_string;
mod join_with;
mod arguments;

use std::{mem, ffi::CStr};
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

fn trimmed_c_string_joined_with_delimiter(
    c_string: &[u8],
    delimiter: &[u8],
) -> Vec<u8> {
    let mut collected = Vec::with_capacity(c_string.len());
    let mut lines = c_string::Lines::from(c_string);

    if let Some(line) = lines.next() {
        collected.extend(line);

        for line in lines {
            collected.extend(delimiter);
            collected.extend(line);
        }
    }

    collected.push(b'\0');

    collected
}

/// [`trim!`] can be used on any string, byte-string, and C-string literals to
/// remove all blank lines and trim each line's leading and trailing whitespace.
///
/// To remove whitespace, under the hood `trim` uses [`str::trim`] for strings,
/// and [`<[u8]>::trim_ascii`][ta] for both byte-strings and C-strings.
///
/// When `trim` processes a byte-string or a C-string literal, it considers
/// either newline characters (`\n`) or sequences of carriage return followed by
/// a line feed (`\r\n`) as line boundaries.  (This also means that carriage
/// return (`\r`) not immediately followed by a line feed (`\n`) is not
/// considered a line break.)
///
/// In all scenarios, whether a line is blank is considered after it has been
/// trimmed, that is, if a line contains whitespace only, then it will be
/// considered as blank and therefore ignored.
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
/// character or a string literal.  If the input is a byte-string or a C-string
/// literal, the delimiter is expected to be either a byte literal for both, or
/// a byte-string literal for the former and a C-string literal for the latter.
///
/// For C-strings, if the delimiter is a byte literal, that cannot be the
/// nul-terminator.  `trim` produces a compile-time error if it is.  In case the
/// delimiter is a C-string, its nul-terminator will be omitted when joining the
/// trimmed lines together, i.e. the resulting C-string literal will have one
/// nul-terminator at the end.
///
/// > **N.B.** Only non-blank lines are joined together, i.e. the delimiter will
/// > only be inserted between two non-blank lines.  What is considered as blank
/// > line is explained above.
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
        Arguments::CString {
            input,
            delimiter: c_string::Delimiter::Byte(delimiter),
        } => {
            let bytes = trimmed_c_string_joined_with_delimiter(
                input.to_bytes(),
                &[delimiter],
            );
            let c_string =
                CStr::from_bytes_with_nul(&bytes)
                    .expect("Invalid bytes for a C-string");

            TokenTree::from(Literal::c_string(c_string))
        },
        Arguments::CString {
            input,
            delimiter: c_string::Delimiter::CString(delimiter),
        } => {
            let bytes = trimmed_c_string_joined_with_delimiter(
                input.to_bytes(),
                delimiter.to_bytes(),
            );
            let c_string =
                CStr::from_bytes_with_nul(&bytes)
                    .expect("Invalid bytes for a C-string");

            TokenTree::from(Literal::c_string(c_string))
        },
    };

    token_tree.into()
}
