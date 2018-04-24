use lex::{Lex,Token,TokenType};

// assert_eq!(lexer.next(),Token{span: &json[], token_type: } );


#[test]
fn test_simple()
{
    let json = r#"-3.12e-10 [-4559,12.66,"string",[]] true false null {}"#;
    let mut lexer = Lex::new(json);
    assert_eq!(lexer.next(),Token{span: &json[0..9], token_type: TokenType::Number(-3.12e-10)});
    assert_eq!(lexer.next(),Token{span: &json[10..11], token_type: TokenType::LeftBracket});
    assert_eq!(lexer.next(),Token{span: &json[11..16], token_type: TokenType::Number(-4559.0)});
    assert_eq!(lexer.next(),Token{span: &json[16..17], token_type: TokenType::Comma});
    assert_eq!(lexer.next(),Token{span: &json[17..22], token_type: TokenType::Number(12.66)});
    assert_eq!(lexer.next(),Token{span: &json[22..23], token_type: TokenType::Comma});
    assert_eq!(lexer.next(),Token{span: &json[23..31], token_type: TokenType::String("string".to_string())});
    assert_eq!(lexer.next(),Token{span: &json[31..32], token_type: TokenType::Comma});
    assert_eq!(lexer.next(),Token{span: &json[32..33], token_type: TokenType::LeftBracket});
    assert_eq!(lexer.next(),Token{span: &json[33..34], token_type: TokenType::RightBracket});
    assert_eq!(lexer.next(),Token{span: &json[34..35], token_type: TokenType::RightBracket});
    assert_eq!(lexer.next(),Token{span: &json[36..40], token_type: TokenType::Bool(true)});
    assert_eq!(lexer.next(),Token{span: &json[41..46], token_type: TokenType::Bool(false)});
    assert_eq!(lexer.next(),Token{span: &json[47..51], token_type: TokenType::Null});
    assert_eq!(lexer.next(),Token{span: &json[52..53], token_type: TokenType::LeftBrace});
    assert_eq!(lexer.next(),Token{span: &json[53..54], token_type: TokenType::RightBrace});
    assert_eq!(lexer.next(),Token{span: "", token_type : TokenType::End});
}


#[test]
fn test1()
{
    let json = r#"{ "integer": 565, "string": "test string", "bool":true, "lie":false }"#;
    let mut lexer = Lex::new(json);
    assert_eq!(lexer.next(),Token{span: &json[0..1], token_type: TokenType::LeftBrace});
    assert_eq!(lexer.next(),Token{span: &json[2..11], token_type: TokenType::String("integer".to_string())});
    assert_eq!(lexer.next(),Token{span: &json[11..12], token_type : TokenType::Colon} );
    assert_eq!(lexer.next(),Token{span: &json[13..16], token_type : TokenType::Number(565.0)} );
    assert_eq!(lexer.next(),Token{span: &json[16..17], token_type : TokenType::Comma} );
    assert_eq!(lexer.next(),Token{span: &json[18..26], token_type : TokenType::String("string".to_string())} );
    assert_eq!(lexer.next(),Token{span: &json[26..27], token_type : TokenType::Colon});
    assert_eq!(lexer.next(),Token{span: &json[28..41], token_type : TokenType::String("test string".to_string())} );
    assert_eq!(lexer.next(),Token{span: &json[41..42], token_type : TokenType::Comma} );
    assert_eq!(lexer.next(),Token{span: &json[43..49], token_type : TokenType::String("bool".to_string())} );
    assert_eq!(lexer.next(),Token{span: &json[49..50], token_type : TokenType::Colon});
    assert_eq!(lexer.next(),Token{span: &json[50..54], token_type : TokenType::Bool(true)});
    assert_eq!(lexer.next(),Token{span: &json[54..55], token_type : TokenType::Comma} );
    assert_eq!(lexer.next(),Token{span: &json[56..61], token_type : TokenType::String("lie".to_string())} );
    assert_eq!(lexer.next(),Token{span: &json[61..62], token_type : TokenType::Colon});
    assert_eq!(lexer.next(),Token{span: &json[62..67], token_type : TokenType::Bool(false)});
    assert_eq!(lexer.next(),Token{span: &json[68..69], token_type : TokenType::RightBrace});
    assert_eq!(lexer.next(),Token{span: "", token_type : TokenType::End});
}

