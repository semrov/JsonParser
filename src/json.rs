use std::collections::HashMap;

pub type JsonObject = HashMap<String, JsonValue>;
pub type JsonArray = Vec<JsonValue>;

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