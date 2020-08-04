#![allow(dead_code)]
#![allow(non_snake_case)]
#[path = "lexer.rs"] mod lexer;
use tokenlib as token; //(TYPE: &str, LITERAL: STRING)

struct TestItem<'a> {
    expected_type: &'a str,
    expected_literal: String
}

pub fn test_simple_input() {
    println!("Testing simple input...");
    let _input = "!=;*/+{}()-<>".as_bytes();
    let tokens = lexer::tokenize(&_input);
    let tests: [TestItem; 14] =  [
        TestItem {expected_type: token::BANG, expected_literal:"!".to_string()},
        TestItem {expected_type: token::ASSIGN, expected_literal:"=".to_string()},
        TestItem {expected_type: token::SEMICOLON, expected_literal:";".to_string()},
        TestItem {expected_type: token::ASTERISK, expected_literal:"*".to_string()},
        TestItem {expected_type: token::SLASH, expected_literal:"/".to_string()},
        TestItem {expected_type: token::PLUS, expected_literal:"+".to_string()},
        TestItem {expected_type: token::LBRACE, expected_literal:"{".to_string()},
        TestItem {expected_type: token::RBRACE, expected_literal:"}".to_string()},
        TestItem {expected_type: token::LPAREN, expected_literal:"(".to_string()},
        TestItem {expected_type: token::RPAREN, expected_literal:")".to_string()},
        TestItem {expected_type: token::MINUS, expected_literal:"-".to_string()},
        TestItem {expected_type: token::LT, expected_literal:"<".to_string()},
        TestItem {expected_type: token::GT, expected_literal:">".to_string()},
        TestItem {expected_type: token::EOF, expected_literal:'\0'.to_string()},
    ];

    assert_eq!(tests.len(), tokens.len());
    for i in 0..tests.len() {
        assert_eq!(tests[i].expected_type, tokens[i].0);
        assert_eq!(tests[i].expected_literal, tokens[i].1);
    }

    println!("PASSED: test_simple_input");
}

pub fn test_identifiers_added() {
    println!("Test identifiers added");
    let _input = "let x = 5; y = fn(x)
    z = true ; j = false ;
    ".as_bytes();
    let tokens = lexer::tokenize(_input);
    let tests: [TestItem; 19] = [
        TestItem {expected_type: token::LET, expected_literal: "let".to_string()},
        TestItem {expected_type: token::IDENT, expected_literal: "x".to_string()},
        TestItem {expected_type: token::ASSIGN, expected_literal: "=".to_string()},
        TestItem {expected_type: token::INT, expected_literal: "5".to_string()},
        TestItem {expected_type: token::SEMICOLON, expected_literal: ";".to_string()},
        TestItem {expected_type: token::IDENT, expected_literal: "y".to_string()},
        TestItem {expected_type: token::ASSIGN, expected_literal: "=".to_string()},
        TestItem {expected_type: token::FUNCTION, expected_literal: "fn".to_string()},
        TestItem {expected_type: token::LPAREN, expected_literal: "(".to_string()},
        TestItem {expected_type: token::IDENT, expected_literal: "x".to_string()},
        TestItem {expected_type: token::RPAREN, expected_literal: ")".to_string()},
        TestItem {expected_type: token::IDENT, expected_literal: "z".to_string()},
        TestItem {expected_type: token::ASSIGN, expected_literal: "=".to_string()},
        TestItem {expected_type: token::TRUE, expected_literal: "true".to_string()},
        TestItem {expected_type: token::SEMICOLON, expected_literal: ";".to_string()},
        TestItem {expected_type: token::IDENT, expected_literal: "j".to_string()},
        TestItem {expected_type: token::ASSIGN, expected_literal: "=".to_string()},
        TestItem {expected_type: token::FALSE, expected_literal: "false".to_string()},
        TestItem {expected_type: token::SEMICOLON, expected_literal: ";".to_string()},
    ];
    for i in 0..tests.len() {
        assert_eq!(tests[i].expected_type, tokens[i].0);
        assert_eq!(tests[i].expected_literal, tokens[i].1);
    }
    println!("PASSED: test_identifiers_added");
}

