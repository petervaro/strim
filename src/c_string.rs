use std::ffi::CString;
use syn::{
    Error,
    Lit::{
        self,
        Byte,
        CStr,
    },
    parse::{
        Parse,
        ParseStream,
    },
};
use super::bytes::{self, Bytes as _};

pub enum Delimiter {
    // NOTE: It feels really rather wrong to look for a byte-literal, instead of
    //       whatever `c_char` would resolve to on the given platform (i.e
    //        either `i8` or `u8`).  But since Rust does not have a C-char
    //       literal, nor the `CStr` implementation cares too much about casting
    //       between signed and unsigned integers here (which absolutely baffles
    //       me), for the time being I believe this is the best we could
    //       > Safety: Transmuting a slice of `c_char`s to a slice of `u8`s is
    //       > safe on all supported targets
    Byte(u8),
    CString(CString),
}

impl Default for Delimiter {
    fn default() -> Self {
        Self::CString(c"".to_owned())
    }
}

impl Parse for Delimiter {
    fn parse(stream: ParseStream) -> syn::Result<Self> {
        const MESSAGE: &str =
            "Expected either a C-string or a byte literal as a delimiter";

        match Lit::parse(stream) {
            Ok(Byte(literal)) if literal.value() == b'\0' => {
                let message =
                    "Delimiter for C-string cannot be the nul-terminator";
                Err(Error::new_spanned(literal, message))
            },
            Ok(Byte(literal)) => Ok(Self::Byte(literal.value())),
            Ok(CStr(literal)) => Ok(Self::CString(literal.value())),
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
