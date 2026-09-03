/*
Jirai by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the data
/// structure to catch
/// and handle errors.
use super::err::JiraiErr;

/// An enumeration "listing"
/// all possible types of tokens
/// a Jirai document can contain.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType { 
    NewLine,
    Squiggly,
    BoldText,
    OpenAngle,
    LinkMarker,
    CloseAngle,
    ItalicText,
    OpenSquare,
    UserString,
    ImageMarker,
    UserComment,
    CloseSquare,
    OpenBracket,
    CloseBracket,
    HeadingMarker,
}

/// A data structure to encapsulate
/// data about where a token starts
/// and ends.
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize
}

/// A data structure to encapsulate
/// information on a captured token.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub end: Position,
    pub start: Position,
    pub value: Option<String>,
    pub token_type: TokenType
}

/// A function to check whether
/// an `Option<char>` contains
/// a character and if it does,
/// whether it is a reserved
/// character. A boolean is
/// returned to reflect this.
/// If the operation fails,
/// an error is returned.
pub fn is_alphanumeric(
    sub: &Option<char>
) -> Result<bool, JiraiErr> {
    let character: &char = match sub{
        Some(character) => character,
        None => return Err::<bool, JiraiErr>(
            JiraiErr::new("No character received.")
        )
    };
    Ok(
        !matches!(
            character,
            '\n' | '\r' | '>' | '<' |
            '(' | ')' | '[' | ']' |
            '#' | '_' | '*' | '$' |
            '%'
        )
    )
}

/// A function to tokenize
/// a string of Jirai source 
/// code. If the operation fails,
/// an error is returned. If the
/// operation is successful, a
/// vector of instances of the
/// `Token` structure is 
/// returned.
pub fn tokenize(
    sub: &str
) -> Result<Vec<Token>, JiraiErr>{
    let chars: Vec<char> = sub
        .to_string()
        .chars()
        .collect::<Vec<char>>();
    if chars.is_empty(){
        Err::<Vec<Token>, JiraiErr>(
            JiraiErr::new("The supplied string cannot be empty.")
        )
    }
    else {
        let mut line: usize = 0;
        let mut column: usize = 0;
        let mut cursor: usize = 0;
        let mut stream: Vec<Token> = Vec::new();
        while cursor < chars.len(){
            if chars.get(cursor) == Some(&'~'){
                stream.push(
                    Token{
                        end: Position{ line: line, column: column },
                        start: Position{ line: line, column: column },
                        value: None,
                        token_type: TokenType::Squiggly
                    }
                );
                column += 1;
                cursor += 1;
            }
            else if chars.get(cursor) == Some(&'%'){
                stream.push(
                    Token{
                        end: Position{ line: line, column: column },
                        start: Position{ line: line, column: column },
                        value: None,
                        token_type: TokenType::LinkMarker
                    }
                );
                column += 1;
                cursor += 1;
            }
            else if chars.get(cursor) == Some(&'$'){
                stream.push(
                    Token{
                        end: Position{ line: line, column: column },
                        start: Position{ line: line, column: column },
                        value: None,
                        token_type: TokenType::ImageMarker
                    }
                );
                column += 1;
                cursor += 1;
            }
            else if chars.get(cursor) == Some(&'<') &&
                chars.get(cursor + 1) == Some(&'3')
            {
                stream.push(
                    Token{
                        end: Position{ line: line, column: column + 1},
                        start: Position{ line: line, column: column },
                        value: None,
                        token_type: TokenType::HeadingMarker
                    }
                );
                column += 2;
                cursor += 2;
            }
            else if chars.get(cursor) == Some(&'>'){
                stream.push(
                    Token{
                        end: Position{ line: line, column: column },
                        start: Position{ line: line, column: column },
                        value: None,
                        token_type: TokenType::CloseAngle
                    }
                );
                column += 1;
                cursor += 1;
            }
            else if chars.get(cursor) == Some(&'('){
                stream.push(
                    Token{
                        end: Position{ line: line, column: column },
                        start: Position{ line: line, column: column },
                        value: None,
                        token_type: TokenType::OpenBracket
                    }
                );
                column += 1;
                cursor += 1;
            }
            else if chars.get(cursor) == Some(&'['){
                stream.push(
                    Token{
                        end: Position{ line: line, column: column },
                        start: Position{ line: line, column: column },
                        value: None,
                        token_type: TokenType::OpenSquare
                    }
                );
                column += 1;
                cursor += 1;
            }
            else if chars.get(cursor) == Some(&']'){
                stream.push(
                    Token{
                        end: Position{ line: line, column: column },
                        start: Position{ line: line, column: column },
                        value: None,
                        token_type: TokenType::CloseSquare
                    }
                );
                column += 1;
                cursor += 1;
            }
            else if chars.get(cursor) == Some(&')'){
                stream.push(
                    Token{
                        end: Position{ line: line, column: column },
                        start: Position{ line: line, column: column },
                        value: None,
                        token_type: TokenType::CloseBracket
                    }
                );
                column += 1;
                cursor += 1;
            }
            else if chars.get(cursor) == Some(&'<'){
                stream.push(
                    Token{
                        end: Position{ line: line, column: column },
                        start: Position{ line: line, column: column },
                        value: None,
                        token_type: TokenType::OpenAngle
                    }
                );
                column += 1;
                cursor += 1;
            }
            else if chars.get(cursor) == Some(&'*'){
                stream.push(
                    Token{
                        end: Position{ line: line, column: column },
                        start: Position{ line: line, column: column },
                        value: None,
                        token_type: TokenType::ItalicText
                    }
                );
                column += 1;
                cursor += 1;
            }
            else if chars.get(cursor) == Some(&'_'){
                stream.push(
                    Token{
                        end: Position{ line: line, column: column },
                        start: Position{ line: line, column: column },
                        value: None,
                        token_type: TokenType::BoldText
                    }
                );
                column += 1;
                cursor += 1;
            }
            else if chars.get(cursor) == Some(&'#'){
                let initial_char: char = match chars.get(cursor){
                    Some(initial_char) => *initial_char,
                    None => return Err::<Vec<Token>, JiraiErr>(
                        JiraiErr::new("Unexpected end of character stream.")
                    )
                };
                let column_start: usize = column;
                let mut value_buf: Vec<char> = Vec::new();
                value_buf.push(initial_char);
                cursor += 1;
                column += 1;
                while let Some(character) = chars.get(cursor){
                    if character == &'\n'{
                        break;
                    }
                    else {
                        value_buf.push(*character);
                        column += 1;
                        cursor += 1;
                    }
                }
                stream.push(
                    Token {
                        end: Position { line: line, column: column },
                        start: Position { line: line, column: column_start },
                        value: Some(value_buf.iter().collect::<String>()),
                        token_type: TokenType::UserComment
                    }
                );
            }
            else if is_alphanumeric(&chars.get(cursor).cloned())?{
                let initial: char = match chars.get(cursor){
                    Some(initial) => *initial,
                    None => return Err::<Vec<Token>, JiraiErr>(
                        JiraiErr::new("Unexpected end.")
                    )
                };
                let mut value_buf: Vec<char> = Vec::new();
                value_buf.push(initial);
                let column_start: usize = column;
                cursor += 1;
                column += 1;
                while let Some(stream_char) = chars.get(cursor){
                    if !is_alphanumeric(&Some(stream_char).cloned())?{
                        break;
                    }
                    else {
                        value_buf.push(*stream_char);
                        column += 1;
                        cursor += 1;
                    }
                }
                stream.push(
                    Token{
                        end: Position { line: line, column: column },
                        start: Position { line: line, column: column_start },
                        value: Some(value_buf.iter().collect::<String>()),
                        token_type: TokenType::UserString
                    }
                );
            } 
            else if chars.get(cursor) == Some(&'\r') &&
                chars.get(cursor + 1) == Some(&'\n')
            {
                 stream.push(
                    Token{
                        end: Position{ line: line, column: column + 1 },
                        start: Position{ line: line, column: column },
                        value: None,
                        token_type: TokenType::NewLine
                    }
                );
                line += 1;
                column = 0;
                cursor += 2;
            }
            else if chars.get(cursor) == Some(&'\r') ||
                chars.get(cursor) == Some(&'\n')
            {
                stream.push(
                    Token{
                        end: Position{ line: line, column: column },
                        start: Position{ line: line, column: column },
                        value: None,
                        token_type: TokenType::NewLine
                    }
                );
                line += 1;
                column = 0;
                cursor += 1;
            }
            else {
                let e: String = format!(
                    "Unexpected character at position \"{}:{}\"!",
                    line,
                    column
                );
                return Err::<Vec<Token>, JiraiErr>(
                    JiraiErr::new(&e.to_string())
                );
            }
        }
        Ok(stream)
    }
}
