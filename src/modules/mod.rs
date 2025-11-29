/*
Jirai by Alyx Shang.
Licensed under the FSL v1.
*/

/// Exporting the module
/// responsible for creating
/// an intermediate representation
/// from a Jirai abstract syntax
/// tree.
pub mod ir;

/// Exporting the module
/// containing the structure
/// to catch and handle errors.
pub mod err;

/// Exporting the module
/// containing entities 
/// to generate HTML code
/// from the AST received
/// from the Jirai parser.
pub mod html;

/// Exporting the module containing
/// entities to tokenize a string 
/// of Jirai source code.
pub mod lexer;

/// Declaring the module
/// containin this crate's
/// unit tests.
#[cfg(test)]
pub mod tests;

/// Exporting the module containing
/// entities to parse a stream of
/// tokens obtained from Jirai
/// source code.
pub mod parser;

/// Exporting the module
/// containing functions
/// to compile Jirai source
/// into other formats of
/// code.
pub mod compiler;
