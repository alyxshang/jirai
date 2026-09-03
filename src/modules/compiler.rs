/*
Jirai by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the data 
/// structure to work
/// with files in a 
/// cross-platform
/// way.
use std::fs::File;

/// Importing the "Write"
/// trait to write to 
/// files.
use std::io::Write;

/// Importing the data
/// structure for working
/// with files in a 
/// cross-platform way.
use std::path::PathBuf;

/// Importing the structure
/// encapsulating data about
/// a captured token for 
/// explicit typing.
use super::lexer::Token;

/// Importing the data
/// structure to catch
/// and handle errors.
use super::err::JiraiErr;

/// Importing the structure
/// that parses a stream of
/// tokens obtained from Jirai
/// source code into a syntax
/// tree.
use super::parser::Parser;

/// Importing the function that
/// tokenizes Jirai source code
/// into a stream of tokens.
use super::lexer::tokenize;

/// Importing the function 
/// to read the contents
/// of a file as a string.
use std::fs::read_to_string;

/// Importing the structure that
/// contains functions to generate
/// HTML code from an AST parsed
/// from Jirai source code.
use super::html::HTMLGenerator;

/// An enumeration that lists all
/// possible top-level block statements
/// Jirai source code can have.
use super::parser::BlockStatement;

/// A function to accept a string slice
/// of Jirai source code and compile this
/// source code into HTML. If the operation
/// is successful, this string is returned.
/// If the operation fails, an error is 
/// returned.
pub fn to_html(
    sub: &str
) -> Result<String, JiraiErr>{
    let tokens: Vec<Token> = tokenize(sub)?;
    let mut parser: Parser = Parser::new(&tokens)?;
    let parsed: Vec<BlockStatement> = parser.parse()?;
    let mut generator: HTMLGenerator = HTMLGenerator::new(&parsed)?;
    let html: String = generator.generate()?;
    Ok(html)
}

/// A function to accept a string slice
/// of Jirai source code and check whether
/// this string can be parsed. If the operation
/// is successful, a boolean is returned. If the
/// operation fails, an error is returned.
pub fn lint_str(
    sub: &str
) -> Result<bool, JiraiErr>{
    let tokens: Vec<Token> = tokenize(sub)?;
    let mut parser: Parser = Parser::new(&tokens)?;
    let parsed: Result<Vec<BlockStatement>, JiraiErr> = parser
        .parse();
    Ok(parsed.is_ok())
}

/// A function to compile
/// a Jirai file into an HTML
/// file that has the same name
/// as the input file. If the 
/// operation is successful, nothing
/// is returned. If the operation
/// fails, an error is returned.
pub fn compile_file_auto(
    sub: &str,
) -> Result<(), JiraiErr>{
    let file_buf: PathBuf = PathBuf::from(sub);
    let contents: String = read_to_string(&file_buf)?;
    let html: String = to_html(&contents)?;
    let mut parent: PathBuf = match file_buf.parent(){
        Some(parent) => parent.to_path_buf(),
        None => return Err::<(), JiraiErr>(
            JiraiErr::new("Could not get parent directory.")
        )
    };
    let file_stem_os_str: Option<&str> = match file_buf.file_stem(){
        Some(file_stem_os_str) => file_stem_os_str.to_str(),
        None => return Err::<(), JiraiErr>(
            JiraiErr::new("Could not get file stem format.")
        )
    };
    let file_stem: String = match file_stem_os_str{
        Some(file_stem) => file_stem.to_string(),
        None => return Err::<(), JiraiErr>(
            JiraiErr::new("Could not get file stem.")
        )
    };
    let html_file_name: String = format!("{}.html", file_stem);
    parent.push(html_file_name);
    if parent.exists(){
        Err::<(), JiraiErr>(
            JiraiErr::new(
                &format!(
                    "The path \"{}\" already exists.",
                    parent.display()
                )
            )
        )
    }
    else{
        let mut html_file: File = File::create(&parent)?;
        let w_op: () = html_file.write_all(html.as_bytes())?;
        Ok(w_op)
    }
}

/// A function to compile
/// a Jirai file into an HTML
/// file that has a custom name
/// as the input file. If the 
/// operation is successful, nothing
/// is returned. If the operation
/// fails, an error is returned.
pub fn compile_file_custom(
    sub: &str,
    out: &str
) -> Result<(), JiraiErr>{
    let file_buf: PathBuf = PathBuf::from(sub);
    let contents: String = read_to_string(&file_buf)?;
    let html: String = to_html(&contents)?;
    let out_buf: PathBuf = PathBuf::from(out);
    if out_buf.exists(){
        Err::<(), JiraiErr>(
            JiraiErr::new(
                &format!(
                    "The path \"{}\" already exists.",
                    out_buf.display()
                )
            )
        )
    }
    else {
        let mut html_file: File = File::create(out)?;
        let w_op: () = html_file.write_all(html.as_bytes())?;
        Ok(w_op)
    }
}

/// A function to compile
/// a Jirai file into an HTML
/// file. If the operation is 
/// successful, nothing is 
/// returned. If the operation
/// fails, an error is returned.
pub fn compile_file(
    sub: &str,
    out: &Option<&str>
) -> Result<(), JiraiErr>{
    match out {
        Some(c_out) => Ok(compile_file_custom(sub, c_out)?),
        None => Ok(compile_file_auto(sub)?)
    }
}

/// A function to lint a file containing
/// Jirai source code. If the operation is
/// successful, a boolean is returned.
/// If the operation fails, an error 
/// is returned.
pub fn lint_file(
    sub: &str
) -> Result<bool, JiraiErr>{
    let file_buf: PathBuf = PathBuf::from(sub);
    let contents: String = read_to_string(&file_buf)?;
    let linted: bool = lint_str(&contents)?;
    Ok(linted)
}
