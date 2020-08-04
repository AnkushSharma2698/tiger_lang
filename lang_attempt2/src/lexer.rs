#![allow(non_snake_case)]
use tokenlib as tl; //(TYPE: &str, LITERAL: STRING)

// Returns a list of tokens: Tuple (&str, String)
pub fn tokenize(input: &[u8]) -> Vec<tl::Token> {
    let mut output:Vec<tl::Token> = Vec::new();

    // Initialize our reading values
    let mut position: usize = 0; // Current position in input we are at
    let mut read_position: usize = 0; // Next position we want to read in input
    let mut ch: char; // Currently what character is being read
    ch = read_char(input, &mut position, &mut read_position);
    let mut tokenized = false;
    loop {
        if tokenized {
            break
        }
        // Get the next token
        let token = next_token(input, &mut position, &mut read_position, &mut ch);
        if token.0 == tl::EOF {
            tokenized = true;
        }
        output.push(token);
    }

    output
}

fn next_token<'a>(input: &[u8], position: &mut usize, read_position: &mut usize, ch: &mut char) -> tl::Token<'a> {
    // Initialize token
    let token: tl::Token;

    // Skip whitespace
    while *ch == ' ' || *ch == '\t' || *ch == '\r' || *ch == '\n' {
        *ch = read_char(input, position, read_position);
    }
    // Match a token
    match *ch {
        tl::ASSIGN_CH => {
                if peek_char(input, read_position) == '=' {
                    let mut eq = String::new();
                    eq.push(ch.clone() as char);
                    *ch = read_char(input, position, read_position);
                    eq.push(ch.clone() as char);
                    token = tl::Token::init(tl::EQ, eq);
                } else {
                    token = tl::Token::new(tl::ASSIGN, *ch);
                }
        },
        tl::SEMICOLON_CH => token = tl::Token::new(tl::SEMICOLON, *ch),
        tl::LPAREN_CH => token = tl::Token::new(tl::LPAREN, *ch),
        tl::RPAREN_CH => token = tl::Token::new(tl::RPAREN, *ch),
        tl::COMMA_CH => token = tl::Token::new(tl::COMMA, *ch),
        tl::PLUS_CH => token = tl::Token::new(tl::PLUS, *ch),
        tl::BANG_CH =>  {
            if peek_char(input, read_position) == '=' {
                let mut eq = String::new();
                eq.push(ch.clone() as char);
                *ch = read_char(input, position, read_position);
                eq.push(ch.clone() as char);
                token = tl::Token::init(tl::NOT_EQ, eq);
            } else {
                token = tl::Token::new(tl::BANG, *ch);
            }
        },
        tl::SLASH_CH => token = tl::Token::new(tl::SLASH, *ch),
        tl::MINUS_CH => token = tl::Token::new(tl::MINUS, *ch),
        tl::ASTERISK_CH => token = tl::Token::new(tl::ASTERISK, *ch),
        tl::LT_CH =>  {
            if peek_char(input, read_position) == '=' {
                let mut eq = String::new();
                eq.push(ch.clone() as char);
                *ch = read_char(input, position, read_position);
                eq.push(ch.clone() as char);
                token = tl::Token::init(tl::LT_OR_EQ, eq);
            } else {
                token = tl::Token::new(tl::LT, *ch)
            }
        },
        tl::GT_CH => {
            if peek_char(input, read_position) == '=' {
                let mut eq = String::new();
                eq.push(ch.clone() as char);
                *ch = read_char(input, position, read_position);
                eq.push(ch.clone() as char);
                token = tl::Token::init(tl::GT_OR_EQ, eq); 
            } else {
                token = tl::Token::new(tl::GT, *ch)
            }
        },
        tl::LBRACE_CH => token = tl::Token::new(tl::LBRACE, *ch),
        tl::RBRACE_CH => token = tl::Token::new(tl::RBRACE, *ch),
        tl::STRING_QUOTE => {
            let Literal: String = read_string(input, position, read_position, ch);
            let Type: &str = tl::STRING;
            token = tl::Token::init(Type, Literal);
        }
        tl::EOF_CH => token = tl::Token::new(tl::EOF, *ch),
        _ => {
            if is_letter(*ch) {
                // Read the identifier
                let Literal: String = read_identifier(input, position, read_position, ch);
                let Type: &str = tl::lookup_ident(Literal.clone());
                token = tl::Token::init(Type, Literal);
                return token;
            } else if is_digit(*ch) {
                // Read the number
                let Literal: String = read_number(input, position, read_position, ch);
                let Type: &str = tl::INT;
                token = tl::Token::init(Type, Literal);
                return token;
            } else {
                // Illegal Token
                token = tl::Token::new(tl::ILLEGAL, *ch)
            }
        }
    }
    *ch = read_char(input, position, read_position);
    token

}

// Arbitrarily reads a character given an input array, position and readposition
fn read_char<'a>(input: &'a [u8], position: &mut usize, read_position: &mut usize) -> char {
    let mut ch = '\0';
    if *read_position < input.len() {
        ch = input[*read_position] as char;
    }
    *position = *read_position;
    *read_position += 1;
    ch
}

fn read_identifier<'a>(input: &'a [u8], position: &mut usize, read_position: &mut usize, ch: &mut char) -> String {
    let mut identifier = String::new();
    loop {
        if !is_letter(ch.clone()) {
            break
        }
        identifier.push(ch.clone());
        *ch = read_char(input, position, read_position);
    }
    identifier
}


fn read_number<'a>(input: &'a [u8], position: &mut usize, read_position: &mut usize, ch: &mut char) -> String {
    let mut number = String::new();
    loop {
        if !is_digit(ch.clone()) {
            break
        }
        number.push(ch.clone());
        *ch = read_char(input, position, read_position);
    }
    number
}

fn read_string<'a>(input: &'a [u8], position: &mut usize, read_position: &mut usize, ch: &mut char) -> String {
    let mut string = String::new();
    // Pass in initial quote character
    loop {
        *ch = read_char(input, position, read_position);
        if *ch == tl::STRING_QUOTE || *ch == tl::EOF_CH {
            // There can be the case where a user forgets to add ending double quotes, so look for EOF as a fallback failure
            break
        }
        string.push(ch.clone());
    }

    string
}

// Peek one character ahead
fn peek_char<'a>(input: &'a [u8], read_position: &mut usize) -> char {
    if *read_position >= input.len() {
        '\0'
    } else {
        input[*read_position] as char
    }
}

fn is_letter(ch: char) -> bool {
    if ch.is_alphabetic() || ch == '_' {
        true
    } else {
        false
    }
}

fn is_digit(ch: char) -> bool {
        // TODO: Constrain this is_numeric function to only include 0 to 9 while only integers are supported
        if ch.is_numeric() {
            true
        } else {
            false
        }
}