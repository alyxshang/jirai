/*
Jirai by Alyx Shang.
Licensed under the FSL v1.
*/

#![deny(clippy::all)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::borrowed_box)]
#![allow(clippy::if_same_then_else)]
#![allow(clippy::redundant_field_names)]

/// Declaring the "modules"
/// directory as a module.
pub mod modules;

/// Re-exporting the module
/// containing the structure
/// to catch and handle errors.
pub use modules::err::*;

/// Re-exporting the module containing
/// Jirai's compiler CLI. This module
/// is only active if the `cli` 
/// feature is active.
#[cfg(feature="cli")]
pub use modules::cli::*;

/// Re-exporting the module
/// containing a structure
/// to generate HTML code
/// from a parsed AST.
pub use modules::html::*;

/// Re-exporting the module
/// containing the function
/// to tokenize Jirai 
/// source code.
pub use modules::lexer::*;

/// Re-exporting the module
/// containing the structure
/// that parses a stream of
/// Jirai tokens.
pub use modules::parser::*;

/// Re-exporting the module
/// containing a function
/// to deserialize a string
/// obtained from an
/// Extended Jirai document
/// if the `ejirai` feature
/// is enabled.
#[cfg(feature="ejirai")]
pub use modules::ejirai::*;

/// Re-exporting the module
/// containing functions
/// to compile Jirai into
/// different formats.
pub use modules::compiler::*;
