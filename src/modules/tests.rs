/*
Jirai by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the structure
/// holding information on
/// a capture token.
use super::lexer::Token;

/// Importing the structure
/// responsible for parsing
/// a stream of tokens.
use super::parser::Parser;

/// Importing the structure
/// encapsulating data about
/// the position of a token
/// in a stream of characters.
use super::lexer::Position;

/// Importing the enumeration
/// containing all possible types
/// Jirai tokens.
use super::lexer::TokenType;

/// Importing the enumeration
/// describing all possible types
/// of block-level statements
/// Jirai source code can contain.
use super::parser::Statement;

/// Importing the function to
/// tokenize a string of Jirai
/// source code.
use super::lexer::tokenize_string;

/// A function to test the 
/// Jirai tokenizer.
#[test]
pub fn test_lexer(){
    let source: String = "<3<3 _Second Heading_"
        .to_string();
    let tokens: Vec<Token> = tokenize_string(&source)
        .expect("Error tokenizing heading string.");
   let expected: Vec<Token> = vec![
        Token::new(&Position::new(&0,&2),&Position::new(&0,&0),&TokenType::HeadingMarker,&None),
        Token::new(&Position::new(&0,&4),&Position::new(&0,&2),&TokenType::HeadingMarker,&None), 
        Token::new(&Position::new(&0,&5),&Position::new(&0,&4),&TokenType::UserString,&Some(" ".to_string())), 
        Token::new(&Position::new(&0,&6),&Position::new(&0,&5),&TokenType::ItalicText,&None), 
        Token::new(&Position::new(&0,&20),&Position::new(&0,&6),&TokenType::UserString,&Some("Second Heading".to_string())), 
        Token::new(&Position::new(&0,&21),&Position::new(&0,&20),&TokenType::ItalicText,&None)
    ];
    assert_eq!(tokens, expected);
}

/// A function to test the
/// Jirai parser.
#[test]
pub fn test_parser(){
    let source: String = "<3<3 _Second Heading_\n\nLorem _ipsum_ *sit* dolor amet. This is {![generic text][https://en.wikipedia.org/wiki/Lorem_ipsum]}!\n\n"
        .to_string();
    let tokens: Vec<Token> = tokenize_string(&source)
        .expect("Error tokenizing heading string.");
    let mut parser: Parser = Parser::new(&tokens);
    let statements: Vec<Statement> = parser.parse()
        .expect("Could not parse heading string.");
    for statement in statements{
        println!("{:?}", statement);
    }
}
