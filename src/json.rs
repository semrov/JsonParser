use std::collections::HashMap;
use std::error::Error;
use std::fmt;

pub type JsonObject = HashMap<String, JsonValue>;
pub type JsonArray = Vec<JsonValue>;

pub struct JsonObjectContainer(JsonObject);
pub struct JsonObjectContainerRef<'a>(&'a JsonObject);
pub struct JsonObjectContainerMutRef<'a>(&'a mut JsonObject);
pub struct JsonArrayContainer(JsonArray);
pub struct JsonArrayContainerRef<'a>(&'a JsonArray);
pub struct JsonArrayContainerMutRef<'a>(&'a mut JsonArray);

#[derive(Debug)]
pub struct InvalidValueError<'a>{pub(crate) value : &'a JsonValue}

impl<'a> fmt::Display for InvalidValueError<'a>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid json value type fetched: {}", self.value.desc())
    }
}

impl<'a>  Error for InvalidValueError<'a>
{
    fn description(&self) -> &str {
        "Invalid json value type"
    }
}

#[derive(PartialEq,Debug)]
pub enum JsonValue
{
    JsonObject(JsonObject),
    JsonArray(JsonArray),
    JsonNumber(f64),
    JsonString(String),
    JsonBool(bool),
    Null,
}

impl JsonValue {
    pub fn is_object(&self) -> bool
    {
        match *self
        {
            JsonValue::JsonObject(_) => true,
            _ => false,
        }
    }
    pub fn is_array(&self) -> bool
    {
        match *self
        {
            JsonValue::JsonArray(_) => true,
            _ => false,
        }
    }
    pub fn is_number(&self) -> bool
    {
        match *self
        {
            JsonValue::JsonNumber(_) => true,
            _ => false,
        }
    }
    pub fn is_string(&self) -> bool
    {
        match *self
        {
            JsonValue::JsonString(_) => true,
            _ => false,
        }
    }
    pub fn is_bool(&self) -> bool
    {
        match *self
        {
            JsonValue::JsonBool(_) => true,
            _ => false,
        }
    }
    pub fn is_null(&self) -> bool
    {
        match *self
        {
            JsonValue::Null => true,
            _ => false,
        }
    }

    fn desc(&self) -> &'static str
    {
        match *self
        {
            JsonValue::JsonObject(_) => "JsonObject",
            JsonValue::JsonArray(_) => "JsonArray",
            JsonValue::JsonNumber(_) => "JsonNumber",
            JsonValue::JsonString(_) => "JsonString",
            JsonValue::JsonBool(_) => "JsonBool",
            JsonValue::Null => "Null",
        }
    }

    pub fn get_object_ref<'a>(&'a self) -> Result<JsonObjectContainerRef<'a>, InvalidValueError<'a>>
    {
        match self
        {
            JsonValue::JsonObject(ref object) => Ok(JsonObjectContainerRef(object)),
            value => Err(InvalidValueError{value}),
        }
    }

    pub fn get_object_mut<'a>(&'a mut self) -> Result<JsonObjectContainerMutRef<'a>, InvalidValueError<'a>>
    {
        match self
        {
            JsonValue::JsonObject(ref mut object) => Ok(JsonObjectContainerMutRef(object)),
            value => Err(InvalidValueError{value}),
        }
    }

    pub fn get_array_ref<'a>(&'a self) -> Result<JsonArrayContainerRef<'a>, InvalidValueError<'a>>
    {
        match self
        {
            JsonValue::JsonArray(ref array) => Ok(JsonArrayContainerRef(array)),
            value => Err(InvalidValueError{value}),
        }
    }

    pub fn get_array_mut<'a>(&'a mut self) -> Result<JsonArrayContainerMutRef<'a>, InvalidValueError<'a>>
    {
        match self
        {
            JsonValue::JsonArray(ref mut array) => Ok(JsonArrayContainerMutRef(array)),
            value => Err(InvalidValueError{value}),
        }
    }

    pub fn get_string_ref<'a>(&'a self) -> Result<&'a str, InvalidValueError<'a>>
    {
        match self 
        {
            JsonValue::JsonString(ref string) => Ok(string),
            value => Err(InvalidValueError{value}),
        }
    }

    pub fn get_string_mut<'a>(&'a mut self) -> Result<&'a mut String, InvalidValueError<'a>>
    {
        match self 
        {
            JsonValue::JsonString(ref mut string) => Ok(string),
            value => Err(InvalidValueError{value}),
        }
    }

    pub fn get_number<'a>(&'a self) -> Result<f64, InvalidValueError<'a>>
    {
        match self
        {
            JsonValue::JsonNumber(number) => Ok(*number),
            value => Err(InvalidValueError{value}),
        }
    }

    pub fn get_number_mut<'a>(&'a mut self) -> Result<&'a mut f64, InvalidValueError<'a>>
    {
        match self
        {
            JsonValue::JsonNumber(ref mut number) => Ok(number),
            value => Err(InvalidValueError{value}),
        }
    }

    pub fn get_bool<'a>(&'a self) -> Result<bool, InvalidValueError<'a>>
    {
        match self 
        {
            JsonValue::JsonBool(b) => Ok(*b),
            value => Err(InvalidValueError{value}),
        }
    }

    pub fn get_bool_mut<'a>(&'a mut self) -> Result<&'a mut bool, InvalidValueError<'a>>
    {
        match self 
        {
            JsonValue::JsonBool(ref mut b) => Ok(b),
            value => Err(InvalidValueError{value}),
        }
    }
}

