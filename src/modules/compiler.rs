/*
Jirai by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the structure
/// holding information on
/// a capture token.
use super::lexer::Token;

/// Importing the structure
/// to catch and handle errors.
use super::err::JiraiErr;

/// Importing the structure
/// responsible for parsing
/// a stream of tokens.
use super::parser::Parser;

/// Importing the enumeration
/// describing all possible types
/// of block-level statements
/// Jirai source code can contain.
use super::parser::Statement;

/// Importing the enumeration
/// describing all possible
/// types of Jirai source 
/// code.
use super::parser::SourceType;

/// Importing the function to
/// tokenize a string of Jirai
/// source code.
use super::lexer::tokenize_string;

/// Importing the structure holding
/// the parsed AST and that generates
/// HTML code from this AST.
use super::html::HTMLCodeGenerator;

/// A function to compile
/// Jirai source into
/// HTML code and return
/// that generated code.
/// If the operation fails,
/// an error is returned.
pub fn to_html(
    source: &str,
    minify: &bool,
    alt_enforcing: &bool,
    source_type: &SourceType
) -> Result<String, JiraiErr>{
    let tokens: Vec<Token> = tokenize_string(
        source
    )?;
    let mut parser: Parser = Parser::new(
        source_type, 
        &tokens
    )?;
    let parsed: Vec<Statement> = parser.parse()?;
    let mut code_generator: HTMLCodeGenerator = HTMLCodeGenerator::new(
        minify,
        alt_enforcing,
        &parsed
    )?;
    let generated: String = code_generator.generate()?;
    Ok(generated)
}
