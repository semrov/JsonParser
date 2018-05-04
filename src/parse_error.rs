use lex::Token;
use std::{fmt,result};

/// Syntax error - An unexpected token
pub struct ParseError<'src>
{
    pub(crate) token : Token<'src>,
}

pub type Result<'src,T> = result::Result<T,ParseError<'src>>;

impl<'src> fmt::Debug for ParseError<'src> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        write!(f, "Syntax error: unexpected token {:?}", self.token)?;
        Ok(())
    }
}
