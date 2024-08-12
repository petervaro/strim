use syn::{
    Error,
    Lit::{
        self,
        Str,
        ByteStr,
    },
    parse::{
        Parse,
        ParseStream,
    },
};
use super::{
    join_with::JoinWith,
    string,
    byte_string,
};


#[derive(Debug)]
pub enum Arguments {
    String {
        input: String,
        delimiter: string::Delimiter,
    },
    ByteString {
        input: Vec<u8>,
        delimiter: byte_string::Delimiter,
    },
}

impl Parse for Arguments {
    fn parse(stream: ParseStream) -> syn::Result<Self> {
        const MESSAGE: &str =
            "Expected either a string or a byte-string literal";

        match stream.parse::<Lit>() {
            Ok(Str(literal)) => {
                let join_with =
                    JoinWith::<string::Delimiter>::parse(stream)?;
                let delimiter = join_with.unwrap_delimiter();
                let input = literal.value();

                Ok(Self::String { input, delimiter })
            },
            Ok(ByteStr(literal)) => {
                let join_with =
                    JoinWith::<byte_string::Delimiter>::parse(stream)?;
                let delimiter = join_with.unwrap_delimiter();
                let input = literal.value();

                Ok(Self::ByteString { input, delimiter })
            },
            Ok(any) => Err(Error::new_spanned(any, MESSAGE)),
            Err(error) => Err(Error::new(error.span(), MESSAGE)),
        }
    }
}
