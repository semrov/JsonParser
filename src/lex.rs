
/// A JSON lexical analyzer of UTF-8 string
/// 
/// Produces JSON tokens according to RFC 7159
/// Returns an error token if it encounters invalid tokens
/// with invalid bytes in a span
pub(crate) struct Lex<'src> {
    source: &'src [u8],
}

#[derive(Debug,PartialEq)]
pub(crate) struct Token<'src> 
{
    pub(crate) span : &'src [u8],
    pub(crate) token_type : TokenType
}

#[derive(Debug,PartialEq)]
pub(crate) enum TokenType
{
    // structural characters
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,

    //value types
    String(String),
    Number(f64),
    Bool(bool),
    Null,

    //other types
    Error,
    End,
}

impl<'src> Lex<'src> {
    ///create a new lexer from JSON string
    pub(crate) fn new(source : &'src str) -> Lex
    {
        Lex{source : source.as_bytes()}
    }

    ///Get next token from lexer
    pub(crate) fn next(&mut self) -> Token<'src>
    {
        // skip any number of whitespace characters
        loop {
            match *self.source {
                [b, ref rest..] if [b' ', b'\t', b'\r', b'\n'].contains(&b) => self.source = rest,
                _ => break,
            }
        }

        // Determine the type of the token by its first byte
        let (token_type, rest) = match *self.source
        {
            [b'{', ref rest..] => (TokenType::LeftBrace, rest),
            [b'}', ref rest..] => (TokenType::RightBrace, rest),
            [b'[', ref rest..] => (TokenType::LeftBracket, rest),
            [b']', ref rest..] => (TokenType::RightBracket, rest),
            [b',', ref rest..] => (TokenType::Comma, rest),
            [b':', ref rest..] => (TokenType::Colon, rest),
            
        }

    }
}

