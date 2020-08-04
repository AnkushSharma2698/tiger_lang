use std::io;
use std::io::Write;
#[path = "lexer.rs"] mod lexer;

pub fn start() {
    const PROMPT: &str = "~> ";
    loop {
        print!("{}", PROMPT);
        let line = get_stdin_input();
        if line.len() == 0 {
            return
        }
        let byte_input = line.as_bytes();
        println!("{:?}", byte_input);
        let tokens = lexer::tokenize(byte_input);
        println!("{:?}", tokens);
    }
}

// Ensure strings are prefaced with escaping characters \"
fn get_stdin_input() -> String {
    // Explicit Sync
            io::stdout().flush().unwrap();
            let mut line = String::new();
            io::stdin().read_line(&mut line)
                .expect("Error getting input");
            trim_newline(&mut line);
            return line;
    }
    
    fn trim_newline(s: &mut String) {
        if s.ends_with('\n') {
            s.pop();
            if s.ends_with('\r') {
                s.pop();
            }
        }
    }