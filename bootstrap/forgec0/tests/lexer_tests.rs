use forgec0::{Token, tokenize};

#[test]
fn test_full_capability_declaration() {
    let input = "fn process(data: Text) -> Int !{net, io, tokens ≤ 100}";
    let tokens = tokenize(input);
    
    let expected = vec![
        Token::Fn,
        Token::Ident("process".to_string()),
        Token::LParen,
        Token::Ident("data".to_string()),
        Token::Colon,
        Token::Ident("Text".to_string()),
        Token::RParen,
        Token::Arrow,
        Token::Ident("Int".to_string()),
        Token::Bang,
        Token::LBrace,
        Token::Ident("net".to_string()),
        Token::Comma,
        Token::Ident("io".to_string()),
        Token::Comma,
        Token::Ident("tokens".to_string()),
        Token::LessThanEqual,
        Token::Number(100),
        Token::RBrace,
        Token::Eof,
    ];
    
    assert_eq!(tokens, expected);
}

#[test]
fn test_module_with_capability() {
    let input = "module data.pipeline !{energy ≤ 10}";
    let tokens = tokenize(input);
    
    assert_eq!(tokens[0], Token::Module);
    assert_eq!(tokens[1], Token::Ident("data".to_string()));
    // Note: period is not yet handled, will be Unknown('.')
    assert_eq!(tokens[2], Token::Unknown('.'));
    assert_eq!(tokens[3], Token::Ident("pipeline".to_string()));
    assert_eq!(tokens[4], Token::Bang);
    assert_eq!(tokens[5], Token::LBrace);
    assert_eq!(tokens[6], Token::Ident("energy".to_string()));
    assert_eq!(tokens[7], Token::LessThanEqual);
    assert_eq!(tokens[8], Token::Number(10));
    assert_eq!(tokens[9], Token::RBrace);
}

#[test]
fn test_intent_blocks() {
    let input = "⟦ sort_by relevance group ≤ 16 ⟧";
    let tokens = tokenize(input);
    
    assert_eq!(tokens[0], Token::IntentOpen);
    assert_eq!(tokens[1], Token::Ident("sort_by".to_string()));
    assert_eq!(tokens[2], Token::Ident("relevance".to_string()));
    assert_eq!(tokens[3], Token::Ident("group".to_string()));
    assert_eq!(tokens[4], Token::LessThanEqual);
    assert_eq!(tokens[5], Token::Number(16));
    assert_eq!(tokens[6], Token::IntentClose);
}

#[test]
fn test_multiple_resource_constraints() {
    let input = "!{tokens ≤ 8, latency ≤ 200, energy ≤ 5}";
    let tokens = tokenize(input);
    
    assert_eq!(tokens[0], Token::Bang);
    assert_eq!(tokens[1], Token::LBrace);
    
    // First constraint
    assert_eq!(tokens[2], Token::Ident("tokens".to_string()));
    assert_eq!(tokens[3], Token::LessThanEqual);
    assert_eq!(tokens[4], Token::Number(8));
    assert_eq!(tokens[5], Token::Comma);
    
    // Second constraint
    assert_eq!(tokens[6], Token::Ident("latency".to_string()));
    assert_eq!(tokens[7], Token::LessThanEqual);
    assert_eq!(tokens[8], Token::Number(200));
    assert_eq!(tokens[9], Token::Comma);
    
    // Third constraint
    assert_eq!(tokens[10], Token::Ident("energy".to_string()));
    assert_eq!(tokens[11], Token::LessThanEqual);
    assert_eq!(tokens[12], Token::Number(5));
    
    assert_eq!(tokens[13], Token::RBrace);
}