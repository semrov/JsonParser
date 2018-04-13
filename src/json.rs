use std::collections::HashMap;

pub type JsonObject = HashMap<String, JsonValue>;
pub type JsonArray = Vec<JsonValue>;

pub enum JsonValue
{
    JsonObject(JsonObject),
    JsonArray(JsonArray),
    Number(f64),
    String(String),
    Bool(bool),
    Null,
}