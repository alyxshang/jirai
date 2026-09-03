/*
Jirai by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the
/// function to 
/// deserialize
/// a JMU string.
use jmu::from_str;

/// Importing the data
/// structure to catch
/// and handle errors.
use super::err::JiraiErr;

/// Importing the function
/// to compile a Jirai string
/// into a HTML string.
use super::compiler::to_html;

/// Importing the trait
/// that constrains which structures
/// can be used for deserializing the
/// JMU code inside a string obtained
/// from an Extended Jirai document.
use serde::de::DeserializeOwned;

/// A structure to encapsulate
/// the data deserialized from
/// a string obtained from an
/// Extended Jirai document.
#[derive(Debug)]
pub struct ExtendedJirai<T: DeserializeOwned>{
    pub data: T,
    pub code: Option<String>
}

/// A function to deserialize data
/// from a string obtained from an
/// Extended Jirai document. If the
/// operation is successful, an
/// instance is of the `ExtendedJirai<T>`
/// structure is returned. `T` is the
/// structure that implements 
/// `DeserializeOwned` via the 
/// `Deserialize` trait from the 
/// `serde` crate. If the operation
/// fails, an error is returned.
pub fn from_ejirai<T: DeserializeOwned>(
    sub: &str
) -> Result<ExtendedJirai<T>, JiraiErr>{
    let parts: Vec<&str> = sub
        .split("(^-^)")
        .collect();
    let mut result: Vec<String> = Vec::new();
    for part in parts.clone(){
        if !part.trim().is_empty(){
            result.push(part.to_string());
        }
    }
    if result.len() == 2{
        let data_str: String = match result.first(){
            Some(data_str) => data_str.to_string(),
            None => return Err::<ExtendedJirai<T>, JiraiErr>(
                JiraiErr::new("Could not get data string.")
            )
        };
        let jirai_str: String = match result.get(1){
            Some(jirai_str) => jirai_str.to_string(),
            None => return Err::<ExtendedJirai<T>, JiraiErr>(
                JiraiErr::new("Could not get Jirai string.")
            )
        };
        let data: T = match from_str(&data_str){
            Ok(data) => data,
            Err(e) => return Err::<ExtendedJirai<T>, JiraiErr>(
                JiraiErr::new(&e.to_string())
            )
        };
        let html: String = to_html(&jirai_str)?; 
        let result: ExtendedJirai<T> = ExtendedJirai{
            data: data,
            code: Some(html)
        };
        Ok(result)
    }
    else if result.len() == 1{
        let data_str: String = match result.first(){
            Some(data_str) => data_str.to_string(),
            None => return Err::<ExtendedJirai<T>, JiraiErr>(
                JiraiErr::new("Could not get data string.")
            )
        };
        let data: T = match from_str(&data_str){
            Ok(data) => data,
            Err(e) => return Err::<ExtendedJirai<T>, JiraiErr>(
                JiraiErr::new(&e.to_string())
            )
        };
        let result: ExtendedJirai<T> = ExtendedJirai{
            data: data,
            code: None
        };
        Ok(result)
    }
    else {
        Err::<ExtendedJirai<T>, JiraiErr>(
            JiraiErr::new("Could not deserialize string.")
        )
    }
}
