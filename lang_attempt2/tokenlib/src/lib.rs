#![allow(non_snake_case)]

// Token.0 is Type
// Token.1 is Literal
#[derive(Debug)]
pub struct Token<'a>(pub &'a str, pub String);

impl Token<'_>{
    pub fn new(Type: &str, Literal: char) -> Token{
        Token(Type, Literal.to_string())
    }

    pub fn init(Type: &str, Literal: String) -> Token {
        Token(Type, Literal)
    }
}

pub fn lookup_ident(Literal: String) -> &'static str {
    let ident: &'static str;
    match Literal.as_str() {
		"let" => ident = LET,
		"fn" => ident = FUNCTION,
		"true" => ident = TRUE,
		"false" => ident = FALSE,
		"if" => ident = IF,
		"else" => ident = ELSE,
		"return" => ident = RETURN,
		_ => ident = IDENT
    }
    ident
}

pub const ILLEGAL : &str = "ILLEGAL"; // Any tokens we do not know about

pub const EOF : &str = "EOF"; // EOF lets our parser know it can stop
pub const EOF_CH : char = '\0';

// Identifiers + Literals
pub const IDENT : &str = "IDENT";

pub const INT : &str = "INT";

pub const STRING: &str = "STRING";
pub const STRING_QUOTE: char = '"';

// ===OPERATORS===
pub const ASSIGN : &str = "=";
pub const ASSIGN_CH: char = '=';

pub const PLUS : &str = "+";
pub const PLUS_CH : char = '+';

pub const MINUS : &str = "-";
pub const MINUS_CH : char = '-';

pub const BANG : &str = "!";
pub const BANG_CH : char = '!';

pub const ASTERISK : &str = "*";
pub const ASTERISK_CH : char = '*';

pub const SLASH : &str = "/";
pub const SLASH_CH : char = '/';

pub const LT : &str = "<";
pub const LT_CH : char = '<';

pub const GT : &str = ">";
pub const GT_CH : char = '>';

pub const EQ : &str = "==";

pub const NOT_EQ : &str = "!=";

pub const GT_OR_EQ: &str = "GREATER_EQUAL";

pub const LT_OR_EQ: &str = "LESS_EQUAL";

// ===DELIMETERS===
pub const COMMA : &str = ",";
pub const COMMA_CH : char = ',';

pub const SEMICOLON : &str = ";";
pub const SEMICOLON_CH : char = ';';

pub const LPAREN : &str = "(";
pub const LPAREN_CH: char = '(';

pub const RPAREN : &str = ")";
pub const RPAREN_CH: char = ')';

pub const LBRACE : &str = "{";
pub const LBRACE_CH: char = '{';

pub const RBRACE : &str = "}";
pub const RBRACE_CH: char = '}';

pub const LBRACK : &str = "[";

pub const RBRACK : &str = "]";

// ===KEYWORDS===
pub const FUNCTION : &str = "FUNCTION";
pub const LET : &str = "LET";
pub const TRUE: &str = "TRUE";
pub const FALSE: &str = "FALSE";
pub const IF: &str = "IF";
pub const ELSE: &str = "ELSE";
pub const RETURN: &str = "RETURN";
pub const USE: &str = "USE"; // This will be implemented separately, since it will be an anonymous function