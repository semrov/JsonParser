use parser::JsonParser;
use json::*;
use parse_error::*;

#[test]
fn test_syntax_simple() {
    let source = r#"-3.12e-10 "#;
    let mut parser = JsonParser::new(source);
    let json = parser.parse().unwrap();
    assert_eq!(json,JsonValue::JsonNumber(-3.12e-10));

    let source = r#" "test string" "#;
    let mut parser = JsonParser::new(source);
    let json = parser.parse().unwrap();
    assert_eq!(json,JsonValue::JsonString("test string".to_owned()));

    let source = r#" true "#;
    let mut parser = JsonParser::new(source);
    let json = parser.parse().unwrap();
    assert_eq!(json,JsonValue::JsonBool(true));    

    let source = r#" false "#;
    let mut parser = JsonParser::new(source);
    let json = parser.parse().unwrap();
    assert_eq!(json,JsonValue::JsonBool(false));  

    let source = r#" null "#;
    let mut parser = JsonParser::new(source);
    let json = parser.parse().unwrap();
    assert_eq!(json,JsonValue::Null); 

    let source = r#" [] "#;
    let mut parser = JsonParser::new(source);
    let json = parser.parse().unwrap();
    assert_eq!(json,JsonValue::JsonArray(JsonArray::new()));

    let source = r#" [5] "#;
    let mut parser = JsonParser::new(source);
    let json = parser.parse().unwrap();
    assert_eq!(json,JsonValue::JsonArray(vec![JsonValue::JsonNumber(5.0)])); 

    let source = r#" [5,2.1,[],10] "#;
    let mut parser = JsonParser::new(source);
    let json = parser.parse().unwrap();
    assert_eq!(json,JsonValue::JsonArray(vec![
        JsonValue::JsonNumber(5.0),
        JsonValue::JsonNumber(2.1),
        JsonValue::JsonArray(JsonArray::new()),
        JsonValue::JsonNumber(10.0)]));      


    let source = r#" {} "#;
    let mut parser = JsonParser::new(source);
    let json = parser.parse().unwrap();
    assert_eq!(json,JsonValue::JsonObject(JsonObject::new()));

    let source = r#" {"name" : "Alex", "age" : 22, "marks" : [10,8,9] } "#;
    let mut parser = JsonParser::new(source);
    let json = parser.parse().unwrap();
    let mut object = JsonObject::new();
    object.insert("name".to_owned(),JsonValue::JsonString("Alex".to_owned()));
    object.insert("age".to_owned(),JsonValue::JsonNumber(22.0));
    let marks = JsonValue::JsonArray(
        vec![JsonValue::JsonNumber(10.0),JsonValue::JsonNumber(8.0),JsonValue::JsonNumber(9.0)]);
    object.insert("marks".to_owned(),marks);
    
    assert_eq!(json,JsonValue::JsonObject(object));           

}
#[test]
fn test_array() {
    let source = r#" [{"number" : 123456, "object" : null, "valid" : false}, true, [1,2,3], "hello" ] "#;
    let mut parser = JsonParser::new(source);
    let json = parser.parse().unwrap();
    let mut object = JsonObject::new();
    object.insert("number".to_owned(), JsonValue::JsonNumber(123456.0));
    object.insert("object".to_owned(), JsonValue::Null);
    object.insert("valid".to_owned(), JsonValue::JsonBool(false));
    let array = vec![JsonValue::JsonNumber(1.0),JsonValue::JsonNumber(2.0),JsonValue::JsonNumber(3.0)];
    assert_eq!(json,JsonValue::JsonArray(vec![
    JsonValue::JsonObject(object),
    JsonValue::JsonBool(true),
    JsonValue::JsonArray(array),
    JsonValue::JsonString("hello".to_owned())
    ]));    

    let source = r#" [ "hello", false, null, -3.12e-10, [["A",true,null],["B",false,{"int" : 7}]]  ] "#;
    let mut parser = JsonParser::new(source);
    let json = parser.parse().unwrap();
    let mut object = JsonObject::new();
    object.insert("int".to_owned(),JsonValue::JsonNumber(7.0));
    let arrayA = vec![JsonValue::JsonString("A".to_owned()), JsonValue::JsonBool(true),JsonValue::Null];
    let arrayB = vec![JsonValue::JsonString("B".to_owned()), JsonValue::JsonBool(false),JsonValue::JsonObject(object)];
    let array = vec![JsonValue::JsonArray(arrayA),JsonValue::JsonArray(arrayB)];
    assert_eq!(json,JsonValue::JsonArray(vec![
        JsonValue::JsonString("hello".to_owned()),
        JsonValue::JsonBool(false),
        JsonValue::Null,
        JsonValue::JsonNumber(-3.12e-10),
        JsonValue::JsonArray(array),
    ]));
}

#[test]
fn test_object() {
    let source = r#" { "object" : 
    { 
    "int" : 5, 
    "poem" : "He's there, the Phantom of the Opera",
    "subobject" :
        {
        "array" : ["false", 55.33, null, [], true],
        "bool" : false,
        "subsubobject" : {},
        "random number" : 4
        },
    "vector" : [1,2,3]
    } 
    } "#;

    let mut parser = JsonParser::new(source);
    let json = parser.parse().unwrap();

    let mut object = JsonObject::new();
    object.insert("int".to_owned(), JsonValue::JsonNumber(5.0));
    object.insert("poem".to_owned(), JsonValue::JsonString("He's there, the Phantom of the Opera".to_owned()));

    let mut subobject = JsonObject::new();
    let array = JsonValue::JsonArray(vec![JsonValue::JsonString("false".to_owned()),
                     JsonValue::JsonNumber(55.33),
                     JsonValue::Null,
                     JsonValue::JsonArray(vec![]),
                     JsonValue::JsonBool(true)]);
    let subsubobject = JsonValue::JsonObject(JsonObject::new());
    subobject.insert("array".to_owned(), array);
    subobject.insert("bool".to_owned(), JsonValue::JsonBool(false));
    subobject.insert("subsubobject".to_owned(), subsubobject);
    subobject.insert("random number".to_owned(), JsonValue::JsonNumber(4.0));

    object.insert("subobject".to_owned(), JsonValue::JsonObject(subobject));

    let vector = JsonValue::JsonArray(
        vec![JsonValue::JsonNumber(1.0),JsonValue::JsonNumber(2.0),JsonValue::JsonNumber(3.0)]);

    object.insert("vector".to_owned(), vector);  

    let mut super_object =  JsonObject::new();
    super_object.insert("object".to_owned(),JsonValue::JsonObject(object));

    let result = JsonValue::JsonObject(super_object);

    assert_eq!(json,result);

}