impl<'a> JsonObjectContainerRef<'a>
{
    pub fn get_object_ref(&'a self, key : &str) -> Result<Option<JsonObjectContainerRef<'a>>, InvalidValueError<'a>>
    {
        match self.0.get(key)
        {
            Some(&JsonValue::JsonObject(ref object)) => Ok(Some(JsonObjectContainerRef(object))),
            Some(value) => Err(InvalidValueError{value}),
            None => Ok(None),
        }
    }

    pub fn get_array_ref(&'a self, key : &str) -> Result<Option<JsonArrayContainerRef<'a>>, InvalidValueError<'a>>
    {
        match self.0.get(key)
        {
            Some(&JsonValue::JsonArray(ref array)) => Ok(Some(JsonArrayContainerRef(array))),
            Some(value) => Err(InvalidValueError{value}),
            None => Ok(None),
        }
    }

    pub fn get_number(&'a self, key : &str) -> Result<Option<f64>, InvalidValueError<'a>>
    {
        match self.0.get(key)
        {
            Some(&JsonValue::JsonNumber(number)) => Ok(Some(number)),
            Some(value) => Err(InvalidValueError{value}),
            None => Ok(None),
        }
    }

    pub fn get_bool(&'a self, key : &str) -> Result<Option<bool>, InvalidValueError<'a>>
    {
        match self.0.get(key)
        {
            Some(&JsonValue::JsonBool(b)) => Ok(Some(b)),
            Some(value) => Err(InvalidValueError{value}),
            None => Ok(None),
        }
    }

    pub fn get_string_ref(&'a self, key : &str) -> Result<Option<&'a str>, InvalidValueError<'a>>
    {
        match self.0.get(key)
        {
            Some(&JsonValue::JsonString(ref string)) => Ok(Some(string)),
            Some(value) => Err(InvalidValueError{value}),
            None => Ok(None),
        }
    }

    pub fn is_null(&self, key : &str) -> bool
    {
        match self.0.get(key) 
        {
            Some(&JsonValue::Null) => true,
            Some(_) => false,
            None => true,
        }
    }

    pub fn lenght(&self) -> usize 
    {
        self.0.len()
    }

    pub fn contains_key(&self, key : &str) -> bool
    {
        self.0.contains_key(key)
    }

}

impl<'a> JsonObjectContainerMutRef<'a>
{
    pub fn get_object_ref(&'a mut self, key : &str) -> Result<Option<JsonObjectContainerRef<'a>>, InvalidValueError<'a>>
    {
        match self.0.get(key)
        {
            Some(JsonValue::JsonObject(ref object)) => Ok(Some(JsonObjectContainerRef(object))),
            Some(value) => Err(InvalidValueError{value}),
            None => Ok(None),
        }
    }

    pub fn get_object_mut(&'a mut self, key : &str) -> Result<Option<JsonObjectContainerMutRef<'a>>, InvalidValueError<'a>>
    {
        match self.0.get_mut(key)
        {
            Some(JsonValue::JsonObject(ref mut object)) => Ok(Some(JsonObjectContainerMutRef(object))),
            Some(value) => Err(InvalidValueError{value}),
            None => Ok(None),
        }
    }

    pub fn get_array_ref(&'a self, key : &str) -> Result<Option<JsonArrayContainerRef<'a>>, InvalidValueError<'a>>
    {
        match self.0.get(key)
        {
            Some(&JsonValue::JsonArray(ref array)) => Ok(Some(JsonArrayContainerRef(array))),
            Some(value) => Err(InvalidValueError{value}),
            None => Ok(None),
        }
    }

    pub fn get_array_mut(&'a mut self, key : &str) -> Result<Option<JsonArrayContainerMutRef<'a>>, InvalidValueError<'a>>
    {
        match self.0.get_mut(key)
        {
            Some(JsonValue::JsonArray(ref mut array)) => Ok(Some(JsonArrayContainerMutRef(array))),
            Some(value) => Err(InvalidValueError{value}),
            None => Ok(None),
        }
    }

    pub fn get_number(&'a self, key : &str) -> Result<Option<f64>, InvalidValueError<'a>>
    {
        match self.0.get(key)
        {
            Some(&JsonValue::JsonNumber(number)) => Ok(Some(number)),
            Some(value) => Err(InvalidValueError{value}),
            None => Ok(None),
        }
    }

    pub fn get_number_mut(&'a mut self, key : &str) -> Result<Option<&'a mut f64>, InvalidValueError<'a>>
    {
        match self.0.get_mut(key)
        {
            Some(JsonValue::JsonNumber(ref mut number)) => Ok(Some(number)),
            Some(value) => Err(InvalidValueError{value}),
            None => Ok(None),
        }
    }

    pub fn get_bool(&'a self, key : &str) -> Result<Option<bool>, InvalidValueError<'a>>
    {
        match self.0.get(key)
        {
            Some(&JsonValue::JsonBool(b)) => Ok(Some(b)),
            Some(value) => Err(InvalidValueError{value}),
            None => Ok(None),
        }
    }

    pub fn get_bool_mut(&'a mut self, key : &str) -> Result<Option<&'a mut bool>, InvalidValueError<'a>>
    {
        match self.0.get_mut(key)
        {
            Some(JsonValue::JsonBool(b)) => Ok(Some(b)),
            Some(value) => Err(InvalidValueError{value}),
            None => Ok(None),
        }
    }

    pub fn get_string_ref(&'a self, key : &str) -> Result<Option<&'a str>, InvalidValueError<'a>>
    {
        match self.0.get(key)
        {
            Some(&JsonValue::JsonString(ref string)) => Ok(Some(string)),
            Some(value) => Err(InvalidValueError{value}),
            None => Ok(None),
        }
    }

    pub fn get_string_mut(&'a mut self, key : &str) -> Result<Option<&'a mut str>, InvalidValueError<'a>>
    {
        match self.0.get_mut(key)
        {
            Some(JsonValue::JsonString(ref mut string)) => Ok(Some(string)),
            Some(value) => Err(InvalidValueError{value}),
            None => Ok(None),
        }
    }

    pub fn is_null(&self, key : &str) -> bool
    {
        match self.0.get(key) 
        {
            Some(&JsonValue::Null) => true,
            Some(_) => false,
            None => true,
        }
    }

    pub fn lenght(&self) -> usize 
    {
        self.0.len()
    }

    pub fn contains_key(&self, key : &str) -> bool
    {
        self.0.contains_key(key)
    }

}

impl<'a> JsonArrayContainerRef<'a> {
    pub fn get_object_ref(&'a self, index : usize) -> Result<Option<JsonObjectContainerRef<'a>>, InvalidValueError<'a>>
    {
        match self.0.get(index)
        {
            Some(JsonValue::JsonObject(ref object)) => Ok(Some(JsonObjectContainerRef(object))),
            Some(value) => Err(InvalidValueError{value}),
            None => Ok(None),
        }
    }

    pub fn get_array_ref(&'a self, index : usize) -> Result<Option<JsonArrayContainerRef<'a>>, InvalidValueError<'a>>
    {
        match self.0.get(index) 
        {
            Some(JsonValue::JsonArray(ref array)) => Ok(Some(JsonArrayContainerRef(array))),
            Some(value) => Err(InvalidValueError{value}),
            None => Ok(None),
        }
    }

    pub fn get_string_ref(&'a self, index : usize) -> Result<Option<&'a str>, InvalidValueError<'a>>
    {
        match self.0.get(index) 
        {
            Some(JsonValue::JsonString(ref string)) => Ok(Some(string)),
            Some(value) => Err(InvalidValueError{value}),
            None => Ok(None),
        }
    }

    pub fn get_number(&'a self, index : usize) -> Result<Option<f64>, InvalidValueError<'a>>
    {
        match self.0.get(index) 
        {
            Some(JsonValue::JsonNumber(number)) => Ok(Some(*number)),
            Some(value) => Err(InvalidValueError{value}),
            None => Ok(None),
        }
    }

    pub fn get_bool(&'a self, index : usize) -> Result<Option<bool>, InvalidValueError<'a>>
    {
        match self.0.get(index) 
        {
            Some(JsonValue::JsonBool(b)) => Ok(Some(*b)),
            Some(value) => Err(InvalidValueError{value}),
            None => Ok(None),
        }
    }

    pub fn is_null(&self, index : usize) -> bool
    {
        match self.0.get(index) 
        {
            Some(JsonValue::Null) => true,
            Some(_) => false,
            None => true,
        }
    }

    pub fn lenght(&self) -> usize 
    {
        self.0.len()
    }
    
}

impl<'a> JsonArrayContainerMutRef<'a> {
    pub fn get_object_ref(&'a self, index : usize) -> Result<Option<JsonObjectContainerRef<'a>>, InvalidValueError<'a>>
    {
        match self.0.get(index)
        {
            Some(JsonValue::JsonObject(ref object)) => Ok(Some(JsonObjectContainerRef(object))),
            Some(value) => Err(InvalidValueError{value}),
            None => Ok(None),
        }
    }

    pub fn get_object_mut(&'a mut self, index : usize) -> Result<Option<JsonObjectContainerMutRef<'a>>, InvalidValueError<'a>>
    {
        match self.0.get_mut(index)
        {
            Some(JsonValue::JsonObject(ref mut object)) => Ok(Some(JsonObjectContainerMutRef(object))),
            Some(value) => Err(InvalidValueError{value}),
            None => Ok(None),
        }
    }

    pub fn get_array_ref(&'a self, index : usize) -> Result<Option<JsonArrayContainerRef<'a>>, InvalidValueError<'a>>
    {
        match self.0.get(index) 
        {
            Some(JsonValue::JsonArray(ref array)) => Ok(Some(JsonArrayContainerRef(array))),
            Some(value) => Err(InvalidValueError{value}),
            None => Ok(None),
        }
    }

    pub fn get_array_mut(&'a mut self, index : usize) -> Result<Option<JsonArrayContainerMutRef<'a>>, InvalidValueError<'a>>
    {
        match self.0.get_mut(index) 
        {
            Some(JsonValue::JsonArray(ref mut array)) => Ok(Some(JsonArrayContainerMutRef(array))),
            Some(value) => Err(InvalidValueError{value}),
            None => Ok(None),
        }
    }

    pub fn get_string_ref(&'a self, index : usize) -> Result<Option<&'a str>, InvalidValueError<'a>>
    {
        match self.0.get(index) 
        {
            Some(JsonValue::JsonString(ref string)) => Ok(Some(string)),
            Some(value) => Err(InvalidValueError{value}),
            None => Ok(None),
        }
    }

    pub fn get_string_mut(&'a mut self, index : usize) -> Result<Option<&'a mut String>, InvalidValueError<'a>>
    {
        match self.0.get_mut(index) 
        {
            Some(JsonValue::JsonString(ref mut string)) => Ok(Some(string)),
            Some(value) => Err(InvalidValueError{value}),
            None => Ok(None),
        }
    }

    pub fn get_number(&'a self, index : usize) -> Result<Option<f64>, InvalidValueError<'a>>
    {
        match self.0.get(index) 
        {
            Some(JsonValue::JsonNumber(number)) => Ok(Some(*number)),
            Some(value) => Err(InvalidValueError{value}),
            None => Ok(None),
        }
    }

    pub fn get_number_mut(&'a mut self, index : usize) -> Result<Option<&'a mut f64>, InvalidValueError<'a>>
    {
        match self.0.get_mut(index) 
        {
            Some(JsonValue::JsonNumber(ref mut number)) => Ok(Some(number)),
            Some(value) => Err(InvalidValueError{value}),
            None => Ok(None),
        }
    }

    pub fn get_bool(&'a self, index : usize) -> Result<Option<bool>, InvalidValueError<'a>>
    {
        match self.0.get(index) 
        {
            Some(JsonValue::JsonBool(b)) => Ok(Some(*b)),
            Some(value) => Err(InvalidValueError{value}),
            None => Ok(None),
        }
    }

    pub fn get_bool_mut(&'a mut self, index : usize) -> Result<Option<&'a mut bool>, InvalidValueError<'a>>
    {
        match self.0.get_mut(index) 
        {
            Some(JsonValue::JsonBool(ref mut b)) => Ok(Some(b)),
            Some(value) => Err(InvalidValueError{value}),
            None => Ok(None),
        }
    }

    pub fn is_null(&self, index : usize) -> bool
    {
        match self.0.get(index) 
        {
            Some(JsonValue::Null) => true,
            Some(_) => false,
            None => true,
        }
    }

    pub fn lenght(&self) -> usize 
    {
        self.0.len()
    }
}

impl JsonObjectContainer
{
     // to be implemented
}

impl JsonArrayContainer
{
    // to be implemented
}
