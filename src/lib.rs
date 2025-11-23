/*
Jirai by Alyx Shang.
Licensed under the FSL v1.
*/

/// Declaring the "modules"
/// directory as a module.
pub mod modules;

/// Re-exporting the module
/// containing the structure
/// to catch and handle errors.
pub use modules::err::*;
 
/// Re-exporting the module containing
/// entities to tokenize a string 
/// of Jirai source code.
pub use modules::lexer::*;

/// Re-exporting the module containing
/// entities to parse a stream of
/// tokens obtained from Jirai
/// source code.
pub use modules::parser::*;
