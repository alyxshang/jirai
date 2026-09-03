/*
Jirai by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the structure
/// to work with file paths
/// in a cross-platform way.
use std::path::PathBuf;

/// Importing the "Deserialize"
/// macro to use with a testing
/// structure for the "ejirai"
/// feature.
#[cfg(feature="ejirai")]
use serde::Deserialize;

/// Importing the structure
/// encapsulating data about
/// a captured Jirai token.
use super::lexer::Token;

/// Importing the structure
/// containing functions to
/// parse a stream of Jirai
/// tokens and test it.
use super::parser::Parser;

/// Importing the function
/// that tokenizes Jirai 
/// source code to test it.
use super::lexer::tokenize;

/// Importing the function
/// that generates HTML
/// from Jirai source code
/// to test it.
use super::compiler::to_html;

/// Importing the function to
/// lint a string of Jirai source
/// code.
use super::compiler::lint_str;

/// Importing the structure
/// containing functions
/// to recursively generate HTML
/// from an AST to test it.
use super::html::HTMLGenerator;

/// Importing the function to lint
/// a string of Jirai source code
/// obtained from a Jirai file.
use super::compiler::lint_file;

/// Importing the function to
/// deserialize a string of
/// Extended Jirai if the
/// feature is active.
#[cfg(feature="ejirai")]
use super::ejirai::from_ejirai;

/// Importing the structure to
/// encapsulate deserialized
/// data obtained from a 
/// string of Extended Jirai 
/// if the feature is active.
#[cfg(feature="ejirai")]
use super::ejirai::ExtendedJirai;

/// Importing the enumeration that
/// describes every possible type
/// of block statement in Jirai
/// markup.
use super::parser::BlockStatement;

/// Importing the function to compile
/// a Jirai file into an HTML file that
/// has the same name prefix as the Jirai
/// file.
use super::compiler::compile_file_auto;

/// Importing the function to compile
/// a Jirai file into an HTML file that
/// has a custom name prefix.
use super::compiler::compile_file_custom;

/// A test string of Extended Jirai
/// markup with content.
#[cfg(feature="ejirai")]
const EJIRAI_WC: &'static str = r#"
(^-^)
"layout" >~< "page"
"title" >~< "Page Title"
(^-^)
<3 Heading 1
# This is a comment.
This is normal text with a very long sentence.
This is a sentence containing *italic* and _bold text_.
This is a link to an image >($[alt][https://alyxshang.boo])<.
This text contains some _italic text_.
This text contains some *bold text*.
(^-^)
"#;

/// A test string of Extended Jirai
/// markup without content.
#[cfg(feature="ejirai")]
const EJIRAI_NC: &'static str = r#"
(^-^)
"layout" >~< "page"
"title" >~< "Page Title"
(^-^)
(^-^)
"#;


/// A test string of Jirai markup
/// code.
const CONTENTS: &'static str = r#"
<3 Heading 1
# This is a comment.
This is normal text with a very long sentence.
This is a sentence containing *italic* and _bold text_.
This is a link to an image >($[alt][https://alyxshang.boo])<.
This text contains some _italic text_.
This text contains some *bold text*.

<3<3 Heading 2

This paragraph contains some _nested *italic* text_.
And this is a >(%[link to my site][link to my site][https://alyxshang.boo])<.

~ This is item one of an unordered list.
~ This is item two of an unordered list.
"#;

/// The function to test the
/// "tokenizer" function.
#[test]
pub fn test_tokenizer(){ 
    let tokenized: Vec<Token> = tokenize(&CONTENTS.to_string())
        .expect("Could not tokenize file.");
    assert_eq!(tokenized.len(), 84);
}


/// The function to test the
/// "Parser" structure.
#[test]
pub fn test_parser(){ 
    let tokenized: Vec<Token> = tokenize(&CONTENTS.to_string())
        .expect("Could not tokenize file.");
    let mut parser: Parser = Parser::new(&tokenized)
        .expect("Could not construct parser.");
    let parsed: Vec<BlockStatement> = parser.parse()
        .expect("Could not parse statements.");
    assert_eq!(parsed.len(), 11);
}

// The function to test the
// linting function.
#[test]
pub fn test_linter(){
    let linted: bool = lint_str(&CONTENTS.to_string())
        .expect("Failed to lint string.");
    assert_eq!(linted, true);
}

/// The function to test the
/// "HTMLGenerator" structure.
#[test]
pub fn test_html_gen(){ 
    let tokenized: Vec<Token> = tokenize(&CONTENTS.to_string())
        .expect("Could not tokenize file.");
    let mut parser: Parser = Parser::new(&tokenized)
        .expect("Could not construct parser.");
    let parsed: Vec<BlockStatement> = parser.parse()
        .expect("Could not parse statements.");
    let mut generator: HTMLGenerator = HTMLGenerator::new(&parsed)
        .expect("Could not construct generator.");
    let code: String = generator.generate()
        .expect("Could not generate code.");
    assert_eq!(code.len(), 639);
}

/// The function to test the "to_html"
/// function.
#[test]
pub fn test_compiler(){ 
    let code: String = to_html(&CONTENTS.to_string())
        .expect("Could not compile code.");
    assert_eq!(code.len(), 639);
}

/// A data structure for testing
/// the deserialization of an
/// Extended Jirai string.
#[cfg(feature="ejirai")]
#[derive(Deserialize, Debug)]
pub struct PageData{
    pub layout: String,
    pub title: String
}

/// Testing the "from_ejirai" function with content.
#[cfg(feature="ejirai")]
#[test]
pub fn test_ejirai_deserializer_content(){
    let result: ExtendedJirai<PageData> = from_ejirai(
        &EJIRAI_WC.to_string()
    ).expect("Could not deserialize document.");
    assert_eq!(result.data.layout, "page".to_string());
    assert_eq!(result.code.is_some(), true);
}

/// Testing the "from_ejirai" function without content.
#[cfg(feature="ejirai")]
#[test]
pub fn test_ejirai_deserializer_no_content(){
    let result: ExtendedJirai<PageData> = from_ejirai(
        &EJIRAI_NC.to_string()
    ).expect("Could not deserialize document.");
    assert_eq!(result.data.layout, "page".to_string());
    assert_eq!(result.code.is_none(), true);
}

/// Testing the "compile_file_custom" function.
#[test]
pub fn test_compile_file_custom(){
    let mut file_buf: PathBuf = PathBuf::from(
        env!("CARGO_MANIFEST_DIR")
    );
    file_buf.push("sample");
    file_buf.push("sample.jirai");
    let mut compiled_file: PathBuf = PathBuf::from(
        env!("CARGO_MANIFEST_DIR")
    );
    compiled_file.push("sample");
    compiled_file.push("sample_compiled.html");
    let _compile_op: () = compile_file_custom(
        &file_buf.display().to_string(),
        &compiled_file.display().to_string()
    ).expect("Could not compile file.");
    assert_eq!(compiled_file.exists(), true);
}

/// Testing the "compile_file_auto" function.
#[test]
pub fn test_compile_file_auto(){
    let mut file_buf: PathBuf = PathBuf::from(
        env!("CARGO_MANIFEST_DIR")
    );
    file_buf.push("sample");
    file_buf.push("sample.jirai");
    let mut compiled_file: PathBuf = PathBuf::from(
        env!("CARGO_MANIFEST_DIR")
    );
    compiled_file.push("sample");
    compiled_file.push("sample.html");
    let _compile_op: () = compile_file_auto(
        &file_buf.display().to_string()
    ).expect("Could not compile file.");
    assert_eq!(compiled_file.exists(), true);
}

/// Testing the "compile_file" function.
pub fn test_compile_file(){
    let mut file_buf: PathBuf = PathBuf::from(
        env!("CARGO_MANIFEST_DIR")
    );
    file_buf.push("sample");
    file_buf.push("sample.jirai");
    let mut compiled_file: PathBuf = PathBuf::from(
        env!("CARGO_MANIFEST_DIR")
    );
    compiled_file.push("sample");
    compiled_file.push("sample_12.html");
    let _compile_op: () = compile_file_auto(
        &file_buf.display().to_string()
    ).expect("Could not compile file.");
    assert_eq!(compiled_file.exists(), true);

}

/// Testing the "lint_file" function.
#[test]
pub fn test_lint_file(){
    let mut file_buf: PathBuf = PathBuf::from(
        env!("CARGO_MANIFEST_DIR")
    );
    file_buf.push("sample");
    file_buf.push("sample.jirai"); 
    let lint_op: bool = lint_file(
        &file_buf.display().to_string()
    ).expect("Could not lint file.");
    assert_eq!(lint_op, true);
}