pub fn test_eq_neq_gt_lt() {
    println!("Test test_eq_neq_gt_lt");
    let _input = "let x = 5; y = fn(x)
    z = true ; j = false ; x == 2; 3, != 5
    ".as_bytes();
    let tokens = lexer::tokenize(_input);
    let tests: [TestItem; 28] = [
        TestItem {expected_type: token::LET, expected_literal: "let".to_string()},
        TestItem {expected_type: token::IDENT, expected_literal: "x".to_string()},
        TestItem {expected_type: token::ASSIGN, expected_literal: "=".to_string()},
        TestItem {expected_type: token::INT, expected_literal: "5".to_string()},
        TestItem {expected_type: token::SEMICOLON, expected_literal: ";".to_string()},
        TestItem {expected_type: token::IDENT, expected_literal: "y".to_string()},
        TestItem {expected_type: token::ASSIGN, expected_literal: "=".to_string()},
        TestItem {expected_type: token::FUNCTION, expected_literal: "fn".to_string()},
        TestItem {expected_type: token::LPAREN, expected_literal: "(".to_string()},
        TestItem {expected_type: token::IDENT, expected_literal: "x".to_string()},
        TestItem {expected_type: token::RPAREN, expected_literal: ")".to_string()},
        TestItem {expected_type: token::IDENT, expected_literal: "z".to_string()},
        TestItem {expected_type: token::ASSIGN, expected_literal: "=".to_string()},
        TestItem {expected_type: token::TRUE, expected_literal: "true".to_string()},
        TestItem {expected_type: token::SEMICOLON, expected_literal: ";".to_string()},
        TestItem {expected_type: token::IDENT, expected_literal: "j".to_string()},
        TestItem {expected_type: token::ASSIGN, expected_literal: "=".to_string()},
        TestItem {expected_type: token::FALSE, expected_literal: "false".to_string()},
        TestItem {expected_type: token::SEMICOLON, expected_literal: ";".to_string()},
        TestItem {expected_type: token::IDENT, expected_literal: "x".to_string()},
        TestItem {expected_type: token::EQ, expected_literal: "==".to_string()},
        TestItem {expected_type: token::INT, expected_literal: "2".to_string()},
        TestItem {expected_type: token::SEMICOLON, expected_literal: ";".to_string()},
        TestItem {expected_type: token::INT, expected_literal: "3".to_string()},
        TestItem {expected_type: token::COMMA, expected_literal: ",".to_string()},
        TestItem {expected_type: token::NOT_EQ, expected_literal: "!=".to_string()},
        TestItem {expected_type: token::INT, expected_literal: "5".to_string()},
        TestItem {expected_type: token::EOF, expected_literal:'\0'.to_string()},
    ];
    for i in 0..tests.len() {
        assert_eq!(tests[i].expected_type, tokens[i].0);
        assert_eq!(tests[i].expected_literal, tokens[i].1);
    }
    println!("PASSED: test_eq_neq_gt_lt"); 
}

