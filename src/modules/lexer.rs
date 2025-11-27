/*
Jirai by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the structure
/// to catch and handle errors.
use super::err::JiraiErr;

/// An enumeration that
/// lists all possible types
/// of Jirai tokens.
#[derive(Clone, Debug, PartialEq)]
pub enum TokenType{
    NewLine,
    BoldText,
    OpenCurly,
    OpenAngle,
    UserString,
    CloseAngle,
    CloseCurly,
    ListMarker,
    ItalicText,
    LinkMarker,
    OpenSquare,
    ImageMarker,
    CloseSquare,
    OpenBracket,
    CloseBracket,
    HeadingMarker,
    DocumentLimiter,
}

/// A structure encapsulating
/// data about a token encountered
/// in Jirai source code.
#[derive(Clone, Debug, PartialEq)]
pub struct Token{
    pub end: Position,
    pub start: Position,
    pub token_type: TokenType,
    pub value: Option<String>
}

/// Implementing functions for
/// the `Token` structure.
impl Token {

    /// Implementing a function
    /// to create a new instance
    /// of the `Token` structure
    /// and return that instance.
    pub fn new(
        end: &Position,
        start: &Position,
        token_type: &TokenType,
        value: &Option<String>
    ) -> Token {
        Token{
            end: end.clone(),
            start: start.clone(),
            token_type: token_type.clone(),
            value: value.clone()
        }
    }

}

/// A structure to encapsulate
/// data about the start and end
/// of a captured token.
#[derive(Clone, Debug, PartialEq)]
pub struct Position{
    pub line: usize,
    pub column: usize
}

/// implementing important
/// functions for the `Position`
/// structure.
impl Position {

    /// A function to create a 
    /// new instance of the `Position`
    /// structure and return it.
    pub fn new(
        line: &usize,
        column: &usize
    ) -> Position {
        Position{
            line: *line,
            column: *column
        }
    }

    /// A function to return a string
    /// representation of the `Position`
    /// structure.
    pub fn to_string(
        &self
    ) -> String {
        format!(
            "{:?}:{:?}", 
            &self.line, 
            &self.column
        )
    }
}

/// A function to check whether the
/// passed character is any of the
/// reserved characters. If it is,
/// a boolean `false` is returned.
/// If it is not, a boolean `true`
/// is returned.
pub fn is_text(
    sub: &char
) -> bool {
    if "<>*$()[]{}^-~#@\n\r"
        .to_string()
        .chars()
        .collect::<Vec<char>>()
        .contains(sub)
    {
        false
    }
    else {
        true
    }
}

/// A function to split a string
/// of Jirai source into a vector
/// of instances of the `Token` structure.
/// If the tokenization of a Jirai source
/// code string is successful, this vector
/// is returned. If an illegal character 
/// is detected, an error is returned.
pub fn tokenize_string(
    sub: &str
) -> Result<Vec<Token>, JiraiErr> {
    let chars: Vec<char> = sub
        .to_string()
        .chars()
        .collect::<Vec<char>>();
    if chars.len() == 0{
        Err::<Vec<Token>, JiraiErr>(
            JiraiErr::new("Source cannot be empty.")
        )
    }
    else {
        let mut result: Vec<Token> = Vec::new();
        let mut cursor: usize = 0;
        let mut line_count: usize = 0;
        let mut column_count: usize = 0;
        while cursor < chars.len(){

            if chars.get(cursor) == Some(&'<') &&
               chars.get(cursor + 1) == Some(&'3')
            {
                result.push(
                    Token::new(
                        &Position::new(&line_count, &(column_count + 2)),
                        &Position::new(&line_count, &column_count),
                        &TokenType::HeadingMarker,
                        &None
                    )
                );
                cursor += 2;
                column_count += 2;
            }
            else if chars.get(cursor) == Some(&'(') &&
                chars.get(cursor + 1) == Some(&'^') &&
                chars.get(cursor + 2) == Some(&'-') &&
                chars.get(cursor + 3) == Some(&'^') &&
                chars.get(cursor + 4) == Some(&')')
            {
                result.push(
                    Token::new(
                        &Position::new(&line_count, &(column_count + 5)),
                        &Position::new(&line_count, &column_count),
                        &TokenType::DocumentLimiter,
                        &None
                    )
                );
                cursor += 5;
                column_count += 5;
            }
            else if is_text(&chars[cursor])
            {
                let column_count_start = column_count;
                let cursor_start = cursor;
                let mut char_buf: Vec<char> = Vec::new();
                while let Some(&c) = chars.get(cursor){
                    if is_text(&c){
                        char_buf.push(c);
                        cursor += 1;
                    }
                    else {
                        break;
                    }
                }
                let consumed_length = cursor - cursor_start;
                column_count += consumed_length;
                result.push(
                    Token::new(
                        &Position::new(&line_count, &(column_count)),
                        &Position::new(&line_count, &column_count_start),
                        &TokenType::UserString,
                        &Some(char_buf.into_iter().collect::<String>())
                    )
                );
            }
            else if chars.get(cursor) == Some(&'~')
            {
                result.push(
                    Token::new(
                        &Position::new(&line_count, &(column_count + 1)),
                        &Position::new(&line_count, &column_count),
                        &TokenType::ListMarker,
                        &None
                    )
                );
                cursor += 1;
                column_count += 1;

            }
            else if chars.get(cursor) == Some(&'>')
            {
               result.push(
                    Token::new(
                        &Position::new(&line_count, &(column_count + 1)),
                        &Position::new(&line_count, &column_count),
                        &TokenType::CloseAngle,
                        &None
                    )
                );
                cursor += 1;
                column_count += 1;

            }
            else if chars.get(cursor) == Some(&'<')
            {
                result.push(
                    Token::new(
                        &Position::new(&line_count, &(column_count + 1)),
                        &Position::new(&line_count, &column_count),
                        &TokenType::OpenAngle,
                        &None
                    )
                );
                cursor += 1;
                column_count += 1;
            }
            else if chars.get(cursor) == Some(&'[')
            {
                result.push(
                    Token::new(
                        &Position::new(&line_count, &(column_count + 1)),
                        &Position::new(&line_count, &column_count),
                        &TokenType::OpenSquare,
                        &None
                    )
                );
                cursor += 1;
                column_count += 1;
            }
            else if chars.get(cursor) == Some(&']')
            {
                result.push(
                    Token::new(
                        &Position::new(&line_count, &(column_count + 1)),
                        &Position::new(&line_count, &column_count),
                        &TokenType::CloseSquare,
                        &None
                    )
                );
                cursor += 1;
                column_count += 1;

            }
            else if chars.get(cursor) == Some(&'{')
            {
                result.push(
                    Token::new(
                        &Position::new(&line_count, &(column_count + 1)),
                        &Position::new(&line_count, &column_count),
                        &TokenType::OpenCurly,
                        &None
                    )
                );
                cursor += 1;
                column_count += 1;

            }
            else if chars.get(cursor) == Some(&'}')
            {
                result.push(
                    Token::new(
                        &Position::new(&line_count, &(column_count + 1)),
                        &Position::new(&line_count, &column_count),
                        &TokenType::CloseCurly,
                        &None
                    )
                );
                cursor += 1;
                column_count += 1;
            }
            else if chars.get(cursor) == Some(&'$')
            {
                result.push(
                    Token::new(
                        &Position::new(&line_count, &(column_count + 1)),
                        &Position::new(&line_count, &column_count),
                        &TokenType::ItalicText,
                        &None
                    )
                );
                cursor += 1;
                column_count += 1;
            }
            else if chars.get(cursor) == Some(&'*')
            {
                result.push(
                    Token::new(
                        &Position::new(&line_count, &(column_count + 1)),
                        &Position::new(&line_count, &column_count),
                        &TokenType::BoldText,
                        &None
                    )
                );
                cursor += 1;
                column_count += 1;
            } 
            else if chars.get(cursor) == Some(&'@')
            {
                result.push(
                    Token::new(
                        &Position::new(&line_count, &(column_count + 1)),
                        &Position::new(&line_count, &column_count),
                        &TokenType::ImageMarker,
                        &None
                    )
                );
                cursor += 1;
                column_count += 1;
            }
            else if chars.get(cursor) == Some(&'#')
            {
                result.push(
                    Token::new(
                        &Position::new(&line_count, &(column_count + 1)),
                        &Position::new(&line_count, &column_count),
                        &TokenType::LinkMarker,
                        &None
                    )
                );
                cursor += 1;
                column_count += 1;
            }
            else if chars.get(cursor) == Some(&'\n')
            {
                result.push(
                    Token::new(
                        &Position::new(&line_count, &(column_count + 1)),
                        &Position::new(&line_count, &column_count),
                        &TokenType::NewLine,
                        &None
                    )
                );
                cursor += 1;
                column_count += 1;
                line_count += 1;
            }
            else if chars.get(cursor) == Some(&'\r')
            {
                result.push(
                    Token::new(
                        &Position::new(&line_count, &(column_count + 1)),
                        &Position::new(&line_count, &column_count),
                        &TokenType::NewLine,
                        &None
                    )
                );
                cursor += 1;
                column_count += 1;
                line_count += 1;
            }
            else if chars.get(cursor) == Some(&'\r') &&
                chars.get(cursor + 1) == Some(&'\n')
            {
                result.push(
                    Token::new(
                        &Position::new(&line_count, &(column_count + 2)),
                        &Position::new(&line_count, &column_count),
                        &TokenType::NewLine,
                        &None
                    )
                );
                cursor += 2;
                column_count += 2;
                line_count += 1;
            }
            else if chars.get(cursor) == Some(&'(')
            {
                result.push(
                    Token::new(
                        &Position::new(&line_count, &(column_count + 1)),
                        &Position::new(&line_count, &column_count),
                        &TokenType::OpenBracket,
                        &None
                    )
                );
                cursor += 1;
                column_count += 1;
            }
            else if chars.get(cursor) == Some(&')')
            {
                result.push(
                    Token::new(
                        &Position::new(&line_count, &(column_count + 1)),
                        &Position::new(&line_count, &column_count),
                        &TokenType::CloseBracket,
                        &None
                    )
                );
                cursor += 1;
                column_count += 1;
            }
            else {
                let e: String = format!(
                    "Unexpected character(s) at position \"{}:{}\": \"{}\"!", 
                    &line_count, 
                    &column_count,
                    &chars[cursor]
                );
                return Err::<Vec<Token>, JiraiErr>(
                    JiraiErr::new(&e.to_string())
                );
            }
        }
        Ok(result)
    }
}
