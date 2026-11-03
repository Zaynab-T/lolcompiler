//!unit tests

use crate::has_lol_extension;
use crate::lexer::Lexer;
use crate::parser::LolCompiler;
use crate::traits::{Compiler, LexicalAnalyzer, SyntaxAnalyzer};
use std::fs;

///compiles a test file and returns the html or an error message
fn compile_file(path: &str) -> Result<String, String> {
    let source = fs::read_to_string(path).unwrap();
    let mut compiler = LolCompiler::new(&source);
    compiler.compile_source(&source)?;
    Ok(compiler.html)
}

///checks the tokens come out in order
#[test]
fn test_next_token() {
    let mut compiler = LolCompiler::new("#HAI #maek PARAGRAF Hi there! #MKAY #KBYE");
    assert_eq!(compiler.next_token(), "#HAI");
    assert_eq!(compiler.next_token(), "#maek");
    assert_eq!(compiler.next_token(), "PARAGRAF");
    assert_eq!(compiler.next_token(), "Hi");
    assert_eq!(compiler.next_token(), "there!");
    assert_eq!(compiler.current_token(), "there!");
    assert_eq!(compiler.next_token(), "#MKAY");
    assert_eq!(compiler.next_token(), "#KBYE");
    assert_eq!(compiler.next_token(), "");
}

///checks setting the current token
#[test]
fn test_set_current_token() {
    let mut compiler = LolCompiler::new("");
    compiler.set_current_token(String::from("#OIC"));
    assert_eq!(compiler.current_token(), "#OIC");
}

///checks valid and invalid tokens
#[test]
fn test_lookup() {
    let lexer = Lexer::new("");
    assert_eq!(lexer.lookup("#HAI"), true);
    assert_eq!(lexer.lookup("#kbye"), true);
    assert_eq!(lexer.lookup("www.towson.edu"), true);
    assert_eq!(lexer.lookup("#HEY"), false);
    assert_eq!(lexer.lookup("<b>"), false);
}

///checks get_char
#[test]
fn test_get_char() {
    let mut lexer = Lexer::new("ab");
    assert_eq!(lexer.get_char(), 'a');
    assert_eq!(lexer.get_char(), 'b');
}

///checks valid and invalid parsing
#[test]
fn test_parse_lolcode() {
    let mut good = LolCompiler::new("#HAI #GIMMEH BOLD hi #OIC #KBYE");
    assert_eq!(good.parse_lolcode(), Ok(()));
    let mut bad = LolCompiler::new("#HAI #GIMMEH BOLD hi #KBYE");
    assert!(bad.parse_lolcode().is_err());
}

///checks that only .lol files are accepted
#[test]
fn test_extension() {
    assert_eq!(has_lol_extension("Test2.lol"), true);
    assert_eq!(has_lol_extension("Test1.txt"), false);
}

///runs the professor's test cases
#[test]
fn test_professor_cases() {
    assert!(compile_file("test/Test2.lol").unwrap().contains("<title> Test 2 </title>"));
    assert!(compile_file("test/Test3.lol").unwrap().contains("<b> test case 3 </b>"));
    assert!(compile_file("test/Test4.lol").unwrap_err().starts_with("Lexical error"));
    assert!(compile_file("test/Test5.lol").unwrap_err().starts_with("Syntax error"));
    assert!(compile_file("test/Test6.lol").unwrap().contains("is 42"));
    assert!(compile_file("test/Test7.lol").unwrap_err().starts_with("Static semantic error"));
    let test8 = compile_file("test/Test8.lol").unwrap();
    assert!(test8.contains("is 42") && test8.contains("really happiness"));
    let test9 = compile_file("test/Test9.lol").unwrap();
    assert!(test9.contains("is 42") && test9.contains("really happiness"));
    let test10 = compile_file("test/Test10.lol").unwrap();
    assert!(test10.contains("<a href=\"https://www.youtube.com/watch?v=aboZctrHfK8\">"));
}

///runs my own test cases
#[test]
fn test_my_cases() {
    let scope = compile_file("test/valid_scope.lol").unwrap();
    assert!(scope.contains("my name is Josh"));
    assert!(scope.contains("my name is Jon"));
    assert!(compile_file("test/valid_simpsons.lol").is_ok());
    assert!(compile_file("test/valid_variables.lol").unwrap().contains("Homer Simpson"));
    assert!(compile_file("test/err_semantic_out_of_scope.lol").is_err());
    assert!(compile_file("test/err_syntax_nested_paragraph.lol").is_err());
    assert!(compile_file("test/err_lexical_bad_annotation.lol").is_err());
}