pub fn test_advanced_token() {
	let _input = "let five = 5;
	let ten = 10;
   	let add = fn(x, y) {
     x + y;
	};
	let result = add(five, ten);
	!-/*5;
	5 < 10 > 5;

	if (5 < 10) {
		return true;
	} else {
		return false;
	}

	10 == 10;
	10 != 9;\"hello\"
    ".as_bytes();
    
    let tokens = lexer::tokenize(&_input);

	let tests: [TestItem; 75] = [
		TestItem {expected_type: token::LET, expected_literal: "let".to_string()},
		TestItem {expected_type: token::IDENT, expected_literal: "five".to_string()},
		TestItem {expected_type: token::ASSIGN, expected_literal: "=".to_string()},
		TestItem {expected_type: token::INT, expected_literal: "5".to_string()},
		TestItem {expected_type: token::SEMICOLON, expected_literal: ";".to_string()},
		TestItem {expected_type: token::LET, expected_literal: "let".to_string()},
		TestItem {expected_type: token::IDENT, expected_literal: "ten".to_string()},
		TestItem {expected_type: token::ASSIGN, expected_literal: "=".to_string()},
		TestItem {expected_type: token::INT, expected_literal: "10".to_string()},
		TestItem {expected_type: token::SEMICOLON, expected_literal: ";".to_string()},
		TestItem {expected_type: token::LET, expected_literal: "let".to_string()},
		TestItem {expected_type: token::IDENT, expected_literal: "add".to_string()},
		TestItem {expected_type: token::ASSIGN, expected_literal: "=".to_string()},
		TestItem {expected_type: token::FUNCTION, expected_literal: "fn".to_string()},
		TestItem {expected_type: token::LPAREN, expected_literal: "(".to_string()},
		TestItem {expected_type: token::IDENT, expected_literal: "x".to_string()},
		TestItem {expected_type: token::COMMA, expected_literal: ",".to_string()},
		TestItem {expected_type: token::IDENT, expected_literal: "y".to_string()},
		TestItem {expected_type: token::RPAREN, expected_literal: ")".to_string()},
		TestItem {expected_type: token::LBRACE, expected_literal: "{".to_string()},
		TestItem {expected_type: token::IDENT, expected_literal: "x".to_string()},
		TestItem {expected_type: token::PLUS, expected_literal: "+".to_string()},
		TestItem {expected_type: token::IDENT, expected_literal: "y".to_string()},
		TestItem {expected_type: token::SEMICOLON, expected_literal: ";".to_string()},
		TestItem {expected_type: token::RBRACE, expected_literal: "}".to_string()},
		TestItem {expected_type: token::SEMICOLON, expected_literal: ";".to_string()},
		TestItem {expected_type: token::LET, expected_literal: "let".to_string()},
		TestItem {expected_type: token::IDENT, expected_literal: "result".to_string()},
		TestItem {expected_type: token::ASSIGN, expected_literal: "=".to_string()},
		TestItem {expected_type: token::IDENT, expected_literal: "add".to_string()},
		TestItem {expected_type: token::LPAREN, expected_literal: "(".to_string()},
		TestItem {expected_type: token::IDENT, expected_literal: "five".to_string()},
		TestItem {expected_type: token::COMMA, expected_literal: ",".to_string()},
		TestItem {expected_type: token::IDENT, expected_literal: "ten".to_string()},
		TestItem {expected_type: token::RPAREN, expected_literal: ")".to_string()},
		TestItem {expected_type: token::SEMICOLON, expected_literal: ";".to_string()},
		TestItem {expected_type: token::BANG, expected_literal: "!".to_string()},
		TestItem {expected_type: token::MINUS, expected_literal: "-".to_string()},
		TestItem {expected_type: token::SLASH, expected_literal: "/".to_string()},
		TestItem {expected_type: token::ASTERISK, expected_literal: "*".to_string()},
		TestItem {expected_type: token::INT, expected_literal: "5".to_string()},
		TestItem {expected_type: token::SEMICOLON, expected_literal: ";".to_string()},
		TestItem {expected_type: token::INT, expected_literal: "5".to_string()},
		TestItem {expected_type: token::LT, expected_literal: "<".to_string()},
		TestItem {expected_type: token::INT, expected_literal: "10".to_string()},
		TestItem {expected_type: token::GT, expected_literal: ">".to_string()},
		TestItem {expected_type: token::INT, expected_literal: "5".to_string()},
		TestItem {expected_type: token::SEMICOLON, expected_literal: ";".to_string()},
		TestItem {expected_type: token::IF, expected_literal: "if".to_string()},
		TestItem {expected_type: token::LPAREN, expected_literal: "(".to_string()},
		TestItem {expected_type: token::INT, expected_literal: "5".to_string()},
		TestItem {expected_type: token::LT, expected_literal: "<".to_string()},
		TestItem {expected_type: token::INT, expected_literal: "10".to_string()},
		TestItem {expected_type: token::RPAREN, expected_literal: ")".to_string()},
		TestItem {expected_type: token::LBRACE, expected_literal: "{".to_string()},
		TestItem {expected_type: token::RETURN, expected_literal: "return".to_string()},
		TestItem {expected_type: token::TRUE, expected_literal: "true".to_string()},
		TestItem {expected_type: token::SEMICOLON, expected_literal: ";".to_string()},
		TestItem {expected_type: token::RBRACE, expected_literal: "}".to_string()},
		TestItem {expected_type: token::ELSE, expected_literal: "else".to_string()},
		TestItem {expected_type: token::LBRACE, expected_literal: "{".to_string()},
		TestItem {expected_type: token::RETURN, expected_literal: "return".to_string()},
		TestItem {expected_type: token::FALSE, expected_literal: "false".to_string()},
		TestItem {expected_type: token::SEMICOLON, expected_literal: ";".to_string()},
		TestItem {expected_type: token::RBRACE, expected_literal: "}".to_string()},
		TestItem {expected_type: token::INT, expected_literal: "10".to_string()},
		TestItem {expected_type: token::EQ, expected_literal: "==".to_string()},
		TestItem {expected_type: token::INT, expected_literal: "10".to_string()},
		TestItem {expected_type: token::SEMICOLON, expected_literal: ";".to_string()},
		TestItem {expected_type: token::INT, expected_literal: "10".to_string()},
		TestItem {expected_type: token::NOT_EQ, expected_literal: "!=".to_string()},
		TestItem {expected_type: token::INT, expected_literal: "9".to_string()},
        TestItem {expected_type: token::SEMICOLON, expected_literal: ";".to_string()},
        TestItem {expected_type: token::STRING, expected_literal: "hello".to_string()},
		TestItem {expected_type: token::EOF, expected_literal: '\0'.to_string()},
    ];
    
    assert_eq!(tests.len(), tokens.len());
    for i in 0..tests.len() {
        assert_eq!(tests[i].expected_type, tokens[i].0);
        assert_eq!(tests[i].expected_literal, tokens[i].1);
    }

    println!("PASSED: test_advanced_token");
}