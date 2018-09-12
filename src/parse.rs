pub trait ParseInto<'a, T: Parse<'a>> {
    fn parse_into(&'a self) -> Result<T, ParseError<T::Error>>;
}

/// Trait implemented by types which can be parsed from an `&'a str`.
pub trait Parse<'a>: Sized {
    type Error;
    fn parse_from(input: &'a str) -> Result<Self, ParseError<Self::Error>>;
}

// TODO(eliza): add spans!
#[derive(Clone, Debug, Fail)]
pub enum ParseError<E: Fail> {
    #[fail(display = "more input required")]
    NoInput,
    // TODO(eliza): would it be better to represent parse results as a
    // Result<Option<T>,...> instead?
    #[fail(display = "not recognized")]
    Unrecognized,
    Other(E),
    // TODO(eliza): more variants: unrecognized character, too much input,
    // etc...
}

impl<'a, T, P: 'a> ParseInto<'a, T> for P
where
    P: AsRef<str>,
    T: Parse<'a>,
{
    fn parse_into(&'a self) -> Result<T, ParseError<T::Error>> {
        T::parse_from(self.as_ref())
    }
}

impl<E: Fail> From<E> for ParseError<E> {
    fn from(err: E) -> ParseError<E> {
        ParseError::Other(err)
    }
}
