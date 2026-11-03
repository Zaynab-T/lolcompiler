//!Recursive descent parser for lolspeak

use crate::lexer::Lexer;
use crate::semantic;
use crate::traits::{Compiler, SyntaxAnalyzer};
use std::process;

///A token and its line number
pub struct Node {
    pub token: String,
    pub line: usize,
}

///The lexer, current token, parse tree, and generated HTML
pub struct LolCompiler {
    pub lexer: Lexer,
    pub current: String,
    pub parse_tree: Vec<Node>,
    pub html: String,
}

impl LolCompiler {
    ///Reads the source code and initializes the compiler
    pub fn new(source: &str) -> LolCompiler {
        LolCompiler {
            lexer: Lexer::new(source),
            current: String::new(),
            parse_tree: Vec::new(),
            html: String::new(),
        }
    }

    ///Resets lexer and tree, reads first token, parses, generates HTML
    pub fn compile_source(&mut self, source: &str) -> Result<(), String> {
        self.lexer = Lexer::new(source);
        self.parse_tree = Vec::new();
        self.html = String::new();

        self.advance()?;
        self.parse_lolcode()?;
        self.html = semantic::generate_html(&self.parse_tree)?;
        Ok(())
    }

    ///Gets the next token from the lexer
    fn advance(&mut self) -> Result<(), String> {
        self.current = self.lexer.next_token()?;
        Ok(())
    }

    ///Checks if the current token matches the expected string
    fn is(&self, s: &str) -> bool {
        self.current.to_uppercase() == s
    }

    ///Checks if the current token is text
    fn is_text(&self) -> bool {
        !self.current.is_empty() && !self.current.starts_with('#')
    }

    ///Peeks at the next token
    fn next_is(&mut self, s: &str) -> Result<bool, String> {
        let next = self.lexer.peek_token()?;
        Ok(next.to_uppercase() == s)
    }

    ///Adds a token to the parse tree
    fn add_to_tree(&mut self, token: String) {
        let line = self.lexer.line;
        self.parse_tree.push(Node { token, line });
    }

    ///Builds a syntax error message
    fn syntax_error(&self, expected: &str) -> String {
        let found = if self.current.is_empty() {
            String::from("end of file")
        } else {
            self.current.clone()
        };
        format!(
            "Syntax error on line {}: expected {} but found {}",
            self.lexer.line, expected, found
        )
    }

    ///Expects a specific token and advances to the next one
    fn expect(&mut self, s: &str) -> Result<(), String> {
        if !self.is(s) {
            return Err(self.syntax_error(s));
        }
        self.add_to_tree(s.to_string());
        self.advance()
    }

    ///Expects a pair of tokens and advances to the next one
    fn expect_pair(&mut self, first: &str, second: &str) -> Result<(), String> {
        if !self.is(first) {
            return Err(self.syntax_error(first));
        }
        self.advance()?;
        if !self.is(second) {
            return Err(self.syntax_error(second));
        }
        self.add_to_tree(format!("{} {}", first, second));
        self.advance()
    }

    ///Parses optional variable definition
    fn parse_optional_define(&mut self) -> Result<(), String> {
        if self.is("#IHAZ") {
            self.parse_variable_define()?;
        }
        Ok(())
    }

    ///Parses comments until no more are found
    fn parse_comments(&mut self) -> Result<(), String> {
        while self.is("#OBTW") {
            self.parse_comment()?;
        }
        Ok(())
    }

    ///Parses a #GIMMEH statement which can be followed by BOLD, LINX, or VAR
    fn parse_gimmeh(&mut self) -> Result<(), String> {
        if self.next_is("BOLD")? {
            self.parse_bold()
        } else if self.next_is("LINX")? {
            self.parse_link()
        } else if self.next_is("VAR")? {
            self.parse_variable_use()
        } else {
            self.advance()?;
            Err(self.syntax_error("BOLD, LINX or VAR"))
        }
    }

