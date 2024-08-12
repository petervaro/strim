use syn::{
    Error,
    Ident,
    token::{Eq, Comma},
    parse::{
        Parse,
        ParseStream,
    },
};

pub struct JoinWith<D> {
    delimiter: D,
}

impl<D> JoinWith<D> {
    pub fn unwrap_delimiter(self) -> D {
        self.delimiter
    }
}

impl<D> Parse for JoinWith<D>
where
    D: Parse + Default,
{
    fn parse(stream: ParseStream) -> syn::Result<Self> {
        if stream.is_empty() {
            let delimiter = D::default();
            return Ok(Self { delimiter })
        }

        Comma::parse(stream).map_err(|error| {
            let message =
                "Expected `,` to follow or the end of the macro invocation";
            Error::new(error.span(), message)
        })?;

        if stream.is_empty() {
            let delimiter = D::default();
            return Ok(Self { delimiter })
        }

        let identifier = Ident::parse(stream).map_err(|error| {
            let message =
                "Expected named parameter `join_with` or \
                 the end of the macro invocation";
            Error::new(error.span(), message)
        })?;

        let string = identifier.to_string();
        if string != "join_with" {
            let message = "Expected named parameter `join_with`";
            return Err(Error::new_spanned(identifier, message));
        }

        if stream.is_empty() {
            let message = "Expected `=` to follow";
            return Err(Error::new(identifier.span(), message));
        }

        Eq::parse(stream).map_err(|error| {
            let message = "Expected `join_with` to be followed by a `=`";
            Error::new(error.span(), message)
        })?;

        let delimiter = D::parse(stream)?;
        let join_with = Self { delimiter };

        if stream.is_empty() {
            return Ok(join_with);
        }

        Comma::parse(stream).map_err(|error| {
            let message =
                "Expected `,` to follow or the end of the macro invocation";
            Error::new(error.span(), message)
        })?;

        if !stream.is_empty() {
            let message = "Expected end of the macro invocation";
            let span = stream.cursor().span();
            return Err(Error::new(span, message));
        }

        Ok(join_with)
    }
}
