/*
Jirai by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the structure
/// to model paths on disk.
use std::path::PathBuf;

/// Importing the structure
/// holding information on
/// a capture token.
use super::lexer::Token;

/// Importing the structure
/// responsible for parsing
/// a stream of tokens.
use super::parser::Parser;

/// Importing the data structure
/// to encapsulate data about the
/// line and column information
/// of a token.
use super::lexer::Position;

/// Importing the enumeration
/// containing all possible types
/// Jirai tokens.
use super::lexer::TokenType;

/// Importing the function to
/// read the contents of a file
/// from disk as a string.
use std::fs::read_to_string;

/// Importing the enumeration
/// describing all possible types
/// of block-level statements
/// Jirai source code can contain.
use super::parser::Statement;

/// Importing the function to
/// generate HTML code from
/// Jirai source code.
use super::compiler::to_html;

/// Importing the enum
/// to sepcify which type
/// of source string was 
/// received.
use super::parser::SourceType;

/// Importing the function to
/// tokenize a string of Jirai
/// source code.
use super::lexer::tokenize_string;

/// Importing the structure that
/// takes an AST from parsed Jirai
/// source code and generates HTML
/// code from this.
use super::html::HTMLCodeGenerator;

/// A function to test the 
/// Jirai tokenizer.
#[test]
pub fn test_lexer(){
    let mut example_home: PathBuf = PathBuf::new();
    example_home.push(env!("CARGO_MANIFEST_DIR"));
    example_home.push("example/example.jirai");
    let sample_code: String = read_to_string(&example_home.as_path())
        .expect("Could not read file contents.");
    let tokens: Vec<Token> = tokenize_string(&sample_code)
        .expect("Could not tokenize sample string.");
    let expected: Vec<Token> = vec![
        Token::new(
            &Position::new(&0,&5),
            &Position::new(&0,&0),
            &TokenType::DocumentLimiter,
            &None),
        Token::new(
            &Position::new(&0,&6),
            &Position::new(&0,&5),
            &TokenType::NewLine,
            &None), 
        Token::new(
            &Position::new(&1,&8),
            &Position::new(&1,&6),
            &TokenType::HeadingMarker,
            &None), 
        Token::new(
            &Position::new(&1,&9),
            &Position::new(&1,&8),
            &TokenType::UserString,
            &Some(" ".to_string())), 
        Token::new(
            &Position::new(&1,&10),
            &Position::new(&1,&9),
            &TokenType::ItalicText,
            &None), 
        Token::new(
            &Position::new(&1,&19),
            &Position::new(&1,&10),
            &TokenType::UserString,
            &Some("Heading I".to_string())), 
        Token::new(
            &Position::new(&1,&20),
            &Position::new(&1,&19),
            &TokenType::ItalicText,
            &None), 
        Token::new(
            &Position::new(&1,&21),
            &Position::new(&1,&20),
            &TokenType::NewLine,
            &None), 
        Token::new(
            &Position::new(&2,&76),
            &Position::new(&2,&21),
            &TokenType::UserString,
            &Some("Lorem ipsum sit dolor amet. Lorem ipsum sit dolor amet.".to_string())), 
        Token::new(
            &Position::new(&2,&77),
            &Position::new(&2,&76),
            &TokenType::NewLine,
            &None), 
        Token::new(
            &Position::new(&3,&78),
            &Position::new(&3,&77),
            &TokenType::NewLine,
            &None), 
        Token::new(
            &Position::new(&4,&80),
            &Position::new(&4,&78),
            &TokenType::HeadingMarker,
            &None), 
        Token::new(
            &Position::new(&4,&82),
            &Position::new(&4,&80),
            &TokenType::HeadingMarker,
            &None), 
        Token::new(
            &Position::new(&4,&83),
            &Position::new(&4,&82),
            &TokenType::UserString,
            &Some(" ".to_string())), 
        Token::new(
            &Position::new(&4,&84),
            &Position::new(&4,&83),
            &TokenType::BoldText,
            &None), 
        Token::new(
            &Position::new(&4,&94),
            &Position::new(&4,&84),
            &TokenType::UserString,
            &Some("Heading II".to_string())), 
        Token::new(
            &Position::new(&4,&95),
            &Position::new(&4,&94),
            &TokenType::BoldText,
            &None), 
        Token::new(
            &Position::new(&4,&96),
            &Position::new(&4,&95),
            &TokenType::NewLine,
            &None), 
        Token::new(
            &Position::new(&5,&161),
            &Position::new(&5,&96),
            &TokenType::UserString,
            &Some("Lorem ipsum sit dolor amet. Lorem ipsum sit dolor amet. This text".to_string())), 
        Token::new(
            &Position::new(&5,&162),
            &Position::new(&5,&161),
            &TokenType::NewLine,
            &None), 
        Token::new(
            &Position::new(&6,&173),
            &Position::new(&6,&162),
            &TokenType::UserString,
            &Some("contains a ".to_string())), 
        Token::new(
            &Position::new(&6,&174),
            &Position::new(&6,&173),
            &TokenType::OpenCurly,
            &None), 
        Token::new(
            &Position::new(&6,&175),
            &Position::new(&6,&174),
            &TokenType::LinkMarker,
            &None), 
        Token::new(
            &Position::new(&6,&176),
            &Position::new(&6,&175),
            &TokenType::OpenSquare,
            &None), 
        Token::new(
            &Position::new(&6,&180),
            &Position::new(&6,&176),
            &TokenType::UserString,
            &Some("link".to_string())), 
        Token::new(
            &Position::new(&6,&181),
            &Position::new(&6,&180),
            &TokenType::CloseSquare,
            &None), 
        Token::new(
            &Position::new(&6,&182),
            &Position::new(&6,&181),
            &TokenType::OpenSquare,
            &None), 
        Token::new(
            &Position::new(&6,&203),
            &Position::new(&6,&182),
            &TokenType::UserString, 
            &Some("https://alyxshang.boo".to_string())), 
        Token::new(
            &Position::new(&6,&204),
            &Position::new(&6,&203),
            &TokenType::CloseSquare,
            &None), 
        Token::new(
            &Position::new(&6,&205),
            &Position::new(&6,&204),
            &TokenType::CloseCurly,
            &None), 
        Token::new(
            &Position::new(&6,&206),
            &Position::new(&6,&205),
            &TokenType::UserString,
            &Some(".".to_string())), 
        Token::new(
            &Position::new(&6,&207),
            &Position::new(&6,&206),
            &TokenType::NewLine,
            &None), 
        Token::new(
            &Position::new(&7,&208),
            &Position::new(&7,&207),
            &TokenType::NewLine,
            &None), 
        Token::new(
            &Position::new(&8,&210),
            &Position::new(&8,&208),
            &TokenType::HeadingMarker,
            &None), 
        Token::new(
            &Position::new(&8,&212),
            &Position::new(&8,&210),
            &TokenType::HeadingMarker,
            &None), 
        Token::new(
            &Position::new(&8,&214),
            &Position::new(&8,&212),
            &TokenType::HeadingMarker,
            &None), 
        Token::new(
            &Position::new(&8,&215),
            &Position::new(&8,&214),
            &TokenType::UserString,
            &Some(" ".to_string())), 
        Token::new(
            &Position::new(&8,&216),
            &Position::new(&8,&215),
            &TokenType::ItalicText,
            &None), 
        Token::new(
            &Position::new(&8,&227),
            &Position::new(&8,&216),
            &TokenType::UserString,
            &Some("Heading III".to_string())), 
        Token::new(
            &Position::new(&8,&228),
            &Position::new(&8,&227),
            &TokenType::ItalicText,
            &None), 
        Token::new(
            &Position::new(&8,&229),
            &Position::new(&8,&228),
            &TokenType::NewLine,
            &None), 
        Token::new(
            &Position::new(&9,&268),
            &Position::new(&9,&229),
            &TokenType::UserString,
            &Some("This paragraph contains my profile pic.".to_string())), 
        Token::new(
            &Position::new(&9,&269),
            &Position::new(&9,&268),
            &TokenType::NewLine,
            &None), 
        Token::new(
            &Position::new(&10,&270),
            &Position::new(&10,&269),
            &TokenType::OpenCurly,
            &None), 
        Token::new(
            &Position::new(&10,&271),
            &Position::new(&10,&270),
            &TokenType::ImageMarker,
            &None), 
        Token::new(
            &Position::new(&10,&272),
            &Position::new(&10,&271),
            &TokenType::OpenSquare,
            &None), 
        Token::new(
            &Position::new(&10,&278),
            &Position::new(&10,&272),
            &TokenType::UserString,
            &Some("my pfp".to_string())), 
        Token::new(
            &Position::new(&10,&279),
            &Position::new(&10,&278),
            &TokenType::CloseSquare,
            &None), 
        Token::new(
            &Position::new(&10,&280),
            &Position::new(&10,&279),
            &TokenType::OpenSquare,
            &None), 
        Token::new(
            &Position::new(&10,&333),
            &Position::new(&10,&280),
            &TokenType::UserString,
            &Some("https://avatars.githubusercontent.com/u/179976644?v=4".to_string())), 
        Token::new(
            &Position::new(&10,&334),
            &Position::new(&10,&333),
            &TokenType::CloseSquare,
            &None), 
        Token::new(
            &Position::new(&10,&335),
            &Position::new(&10,&334),
            &TokenType::CloseCurly,
            &None), 
        Token::new(
            &Position::new(&10,&336),
            &Position::new(&10,&335),
            &TokenType::NewLine,
            &None), 
        Token::new(
            &Position::new(&11,&337),
            &Position::new(&11,&336),
            &TokenType::NewLine,
            &None), 
        Token::new(
            &Position::new(&12,&339),
            &Position::new(&12,&337),
            &TokenType::HeadingMarker,
            &None), 
        Token::new(
            &Position::new(&12,&341),
            &Position::new(&12,&339),
            &TokenType::HeadingMarker,
            &None), 
        Token::new(
            &Position::new(&12,&352),
            &Position::new(&12,&341),
            &TokenType::UserString,
            &Some(" Heading IV".to_string())), 
        Token::new(
            &Position::new(&12,&353),
            &Position::new(&12,&352),
            &TokenType::NewLine,
            &None), 
        Token::new(
            &Position::new(&13,&354),
            &Position::new(&13,&353),
            &TokenType::NewLine,
            &None), 
        Token::new(
            &Position::new(&14,&369),
            &Position::new(&14,&354),
            &TokenType::UserString,
            &Some("This is a list!".to_string())), 
        Token::new(
            &Position::new(&14,&370),
            &Position::new(&14,&369),
            &TokenType::NewLine, 
            &None), 
        Token::new(
            &Position::new(&15,&371),
            &Position::new(&15,&370),
            &TokenType::NewLine,
            &None), 
        Token::new(
            &Position::new(&16,&372),
            &Position::new(&16,&371),
            &TokenType::ListMarker,
            &None), 
        Token::new(
            &Position::new(&16,&385),
            &Position::new(&16,&372),
            &TokenType::UserString,
            &Some(" List item 1.".to_string())), 
        Token::new(
            &Position::new(&16,&386),
            &Position::new(&16,&385),
            &TokenType::NewLine,
            &None), 
        Token::new(
            &Position::new(&17,&387),
            &Position::new(&17,&386),
            &TokenType::ListMarker,
            &None), 
        Token::new(
            &Position::new(&17,&400),
            &Position::new(&17,&387),
            &TokenType::UserString,
            &Some(" List item 2.".to_string())), 
        Token::new(
            &Position::new(&17,&401),
            &Position::new(&17,&400),
            &TokenType::NewLine,
            &None), 
        Token::new(
            &Position::new(&18,&402),
            &Position::new(&18,&401),
            &TokenType::ListMarker,
            &None), 
        Token::new(
            &Position::new(&18,&415),
            &Position::new(&18,&402),
            &TokenType::UserString,
            &Some(" List item 3.".to_string())), 
        Token::new(
            &Position::new(&18,&416),
            &Position::new(&18,&415),
            &TokenType::NewLine,
            &None), 
        Token::new(
            &Position::new(&19,&421),
            &Position::new(&19,&416),
            &TokenType::DocumentLimiter,
            &None), 
        Token::new(
            &Position::new(&19,&422),
            &Position::new(&19,&421),
            &TokenType::NewLine,
            &None)
    ];
    assert_eq!(tokens, expected);
}

/// A function to test the
/// Jirai parser.
#[test]
pub fn test_parser(){
    let mut example_home: PathBuf = PathBuf::new();
    example_home.push(env!("CARGO_MANIFEST_DIR"));
    example_home.push("example/example.jirai");
    let sample_code: String = read_to_string(&example_home.as_path())
        .expect("Could not read file contents.");
    let tokens: Vec<Token> = tokenize_string(&sample_code)
        .expect("Could not tokenize sample string.");
    let mut parser: Parser = Parser::new(&SourceType::Document, &tokens)
        .expect("Could not create parser.");
    let statements: Vec<Statement> = parser.parse()
        .expect("Could not parse heading string.");
    for statement in statements{
        println!("{:?}", statement);
    }
}

/// The function to test the 
/// functionality of the 
/// `HTMLCodeGenerator` structure.
#[test]
pub fn test_html_generator(){
}

/// The function to test all functions
/// inside the compiler module that
/// compile Jirai source code into other
/// formats of code.
#[test]
pub fn test_compiler(){

}