    ///Parses inner body of the document
    fn parse_inner_body(&mut self) -> Result<(), String> {
        if self.is("#MAEK") {
            if self.next_is("PARAGRAF")? {
                self.parse_paragraph()
            } else if self.next_is("LIST")? {
                self.parse_list()
            } else {
                self.advance()?;
                Err(self.syntax_error("PARAGRAF or LIST"))
            }
        } else if self.is("#GIMMEH") {
            self.parse_gimmeh()
        } else if self.is("#OBTW") {
            self.parse_comment()
        } else if self.is("#NEWLINE") {
            self.parse_newline()
        } else if self.is_text() {
            self.parse_text()
        } else {
            Err(self.syntax_error("text or an annotation"))
        }
    }
}

impl Compiler for LolCompiler {
    ///Handle compilation errors by printing a message and exiting
    fn compile(&mut self, source: &str) {
        if let Err(message) = self.compile_source(source) {
            eprintln!("{}", message);
            process::exit(1);
        }
    }

    ///Gets the next token from the lexical analyzer
    fn next_token(&mut self) -> String {
        if let Err(message) = self.advance() {
            eprintln!("{}", message);
            process::exit(1);
        }
        self.current.clone()
    }

    ///Runs the syntax analyzer starting from `<lolcode>`
    fn parse(&mut self) {
        if let Err(message) = self.parse_lolcode() {
            eprintln!("{}", message);
            process::exit(1);
        }
    }

    ///Gets the current token being processed
    fn current_token(&self) -> String {
        self.current.clone()
    }

    ///Sets the current token
    fn set_current_token(&mut self, tok: String) {
        self.current = tok;
    }
}

impl SyntaxAnalyzer for LolCompiler {
    ///Parses the entire lolcode document starting from `<lolcode>`
    fn parse_lolcode(&mut self) -> Result<(), String> {
        if self.current.is_empty() {
            self.advance()?;
        }
        self.expect("#HAI")?;
        self.parse_comments()?;
        if self.is("#MAEK") && self.next_is("HEAD")? {
            self.parse_head()?;
            self.parse_comments()?;
        }
        self.parse_optional_define()?;
        self.parse_body()?;
        self.expect("#KBYE")?;
        if !self.current.is_empty() {
            return Err(self.syntax_error("nothing after #KBYE"));
        }
        Ok(())
    }

    ///Parses the `<head>` section of the document
    fn parse_head(&mut self) -> Result<(), String> {
        self.expect_pair("#MAEK", "HEAD")?;
        self.parse_comments()?;
        self.parse_title()?;
        self.parse_comments()?;
        self.expect("#MKAY")
    }

    ///Parses the `<title>` section of the document
    fn parse_title(&mut self) -> Result<(), String> {
        self.expect_pair("#GIMMEH", "TITLE")?;
        while self.is_text() {
            self.parse_text()?;
        }
        self.expect("#OIC")
    }

    ///Parses a comment block
    fn parse_comment(&mut self) -> Result<(), String> {
        self.expect("#OBTW")?;
        while self.is_text() {
            self.parse_text()?;
        }
        self.expect("#TLDR")
    }

    ///Parses the body of the document
    fn parse_body(&mut self) -> Result<(), String> {
        while !self.is("#KBYE") && !self.current.is_empty() {
            self.parse_inner_body()?;
        }
        Ok(())
    }

    ///Parses a paragraph section
    fn parse_paragraph(&mut self) -> Result<(), String> {
        self.expect_pair("#MAEK", "PARAGRAF")?;
        self.parse_optional_define()?;
        self.parse_inner_paragraph()?;
        self.expect("#MKAY")
    }

    ///Parses the inner content of a paragraph
    fn parse_inner_paragraph(&mut self) -> Result<(), String> {
        while !self.is("#MKAY") && !self.current.is_empty() {
            self.parse_inner_text()?;
        }
        Ok(())
    }

