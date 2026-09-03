/*
Jirai by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the Jirai
/// compiler's CLI function.
/// This is only imported if
/// the "cli" feature is active.
#[cfg(feature="cli")]
use jirai::cli;

/// The main point of entry
/// for the Rust compiler.
#[cfg(feature="cli")]
fn main(){
    match cli(){
        Ok(f) => println!("{}", f),
        Err(e) => eprintln!("{}", e)
    }
}
