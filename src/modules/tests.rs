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
        let source: String = "<3<3 _Second Heading_\nLorem ipsum sit dolor amet. <this is code> Lorem ipsum sit dolor amet.\n"
        .to_string();
    let tokens: Vec<Token> = tokenize_string(&source)
        .expect("Error tokenizing heading string.");
    let expected: Vec<Token> = vec![
    ];
    for token in tokens {
        println!("{:?}", token);
    }
    //assert_eq!(expectd, tokens);
}

/// A function to test the
/// Jirai parser.
#[test]
pub fn test_parser(){
    let source: String = "<3<3 _Second Heading_\nLorem ipsum sit dolor amet. <this is code> Lorem ipsum sit dolor amet.\n"
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
