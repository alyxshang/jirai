/*
Jirai by Alyx Shang.
Licensed under the FSL v1.
*/

/// Exporting the module
/// containing the structure
/// to catch and handle errors.
pub mod err;

/// Exporting the module containing
/// Jirai's compiler CLI. This module
/// is only active if the `cli` features
/// is active.
#[cfg(feature="cli")]
pub mod cli;

/// Exporting the module
/// containing a structure
/// to generate HTML code
/// from a parsed AST.
pub mod html;

/// Exporting the module
/// containing the function
/// to tokenize Jirai 
/// source code.
pub mod lexer;

/// Declaring the module
/// containing this crate's
/// unit tests.
#[cfg(test)]
pub mod tests;

/// Exporting the module
/// containing the structure
/// that parses a stream of
/// Jirai tokens.
pub mod parser;

/// Exporting the module
/// containing a function
/// to deserialize a string
/// obtained from an
/// Extended Jirai document
/// if the `ejirai` feature
/// is enabled.
#[cfg(feature="ejirai")]
pub mod ejirai;

/// Exporting the module
/// containing functions
/// to compile Jirai into
/// one or more formats.
pub mod compiler;