#[test]
fn test2()
{
    let json = r#"{ "float": -64.452672, "exp":10.3e10 "car":null, "object": { "name":"object" } }"#;
    let mut lexer = Lex::new(json);
    assert_eq!(lexer.next(),Token{span: &json[0..1], token_type: TokenType::LeftBrace});
    assert_eq!(lexer.next(),Token{span: &json[2..9], token_type: TokenType::String("float".to_string())});
    assert_eq!(lexer.next(),Token{span: &json[9..10], token_type : TokenType::Colon});
    assert_eq!(lexer.next(),Token{span: &json[11..21], token_type : TokenType::Number(-64.452672)});
    assert_eq!(lexer.next(),Token{span: &json[21..22], token_type : TokenType::Comma});
    assert_eq!(lexer.next(),Token{span: &json[23..28], token_type: TokenType::String("exp".to_string())});
    assert_eq!(lexer.next(),Token{span: &json[28..29], token_type : TokenType::Colon});
    assert_eq!(lexer.next(),Token{span: &json[29..36], token_type : TokenType::Number(10.3e10)});
    assert_eq!(lexer.next(),Token{span: &json[37..42], token_type: TokenType::String("car".to_string())});
    assert_eq!(lexer.next(),Token{span: &json[42..43], token_type : TokenType::Colon});
    assert_eq!(lexer.next(),Token{span: &json[43..47], token_type : TokenType::Null});
    assert_eq!(lexer.next(),Token{span: &json[47..48], token_type : TokenType::Comma});
    assert_eq!(lexer.next(),Token{span: &json[49..57], token_type: TokenType::String("object".to_string())});
    assert_eq!(lexer.next(),Token{span: &json[57..58], token_type : TokenType::Colon});
    assert_eq!(lexer.next(),Token{span: &json[59..60], token_type: TokenType::LeftBrace});
    assert_eq!(lexer.next(),Token{span: &json[61..67], token_type: TokenType::String("name".to_string())});
    assert_eq!(lexer.next(),Token{span: &json[67..68], token_type : TokenType::Colon});
    assert_eq!(lexer.next(),Token{span: &json[68..76], token_type : TokenType::String("object".to_string())});
    assert_eq!(lexer.next(),Token{span: &json[77..78], token_type: TokenType::RightBrace});
    assert_eq!(lexer.next(),Token{span: &json[79..80], token_type: TokenType::RightBrace});
    assert_eq!(lexer.next(),Token{span: "", token_type : TokenType::End});
}

#[test]
fn test_array()
{
    let json = r#"{ "array" : [["anna",170,62],["matthew",182,84]] }"#;
    let mut lexer = Lex::new(json);
    assert_eq!(lexer.next(),Token{span: &json[0..1], token_type: TokenType::LeftBrace});
    assert_eq!(lexer.next(),Token{span: &json[2..9], token_type: TokenType::String("array".to_string())});
    assert_eq!(lexer.next(),Token{span: &json[10..11], token_type : TokenType::Colon});
    assert_eq!(lexer.next(),Token{span: &json[12..13], token_type: TokenType::LeftBracket});
    assert_eq!(lexer.next(),Token{span: &json[13..14], token_type: TokenType::LeftBracket});
    assert_eq!(lexer.next(),Token{span: &json[14..20], token_type: TokenType::String("anna".to_string())});
    assert_eq!(lexer.next(),Token{span: &json[20..21], token_type : TokenType::Comma});
    assert_eq!(lexer.next(),Token{span: &json[21..24], token_type: TokenType::Number(170.0)});
    assert_eq!(lexer.next(),Token{span: &json[24..25], token_type : TokenType::Comma});
    assert_eq!(lexer.next(),Token{span: &json[25..27], token_type: TokenType::Number(62.0)});
    assert_eq!(lexer.next(),Token{span: &json[27..28], token_type: TokenType::RightBracket});
    assert_eq!(lexer.next(),Token{span: &json[28..29], token_type : TokenType::Comma});
    assert_eq!(lexer.next(),Token{span: &json[29..30], token_type: TokenType::LeftBracket});
    assert_eq!(lexer.next(),Token{span: &json[30..39], token_type: TokenType::String("matthew".to_string())});
    assert_eq!(lexer.next(),Token{span: &json[39..40], token_type : TokenType::Comma});
    assert_eq!(lexer.next(),Token{span: &json[40..43], token_type: TokenType::Number(182.0)});
    assert_eq!(lexer.next(),Token{span: &json[43..44], token_type : TokenType::Comma});
    assert_eq!(lexer.next(),Token{span: &json[44..46], token_type: TokenType::Number(84.0)});
    assert_eq!(lexer.next(),Token{span: &json[46..47], token_type: TokenType::RightBracket});
    assert_eq!(lexer.next(),Token{span: &json[47..48], token_type: TokenType::RightBracket});
    assert_eq!(lexer.next(),Token{span: &json[49..50], token_type: TokenType::RightBrace});
    assert_eq!(lexer.next(),Token{span: "", token_type : TokenType::End});
}

#[test]
fn test_special_chars()
{   
    let json = "\"\u{0041}\u{0042}\u{0043}\u{0044}\""; // \u{00B6}\u{03B1}";
    let mut lexer = Lex::new(json);
    assert_eq!(lexer.next(),Token{span: &json[0..6], token_type : TokenType::String("ABCD".to_string())});
    
    let json = "\"\u{00B6}\u{03B1}\""; // \u{00B6}\u{03B1}";
    let mut lexer = Lex::new(json);
    assert_eq!(lexer.next(),Token{span: &json[0..6], token_type : TokenType::String("¶α".to_string())});
}
