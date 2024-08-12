use std::str;
use syn::{
    Error,
    Lit::{
        self,
        Str,
        Char,
    },
    parse::{
        Parse,
        ParseStream,
    },
};

#[derive(Debug)]
pub enum Delimiter {
    Character(char),
    String(String),
}

impl Default for Delimiter {
    fn default() -> Self {
        Self::String(String::new())
    }
}

impl Parse for Delimiter {
    fn parse(stream: ParseStream) -> syn::Result<Self> {
        const MESSAGE: &str =
            "Expected either a string or a character literal as a delimiter";

        match Lit::parse(stream) {
            Ok(Char(literal)) => Ok(Self::Character(literal.value())),
            Ok(Str(literal)) => Ok(Self::String(literal.value())),
            Ok(any) => Err(Error::new_spanned(any, MESSAGE)),
            Err(error) => Err(Error::new(error.span(), MESSAGE)),
        }
    }
}

pub struct Lines<'a> {
    lines: str::Lines<'a>,
}

impl<'a> From<&'a str> for Lines<'a> {
    fn from(string: &'a str) -> Self {
        Self { lines: string.lines() }
    }
}

impl<'a> Iterator for Lines<'a> {
    type Item = &'a str;

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
