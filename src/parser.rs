use lex::{Lex,Token,TokenType};
use json;
use parse_error::{ParseError,Result};

///A JSON parser
///
/// VALUE: String | Number | Bool | Null | OBJECT | ARRAY
/// 
/// OBJECT = '{' '}' | '{' MEMBERS '}'
/// MEMBERS = member | MEMBERS , member
/// member = String ':' VALUE 
/// 
/// ARRAY = '[' ']' | '[' ELEMENTS ']'
/// ELEMENTS = VALUE | ELEMENTS ',' VALUE

pub struct JsonParser<'src>
{
    lexer : Lex<'src>,
}

struct Value(json::JsonValue);
struct Object(json::JsonObject);
struct Array(json::JsonArray);
struct Members(json::JsonObject);
struct Member((String, json::JsonValue));
struct Elements(json::JsonArray);

enum Either<A,B>
{
    First(A),
    Second(B),
}

impl<'src> JsonParser<'src> {
    pub fn new(source : &'src str) -> JsonParser
    {
        JsonParser { lexer : Lex::new(source) }
    }

    // * is current state of the parser
    //VALUE = * String
    //VALUE = * Number
    //VALUE = * Bool
    //VALUE = * Null
    //VALUE = * OBJECT
    //VALUE = * ARRAY
    //OBJECT = '{' '}'
    //OBJECT = '{' MEMBERS '}'
    //ARRAY = * '[' ']'
    //ARRAY = * '[' ELEMENTS ']'

    /// Parse a JSON Value
    pub fn parse(&mut self) -> Result<'src, json::JsonValue>
    {
        let value = match self.lexer.next()
        {
            Token{token_type: TokenType::String(string), .. } => self.reduce_value_string(string)?,
            Token{token_type: TokenType::Number(number),.. } => self.reduce_value_number(number)?,
            Token{token_type: TokenType::Bool(b),.. } => self.reduce_value_bool(b)?,
            Token{token_type: TokenType::Null,.. } => self.reduce_value_null()?,
            Token{token_type: TokenType::LeftBrace,.. } =>
            {
                let o = self.parse_object()?;
                self.reduce_value_object(o)?
            },
            Token{token_type: TokenType::LeftBracket,.. } =>
            { 
                let array = self.parse_array()?;
                self.reduce_value_array(array)?
            },
            token => return Err(ParseError{token}),
        };

        match self.lexer.next()
        {
            Token{token_type:  TokenType::End, ..} => 
            {
                let Value(value) = value;
                Ok(value)
            },
            token => Err(ParseError{token}),
        }
    }

    /// VALUE = String
    fn reduce_value_string(&mut self, string : String) -> Result<'src, Value>
    {
        Ok(Value(json::JsonValue::JsonString(string)))
    }

    /// VALUE = Number
    fn reduce_value_number(&mut self, number : f64) -> Result<'src, Value>
    {
        Ok(Value(json::JsonValue::JsonNumber(number)))
    }

    /// VALUE = Bool
    fn reduce_value_bool(&mut self, b : bool) -> Result<'src, Value>
    {
        Ok(Value(json::JsonValue::JsonBool(b)))
    }

    /// VALUE = Null
    fn reduce_value_null(&mut self) -> Result<'src, Value>
    {
        Ok(Value(json::JsonValue::Null))
    }

    /// VALUE = OBJECT
    fn reduce_value_object(&mut self, o : Object) -> Result<'src, Value>
    {
        let Object(object) = o;
        Ok(Value(json::JsonValue::JsonObject(object)))
    }

    /// VALUE = ARRAY
    fn reduce_value_array(&mut self, array : Array) -> Result<'src, Value>
    {
        let Array(array) = array;
        Ok(Value(json::JsonValue::JsonArray(array)))
    }

    //OBJECT = '{' * MEMBERS '}'
    //OBJECT = '{' * '}'
    //MEMBERS = * member
    //MEMBERS = MEMBERS * ',' member
    //member = String ':' VALUE 
    fn parse_object(&mut self) -> Result<'src, Object>
    {
        let mut members = match self.lexer.next()
        {
            Token{token_type: TokenType::String(key), ..} => 
            {
                let member = self.member_string(key)?;
                self.members_member(member)?
            },
            Token{token_type: TokenType::RightBrace, ..} => 
            {
                let object = json::JsonObject::new();
                return Ok(Object(object));
            },
            token => return Err(ParseError{token}),
        };

        loop 
        {
            match self.parse_object_members(members)?
            {
                Either::First(m) => members = m,
                Either::Second(object) => return Ok(object),
            }
        }
    }

    //member = String * ':' VALUE
    fn member_string(&mut self, key : String) -> Result<'src,Member>
    {
        match self.lexer.next()
        {
            Token{token_type: TokenType::Colon, ..} => Ok(self.member_string_colon(key)?),
            token => Err(ParseError{token}),
        }
    }

    //member = String ':' * VALUE
    //VALUE = * String
    //VALUE = * Number
    //VALUE = * Bool
    //VALUE = * Null
    //VALUE = * OBJECT
    //VALUE = * ARRAY
    //OBJECT = '{' '}'
    //OBJECT = '{' MEMBERS '}'
    //ARRAY = * '[' ']'
    //ARRAY = * '[' ELEMENTS ']'
    fn member_string_colon(&mut self, key : String) -> Result<'src,Member>
    {
        let value = match self.lexer.next()
        {
            Token{token_type: TokenType::String(string),.. } =>
            {
                self.reduce_value_string(string)?
            },
            Token{token_type: TokenType::Number(number),.. } =>
            {
                self.reduce_value_number(number)?
            },
            Token{token_type: TokenType::Bool(b),.. } =>
            {
                self.reduce_value_bool(b)?
            },
            Token{token_type: TokenType::Null,.. } =>
            {
                self.reduce_value_null()?
            },
            Token{token_type: TokenType::LeftBrace,.. } =>
            {
                let object = self.parse_object()?;
                self.reduce_value_object(object)?
            },
            Token{token_type: TokenType::LeftBracket,.. } =>
            {
                let array = self.parse_array()?;
                self.reduce_value_array(array)?
            },
            token => return Err(ParseError{token}),
        };
        self.member_string_colon_value(key,value) 
    }

    //member = String ':' VALUE * 
    fn member_string_colon_value(&mut self, key : String, value : Value) -> Result<'src, Member>
    {
        let Value(value) = value;
        Ok(Member((key,value)))
    }

    // MEMBERS = member *
    fn members_member(&mut self, member : Member) -> Result<'src, Members>
    {
        let Member((key,value)) = member;
        let mut object = json::JsonObject::new();
        object.insert(key,value);
        Ok(Members(object))
    }

    // OBJECT = '{' MEMBERS * '}'
    // MEMBERS =  MEMBERS * ',' member
    fn parse_object_members(&mut self, members : Members) -> Result<'src, Either<Members, Object>>
    {
        match self.lexer.next()
        {
            Token{token_type: TokenType::Comma, ..} =>
            {
                let members = self.members_members_comma(members)?;
                Ok(Either::First(members))
            },
            Token{token_type: TokenType::RightBrace, ..} => 
            {
                let Members(object) = members;
                Ok(Either::Second(Object(object)))
            },
            token => return Err(ParseError{token}),
        }
    }

    // MEMBERS =  MEMBERS ',' * member
    // member = * String : Value
    fn members_members_comma(&mut self, members : Members) -> Result<'src, Members>
    {
        let member = match self.lexer.next()
        {
            Token{token_type: TokenType::String(key), ..} => self.member_string(key)?,
            token => return Err(ParseError{token}),
        };
        Ok(self.members_members_comma_member(members, member)?)
    }

    // MEMBERS =  MEMBERS ',' member * -> MEMBERS
    fn members_members_comma_member(&mut self, members : Members, member : Member) -> Result<'src, Members>
    {
        let Members(mut object) = members;
        let Member((key,value)) = member;
        object.insert(key,value);
        Ok(Members(object))
    }

    //ARRAY = * '[' ']'
    //ARRAY = * '[' ELEMENTS ']'
    fn parse_array(&mut self) -> Result<'src,Array>
    {
        let value = match self.lexer.next()
        {
            Token{token_type: TokenType::String(string),..} =>
            {
                self.reduce_value_string(string)?
            }
            Token{token_type: TokenType::Number(number),..} => 
            {
                self.reduce_value_number(number)?
            },
            Token{token_type: TokenType::Bool(b),..} => 
            {
                self.reduce_value_bool(b)?
            },
            Token{token_type: TokenType::Null,..} =>
            {
                self.reduce_value_null()?
            },
            Token{token_type: TokenType::RightBracket, ..} => 
            {
                let array = json::JsonArray::new();
                return Ok(Array(array));
            },
            Token{token_type: TokenType::LeftBracket, ..} =>
            {
                let array = self.parse_array()?;
                self.reduce_value_array(array)?
            },
            Token{token_type: TokenType::LeftBrace, ..} => 
            {
                let object = self.parse_object()?;
                self.reduce_value_object(object)?
            },
            token => return Err(ParseError{token}),
        };

        let mut elements = self.elements_value(value)?;
        loop
        {
            match self.parse_array_elements(elements)?
            {
                Either::First(e) => elements = e,
                Either::Second(array) => return Ok(array),
            }
        }
    }

    //MEMBERS = VALUE
    fn elements_value(&mut self, value : Value) -> Result<'src, Elements>
    {
        let Value(member) = value;
        let mut array = json::JsonArray::new();
        array.push(member);
        Ok(Elements(array))
    }

    //array = '[' elements *']'
    //elements = elements * , value
    fn parse_array_elements(&mut self, elements : Elements) ->Result<'src,Either<Elements,Array>>
    {
        match self.lexer.next()
        {
            Token{token_type: TokenType::Comma, ..} =>
            {
                let elements = self.elements_elements_comma(elements)?;
                Ok(Either::First(elements))
            },
            Token{token_type: TokenType::RightBracket,..} => 
            {
                let Elements(array) = elements;
                Ok(Either::Second(Array(array)))
            },
            token => Err(ParseError{token})
        }
    }

    // ELEMENTS = ELEMENTS , * VALUE
    // VALUE = * STRING
    // VALUE = * NUMBER
    // VALUE = * BOOL
    // VALUE = * NULL
    // VALUE = * OBJECT
    // VALUE = * ARRAY
    // OBJECT = * '{' '}' 
    // OBJECT = * '{' MEMBERS '}' 
    // ARRAY = * '[' ']'
    // ARRAY = * '[' ELEMENTS ']'
    fn elements_elements_comma(&mut self, elements : Elements) -> Result<'src,Elements>
    {
        let value = match self.lexer.next()
        {
            Token{token_type: TokenType::String(string),..} => 
            {
                self.reduce_value_string(string)?
            },
            Token{token_type: TokenType::Number(number) ,..} =>
            {
                self.reduce_value_number(number)?
            },
            Token{token_type: TokenType::Bool(b) ,..} =>
            {
                self.reduce_value_bool(b)?
            },
            Token{token_type: TokenType::Null ,..} =>
            {
                self.reduce_value_null()?
            },
            Token{token_type: TokenType::LeftBrace ,..} =>
            {
                let object = self.parse_object()?;
                self.reduce_value_object(object)?
            },
            Token{token_type: TokenType::LeftBracket ,..} =>
            {
               let array = self.parse_array()?;
               self.reduce_value_array(array)?
            },
            token => return Err(ParseError{token}),
        };

        Ok(self.elements_elements_comma_value(elements,value)?)

    }

    // ELEMENTS = ELEMENTS , VALUE *
    fn elements_elements_comma_value(&mut self, elements : Elements, value : Value) -> Result<'src, Elements>
    {
        let Elements(mut array) = elements;
        let Value(value) = value;
        array.push(value);
        Ok(Elements(array))
    }


}