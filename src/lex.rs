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
/*
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
    */


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
                    let (c,rest) = Self::read_unicode_escape(rest);
                    buffer.push(c);
                    source = rest; 
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

    /// Reads a Unicode escape sequence, sequence after '\u'
    /// Reads two escape sequences, if the first is leading surrogate
    /// Invalid or incomplete sequences are replaced by 
    /// 'REPLACEMENT CHARACTER' (U+FFFD)
    fn read_unicode_escape(mut source : &'src [u8]) -> (char, &'src [u8])
    {
        let code_point = match Self::read_unit(source)
        {
            (Some(cp1 @ 0xD800..=0xDBFF),rest) =>
            {
                let (cp2, rest) = match *rest
                {
                    [b'\\', b'u', ref rest..] => Self::read_unit(rest),
                    _ => (None, rest),
                };
                source = rest;

                if let Some(cp2 @ 0xDC00..=0xDFFF) = cp2
                {
                    Some(0x1_0000 + (((cp1 - 0xD800) << 10) | (cp2 - 0xDC00)))
                }
                else 
                {
                    None
                }
            }
            (cp,rest) => {source = rest; cp}
        };

        let cp = code_point.and_then(char::from_u32).unwrap_or('\u{FFFD}');
        (cp,source)
    }

    /// Reads the body of a JSON Unicode escape sequence
    /// Returns u32 on success, None if this is not a correct 
    /// escape sequence
    fn read_unit(mut source : &'src [u8]) -> (Option<u32>,&'src[u8])
    {
        let mut unit = 0u16;
        for _ in 0..4
        {
            let digit = match *source
            {
                [b @ b'0'..=b'9', ref rest..] => 
                {
                    source = rest;
                    (b - b'0') as u16
                },
                [b @ b'A'..=b'F', ref rest..] => 
                {
                    source = rest;
                    (b - b'A' ) as u16 + 10
                },
                [b @ b'a'..=b'f', ref rest..] => 
                {
                    source = rest;
                    (b - b'a' ) as u16 + 10
                },
                _ => return (None,source),
            };
            unit = (unit << 4) + digit;
        }
        (Some(unit as u32),source)
    }

    fn read_number(mut source : &'src [u8]) -> (TokenType, &'src [u8])
    {
        let is_positive = match *source
        {
            [b'-', ref rest..] => {source = rest; false},
            _ => true,
        };

        let mut decimal_part : u64;
        match *source
        {
            [b'0', ref rest..] => 
            {
                source = rest;
                decimal_part = 0;
            },
            [b @ b'1'..=b'9', ref rest..] => 
            {
                source = rest;
                decimal_part = (b - b'0') as u64;
                while let [d @ b'0'..=b'9', ref rest..] = *source {
                    source = rest;
                    let d = (d - b'0') as u64; 
                    decimal_part = decimal_part * 10 + d;
                }
            },
            _ => return (TokenType::Error,source),
        }

        let mut exp : i32 = 0;
        if let [b'.', ref rest..] = *source
        {
            source = rest;
            let mut any_digits = false;
            while let [d @ b'0'..=b'9'] = *source {
                let d = (d - b'0') as u64;
                decimal_part = decimal_part * 10 + d;
                exp -= 1;
            }
            if !any_digits
            {
                return (TokenType::Error,source);
            }
        }

        let (has_exp,rest) = match *source
        {
            [b'e', ref rest..] | [b'E', ref rest..] => (true,rest),
            _ => (false,source),
        };

        if has_exp
        {
            source = rest;
            let is_pos_exp = match *source
            {
                [b'+', ref rest..] => {source = rest; true},
                [b'-', ref rest..] => {source = rest; false},
                _ => true,
            };

            let mut exlicit_exp = 0;
            let mut any_digits = false;
            while let [d @ b'0'..=b'9', ref rest..] = *source
            {
                source = rest;
                any_digits = true;
                let d = (d - b'0') as i32;
                exlicit_exp = 10*exlicit_exp + d;
            }
            if !any_digits
            {
                return (TokenType::Error,source);
            }
            if is_pos_exp
            {
                exp += exlicit_exp;
            }
            else 
            {
                exp -= exlicit_exp;
            }
        }

           let mut val = decimal_part as f64;
           for _ in 0..i32::abs(exp)
           {
               if exp > 0
               {
                   val *= 10.0;
               }
               else 
               {
                   val /= 10.0;
               }
           }
           if !is_positive {val = -val;}

        (TokenType::Number(val),source)
    }
}

