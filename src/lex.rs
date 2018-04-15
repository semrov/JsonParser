use std::{char, str};

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
            //read structural characters
            [b'{', ref rest..] => (TokenType::LeftBrace, rest),
            [b'}', ref rest..] => (TokenType::RightBrace, rest),
            [b'[', ref rest..] => (TokenType::LeftBracket, rest),
            [b']', ref rest..] => (TokenType::RightBracket, rest),
            [b',', ref rest..] => (TokenType::Comma, rest),
            [b':', ref rest..] => (TokenType::Colon, rest),

            //read string
            [b'"', ref rest..] => Self::read_string(rest),
            
        };

    }

    fn read_string(mut source : &'src [u8]) -> (TokenType,&'src [u8])
    {
        let mut buffer = String::new();
        loop
        {
            match *source
            {
                //closing quote
                [b'"', ref rest..] => {source = rest; break;},

                //escape sequences
                [b'\\', b'"', ref rest..] => {source = rest; buffer.push_str("\"");},
                [b'\\', b'\\', ref rest..] => {source = rest; buffer.push_str("\\");},
                [b'\\', b'/', ref rest..] => {source = rest; buffer.push_str("/");},
                [b'\\', b'b', ref rest..] => {source = rest; buffer.push_str("\x08");},
                [b'\\', b'f', ref rest..] => {source = rest; buffer.push_str("\x0c");},
                [b'\\', b'n', ref rest..] => {source = rest; buffer.push_str("\n");},
                [b'\\', b'r', ref rest..] => {source = rest; buffer.push_str("\r");},
                [b'\\', b't', ref rest..] => {source = rest; buffer.push_str("\t");},
                [b'\\', b'u', ref rest..] => 
                {
                    //add function reading unicode escape

                    source = rest; 
                    buffer.push_str("");
                },

                //UTF8 codepoints
                [0x00..=0x7F, ref rest..] => 
                {
                    source = rest;
                    let s = unsafe {str::from_utf8_unchecked(source.get_unchecked(..1))};
                    buffer.push_str(s);
                },
                [0xC0..=0xDF,0x80..=0xBF, ref rest..] => 
                {
                    source = rest;
                    let s = unsafe {str::from_utf8_unchecked(source.get_unchecked(..2))};
                    buffer.push_str(s);
                },
                [0xE0..=0xEF, 0x80..=0xBF,0x80..=0xBF, ref rest..] =>
                {
                    source = rest;
                    let s = unsafe {str::from_utf8_unchecked(source.get_unchecked(..3))};
                    buffer.push_str(s);
                },
                [0xF0..=0xF7,0x80..=0xBF,0x80..=0xBF,0x80..=0xBF, ref rest..] => 
                {
                    source = rest;
                    let s = unsafe {str::from_utf8_unchecked(source.get_unchecked(..4))};
                    buffer.push_str(s);
                },

                //If string is valid utf8 string, then this should not happen
                [_,_..] => unreachable!(),

                //unterminated string
                 [ref rest..] => return (TokenType::Error,rest),
            }
        }
        (TokenType::String(buffer),source)
    }
}