    ///Parses the inner text content of a paragraph
    fn parse_inner_text(&mut self) -> Result<(), String> {
        if self.is("#MAEK") {
            if self.next_is("LIST")? {
                self.parse_list()
            } else {
                self.advance()?;
                Err(self.syntax_error("LIST (paragraphs cannot go inside paragraphs)"))
            }
        } else if self.is("#GIMMEH") {
            self.parse_gimmeh()
        } else if self.is("#OBTW") {
            self.parse_comment()
        } else if self.is("#NEWLINE") {
            self.parse_newline()
        } else if self.is_text() {
            self.parse_text()
        } else {
            Err(self.syntax_error("paragraph text or #MKAY"))
        }
    }

    ///Parses a variable definition statement
    fn parse_variable_define(&mut self) -> Result<(), String> {
        self.expect("#IHAZ")?;
        if !self.is_text() {
            return Err(self.syntax_error("a variable name"));
        }
        self.parse_text()?;
        self.expect("#ITIZ")?;
        if !self.is_text() {
            return Err(self.syntax_error("a variable value"));
        }
        while self.is_text() {
            self.parse_text()?;
        }
        self.expect("#MKAY")
    }

    ///Parses a variable use statement
    fn parse_variable_use(&mut self) -> Result<(), String> {
        self.expect_pair("#GIMMEH", "VAR")?;
        if !self.is_text() {
            return Err(self.syntax_error("a variable name"));
        }
        self.parse_text()?;
        self.expect("#OIC")
    }

    ///Parses bold text
    fn parse_bold(&mut self) -> Result<(), String> {
        self.expect_pair("#GIMMEH", "BOLD")?;
        while !self.is("#OIC") {
            if self.is_text() {
                self.parse_text()?;
            } else if self.is("#GIMMEH") && self.next_is("VAR")? {
                self.parse_variable_use()?;
            } else {
                return Err(self.syntax_error("bold text or #OIC"));
            }
        }
        self.expect("#OIC")
    }

    ///Parses a list
    fn parse_list(&mut self) -> Result<(), String> {
        self.expect_pair("#MAEK", "LIST")?;
        self.parse_list_items()?;
        self.expect("#MKAY")
    }

    ///Parses the items within a list
    fn parse_list_items(&mut self) -> Result<(), String> {
        if !(self.is("#GIMMEH") && self.next_is("ITEM")?) {
            return Err(self.syntax_error("#GIMMEH ITEM"));
        }
        while self.is("#GIMMEH") && self.next_is("ITEM")? {
            self.expect_pair("#GIMMEH", "ITEM")?;
            self.parse_optional_define()?;
            self.parse_inner_list()?;
            self.expect("#OIC")?;
        }
        Ok(())
    }

    ///Parses the inner content of a list
    fn parse_inner_list(&mut self) -> Result<(), String> {
        while !self.is("#OIC") {
            if self.is("#GIMMEH") {
                self.parse_gimmeh()?;
            } else if self.is("#OBTW") {
                self.parse_comment()?;
            } else if self.is("#NEWLINE") {
                self.parse_newline()?;
            } else if self.is_text() {
                self.parse_text()?;
            } else {
                return Err(self.syntax_error("list item text or #OIC"));
            }
        }
        Ok(())
    }

    ///Parses a link
    fn parse_link(&mut self) -> Result<(), String> {
        self.expect_pair("#GIMMEH", "LINX")?;
        if !self.is_text() {
            return Err(self.syntax_error("a web address"));
        }
        self.parse_text()?;
        self.expect("#OIC")
    }

    ///Parses a #NEWLINE
    fn parse_newline(&mut self) -> Result<(), String> {
        self.expect("#NEWLINE")
    }

    ///Parses text content
    fn parse_text(&mut self) -> Result<(), String> {
        if !self.is_text() {
            return Err(self.syntax_error("text"));
        }
        let text = self.current.clone();
        self.add_to_tree(text);
        self.advance()
    }
}
