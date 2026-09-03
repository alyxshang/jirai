/*
Jirai by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the I/O
/// namespace to implement
/// the "Error" trait for
/// "JiraiErr". This
/// implementation is
/// only active if the
/// "cli" features is
/// enabled.
use std::io;

/// Importing the "YueErr"
/// structure to implement
/// the "From" trait for
/// "JiraiErr". This
/// implementation is
/// only active if the
/// "cli" features is
/// enabled.
#[cfg(feature="cli")]
use yue::YueErr;

/// Importing the "Result"
/// type because it is needed
/// by the "Display" trait.
use std::fmt::Result;

/// Importing the "Display"
/// trait to implement for
/// the error structure.
use std::fmt::Display;

/// Importing the "Error"
/// trait to implement it
/// for the error structure.
use std::error::Error;

/// Importing the "Formatter"
/// entity because it is needed
/// by the "Display" trait.
use std::fmt::Formatter;

/// A data structure to
/// store information about
/// errors.
#[derive(Clone,Eq,PartialEq, Debug)]
pub struct JiraiErr {
    pub details: String
}

/// Implementing
/// function(s) for
/// the `JiraiErr`
/// structure.
impl JiraiErr {

    /// A function to create
    /// and return a new
    /// instance of the `JiraiErr`
    /// structure.
    pub fn new(
        details: &str
    ) -> JiraiErr {
        JiraiErr {
            details: details.to_owned()
        }
    }

}

/// Implementing the `Error`
/// trait for the `JiraiErr`
/// structure.
impl Error for JiraiErr {

    /// The function that
    /// implements the `Error`
    /// trait for the `JiraiErr`
    /// structure.
    fn description(
        &self
    ) -> &str {
        &self.details
    }
}

/// Implementing the `Display`
/// trait for the `JiraiErr`
/// structure.
impl Display for JiraiErr {

    /// The function that
    /// implements the `Display`
    /// trait for the `JiraiErr`
    /// structure.
    fn fmt(
        &self, 
        f: &mut Formatter
    ) -> Result {
        write!(f,"{}",self.details)
    }
}

/// Implementing the `From`
/// trait to "convert" the `io::Error`
/// structure to the `JiraiErr`
/// structure.
impl From<io::Error> for JiraiErr{
    fn from(error: io::Error) -> Self {
        JiraiErr::new(&format!("I/O error: {:?}", error.to_string()))
    }
}

/// Implementing the `From`
/// trait to "convert" the `YueErr`
/// structure to the `JiraiErr`
/// structure.
#[cfg(feature="cli")]
impl From<YueErr> for JiraiErr{
    fn from(error: YueErr) -> Self {
        JiraiErr::new(&format!("CLI error: {:?}", error.to_string()))
    }
}
