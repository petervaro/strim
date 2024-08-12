use syn::{
    Error,
    Lit::{
        self,
        ByteStr,
        Byte,
    },
    parse::{
        Parse,
        ParseStream,
    },
};
use super::bytes::{self, Bytes as _};

#[derive(Debug)]
pub enum Delimiter {
    Byte(u8),
    ByteString(Vec<u8>),
}

impl Default for Delimiter {
    fn default() -> Self {
        Self::ByteString(Vec::new())
    }
}

impl Parse for Delimiter {
    fn parse(stream: ParseStream) -> syn::Result<Self> {
        const MESSAGE: &str =
            "Expected either a byte-string or a byte literal as a delimiter";

        match Lit::parse(stream) {
            Ok(Byte(literal)) => Ok(Self::Byte(literal.value())),
            Ok(ByteStr(literal)) => Ok(Self::ByteString(literal.value())),
            Ok(any) => Err(Error::new_spanned(any, MESSAGE)),
            Err(error) => Err(Error::new(error.span(), MESSAGE)),
        }
    }
}

pub struct Lines<'a> {
    lines: bytes::Lines<'a>,
}

impl<'a> From<&'a [u8]> for Lines<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        Self { lines: bytes.lines() }
    }
}

impl<'a> Iterator for Lines<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let line = self.lines.next()?;
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                return Some(trimmed)
            }
        }
    }
}
