//!Lexical analyzer for the lolspeak

use crate::traits::LexicalAnalyzer;
use std::process;

///Lays out the valid words for lolspeak
const ANNOTATIONS: [&str; 11] = [
    "#HAI", "#KBYE", "#OBTW", "#TLDR", "#MAEK", "#OIC", "#GIMMEH", "#MKAY", "#NEWLINE", "#IHAZ",
    "#ITIZ",
];

///The file's characters, the current position in the file, the current token being built, and the current line number
pub struct Lexer {
    chars: Vec<char>,
    position: usize,
    current_lexeme: String,
    pub line: usize,
}

impl Lexer {
    ///Sets up the lexer with the source code
    pub fn new(source: &str) -> Lexer {
        Lexer {
            chars: source.chars().collect(),
            position: 0,
            current_lexeme: String::new(),
            line: 1,
        }
    }

    ///Returns true if there are more characters
    fn has_more_chars(&self) -> bool {
        self.position < self.chars.len()
    }

    ///Returns the next character without advancing the position
    fn peek_char(&self) -> char {
        self.chars[self.position]
    }

    ///Returns the next token from the input
    pub fn next_token(&mut self) -> Result<String, String> {
        while self.has_more_chars() && self.peek_char().is_whitespace() {
            self.get_char();
        }

        self.current_lexeme = String::new();

        if !self.has_more_chars() {
            return Ok(String::new());
        }

        if self.peek_char() == '#' {
            let c = self.get_char();
            self.add_char(c);
            while self.has_more_chars() && self.peek_char().is_ascii_alphabetic() {
                let c = self.get_char();
                self.add_char(c);
            }
        } else {
            while self.has_more_chars() && !self.peek_char().is_whitespace() && self.peek_char() != '#' {
                let c = self.get_char();
                self.add_char(c);
            }
        }

        if self.lookup(&self.current_lexeme) {
            Ok(self.current_lexeme.clone())
        } else if self.current_lexeme.starts_with('#') {
            Err(format!(
                "Lexical error on line {}: {} is not a valid annotation",
                self.line, self.current_lexeme
            ))
        } else {
            Err(format!(
                "Lexical error on line {}: {} contains a character that is not allowed",
                self.line, self.current_lexeme
            ))
        }
    }

    ///Returns the next token without advancing the position
    pub fn peek_token(&mut self) -> Result<String, String> {
        let saved_position = self.position;
        let saved_line = self.line;
        let token = self.next_token();
        self.position = saved_position;
        self.line = saved_line;
        token
    }
}

///Returns true if the character is valid
pub fn is_text_char(c: char) -> bool {
    if c.is_ascii_alphanumeric() {
        return true;
    }
    match c {
        ',' | '.' | '"' | ':' | '?' | '!' | '%' | '=' | '/' | '\'' | '-' | '(' | ')' => true,
        _ => false,
    }
}

impl LexicalAnalyzer for Lexer {
    ///Returns the next character from the input
    fn get_char(&mut self) -> char {
        if !self.has_more_chars() {
            eprintln!("Lexical error: unexpected end of file");
            process::exit(1);
        }
        let c = self.chars[self.position];
        self.position += 1;
        if c == '\n' {
            self.line += 1;
        }
        c
    }

    ///Adds a character to the current token
    fn add_char(&mut self, c: char) {
        self.current_lexeme.push(c);
    }

    ///Looks up a potential token to determine if it's valid
    fn lookup(&self, s: &str) -> bool {
        if s.starts_with('#') {
            let upper = s.to_uppercase();
            for annotation in ANNOTATIONS {
                if annotation == upper {
                    return true;
                }
            }
            return false;
        }

        if s.is_empty() {
            return false;
        }
        for c in s.chars() {
            if !is_text_char(c) {
                return false;
            }
        }
        true
    }
}
