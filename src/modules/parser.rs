/*
Jirai by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the structure
/// that encapsulates data
/// about a captured token.
use super::lexer::Token;

/// Importing the structure
/// to catch and handle errors.
use super::err::JiraiErr;

/// Importing the enumeration
/// that lists every possible
/// type of token Jirai source
/// code can include.
use super::lexer::TokenType;

/// An enumeration that "lists"
/// every possible type of block
/// statement that can exist in 
/// Jirai.
#[derive(Debug, Clone, PartialEq)]
pub enum BlockStatement{
    Comment,
    Paragraph(Vec<AtomicStatement>),
    UnorderedList(Vec<AtomicStatement>),
    Heading(usize, Vec<AtomicStatement>),
}

/// An enumeration "listing" all different
/// types of inline elements that can exist
/// in a Jirai document.
#[derive(Debug, Clone, PartialEq)]
pub enum AtomicStatement{
    Text(String),
    InlineCode(String),
    ImageLink(String,String),
    Link(String,String,String),
    ListItem(Box<Vec<AtomicStatement>>),
    BoldText(Box<Vec<AtomicStatement>>),
    ItalicText(Box<Vec<AtomicStatement>>)
}

/// A structure to
/// hold the stream
/// of captured tokens
/// and keep a cursor
/// to keep track of
/// the position in
/// the stream of 
/// tokens.
pub struct Parser {
    pub cursor: usize,
    pub stream: Vec<Token>
}

/// Implementing functions
/// for the `Parser` structure.
impl Parser {

    /// A function to create a new
    /// instance of the `Parser`
    /// structure and return it.
    /// If the stream of tokens 
    /// is empty, an error is returned.
    pub fn new(
        stream: &Vec<Token>
    ) -> Result<Parser, JiraiErr>{
        if stream.is_empty(){
            Err::<Parser, JiraiErr>(
                JiraiErr::new("Token stream cannot be empty.")
            )
        }
        else {
            Ok(Parser{
                cursor: 0,
                stream: stream.clone()
            })
        }
    }

    /// A function to check whether
    /// the cursor has reached the 
    /// end of the stream of tokens.
    /// A boolean is returned to
    /// reflect this.
    pub fn is_done(
        &self
    ) -> bool {
        self.cursor == self.stream.len()
    }

    /// A function to advance
    /// the internal cursor 
    /// by one.
    pub fn advance(
        &mut self
    ){
        self.cursor += 1;
    }

    /// A function to "peek"
    /// ahead at the token
    /// at the current position
    /// incremented by N. If the
    /// operation is successful,
    /// an instance of the `Token`
    /// structure is returned. If
    /// the operation fails, an
    /// error is returned.
    pub fn peek_n(
        &self,
        n: &usize
    ) -> Result<Token, JiraiErr>{
        let peeked: Token = match self.stream.get(self.cursor + n){
            Some(peeked) => peeked.clone(),
            None => return Err::<Token, JiraiErr>(
                JiraiErr::new("Unexpected end of token stream.")
            )
        };
        Ok(peeked)
    }

    /// A function to retrieve
    /// the current token in the
    /// stream of tokens and advance
    /// the internal cursor. If the
    /// operation is successful,
    /// an instance of the `Token`
    /// structure is returned. If
    /// the operation fails, an
    /// error is returned.
    pub fn expect(
        &mut self,
        token_type: &TokenType
    ) -> Result<Token, JiraiErr>{
        let peeked: Token = match self.stream.get(self.cursor).cloned(){
            Some(peeked) => peeked,
            None => return Err::<Token, JiraiErr>(
                JiraiErr::new("Unexpected end of token stream.")
            )
        };
        if *token_type == peeked.token_type{
            self.advance();
            Ok(peeked)
        }
        else {
            let e: String = format!(
                "Expected a token of type \"{:?}\"!",
                token_type
            );
            Err::<Token, JiraiErr>(
                JiraiErr::new(&e.to_string())
            )
        }
    }

    /// A function to parse the entire internal
    /// stream of tokens and return the parsed
    /// syntax tree as a vector of instances of
    /// the `BlockStatement` enumeration. If the
    /// operation fails, an error is returned.
    pub fn parse(
        &mut self
    ) -> Result<Vec<BlockStatement>, JiraiErr>{
        let mut statements: Vec<BlockStatement> = Vec::new();
        while !self.is_done(){
            let peeked: Token = self.peek_n(&0)?;
            if peeked.token_type == TokenType::NewLine{
                self.advance();
                continue;
            }
            match peeked.token_type{
                TokenType::UserComment => {
                    self.advance();
                    statements.push(
                        BlockStatement::Comment
                    )
                },
                TokenType::HeadingMarker => statements.push(
                    self.parse_heading()?
                ),
                TokenType::Squiggly => statements.push(
                    self.parse_unordered_list()?
                ), 
                _ => statements.push(
                    self.parse_paragraph()?
                )
            };
        }
        Ok(statements)
    }

    /// A function to parse a single heading.
    /// If the operation is successful, the 
    /// `Heading` variant of the 
    /// `BlockStatement` enumeration 
    /// is returned. If the operation fails,
    /// an error is returned.
    pub fn parse_heading(
        &mut self
    ) -> Result<BlockStatement, JiraiErr>{
        let mut level: usize = 0;
        while let Some(token) = self.stream.get(self.cursor){
            if token.token_type == TokenType::HeadingMarker{
                level += 1;
                self.advance();
            }
            else {
                break;
            }
        }
        if level == 0{
            Err::<BlockStatement, JiraiErr>(
                JiraiErr::new("Expected a heading marker.")
            )
        }
        else {
            let mut stmt_stream: Vec<AtomicStatement> = Vec::new();
            while let Some(token) = self.stream.get(self.cursor){
                if token.token_type == TokenType::NewLine{
                    self.advance();
                    break;
                }
                else {
                    stmt_stream.push(self.parse_atomic_statement()?);
                }
            }
            Ok(BlockStatement::Heading(level, stmt_stream))
        }
    }

    /// A function to parse a single unordered 
    /// list. If the operation is successful, 
    /// the `UnorderedList` variant of the 
    /// `BlockStatement` enumeration is 
    /// returned. If the operation fails,
    /// an error is returned.
    pub fn parse_unordered_list(
        &mut self
    ) -> Result<BlockStatement, JiraiErr>{
        let mut stmt_buf: Vec<AtomicStatement> = Vec::new();
        while !self.is_done(){
            let token = self.peek_n(&0)?;
            if token.token_type == TokenType::Squiggly{
                stmt_buf.push(self.parse_list_item()?);
            }
            else {
                break;
            }
        }
        Ok(BlockStatement::UnorderedList(stmt_buf))
    } 

    /// A function to parse a single paragraph.
    /// If the operation is successful, the 
    /// `Paragraph` variant of the 
    /// `BlockStatement` enumeration is 
    /// returned. If the operation fails,
    /// an error is returned.
    pub fn parse_paragraph(
        &mut self
    ) -> Result<BlockStatement, JiraiErr>{
        let mut stmt_buf: Vec<AtomicStatement> = Vec::new();
        while !self.is_done(){
            let next: Token = self.peek_n(&0)?;
            match next.token_type{
                TokenType::NewLine => {
                    self.advance();
                    break;
                },
                TokenType::Squiggly => break,
                TokenType::UserComment => break,
                TokenType::HeadingMarker => break,
                _ => stmt_buf.push(
                    self.parse_atomic_statement()?
                )
            };
        }
        if stmt_buf.is_empty() {
            Err(JiraiErr::new("Empty paragraph."))
        }
        else {
            Ok(BlockStatement::Paragraph(stmt_buf))
        }
    }

    /// A function to parse a single inline
    /// item. If the operation is successful,
    /// an instance of the `AtomicStatement`
    /// enumeration is returned. If the
    /// operation fails, an error is returned.
    pub fn parse_atomic_statement(
        &mut self
    ) -> Result<AtomicStatement, JiraiErr>{
        let peeked: Token = self.peek_n(&0)?;
        match peeked.token_type{
            TokenType::UserString => Ok(self.parse_text()?),
            TokenType::Squiggly => Ok(self.parse_list_item()?),
            TokenType::BoldText => Ok(self.parse_bold_text()?),
            TokenType::OpenAngle => Ok(self.parse_inline_code()?),
            TokenType::ItalicText => Ok(self.parse_italic_text()?),
            TokenType::CloseAngle => Ok(self.parse_linked_item()?),
            _ => Err::<AtomicStatement, JiraiErr>(
                JiraiErr::new("Expected an inline element.")
            )
        }
    }

    /// A function to parse a single linked 
    /// item. If the operation is successful,
    /// either the `ImageLink` variant or the 
    /// `Link` variant of the `AtomicStatement` 
    /// enumeration is returned. If the operation
    /// fails, an error is returned.
    pub fn parse_linked_item(
        &mut self
    ) -> Result<AtomicStatement, JiraiErr>{
        let open_angle: Token = self.peek_n(&0)?;
        let open_bracket: Token = self.peek_n(&1)?;
        let type_marker: Token = self.peek_n(&2)?;
        if open_angle.token_type == TokenType::CloseAngle &&
           open_bracket.token_type == TokenType::OpenBracket &&
           type_marker.token_type == TokenType::ImageMarker
        {
            Ok(self.parse_image_link()?)
        }
        else if open_angle.token_type == TokenType::CloseAngle &&
           open_bracket.token_type == TokenType::OpenBracket &&
           type_marker.token_type == TokenType::LinkMarker
        {
            Ok(self.parse_link()?)
        }
        else {
            Err::<AtomicStatement, JiraiErr>(
                JiraiErr::new("Expected a linked item.")
            )
        }
    }

    /// A function to parse an italic section
    /// of text. If the operation is successful,
    /// the `ItalicText` variant of the 
    /// `AtomicStatement` enumeration is 
    /// returned. If the operation fails,
    /// an error is returned.
    pub fn parse_italic_text(
        &mut self
    ) -> Result<AtomicStatement, JiraiErr>{
        let _open_italic: Token = self.expect(&TokenType::ItalicText)?;
        let mut stmt_buf: Vec<AtomicStatement> = Vec::new();
        while let Some(token) = self.stream.get(self.cursor){
            if token.token_type == TokenType::ItalicText{
                self.advance();
                break;
            }
            else {
                stmt_buf.push(self.parse_atomic_statement()?);
            }
        }
        Ok(AtomicStatement::ItalicText(Box::new(stmt_buf)))
    }

    /// A function to parse a bold section
    /// of text. If the operation is successful,
    /// the `BoldText` variant of the 
    /// `AtomicStatement` enumeration is 
    /// returned. If the operation fails,
    /// an error is returned.
    pub fn parse_bold_text(
        &mut self
    ) -> Result<AtomicStatement, JiraiErr>{
        let _open_italic: Token = self.expect(&TokenType::BoldText)?;
        let mut stmt_buf: Vec<AtomicStatement> = Vec::new();
        while let Some(token) = self.stream.get(self.cursor){
            if token.token_type == TokenType::BoldText{
                self.advance();
                break;
            }
            else {
                stmt_buf.push(self.parse_atomic_statement()?);
            }
        }
        Ok(AtomicStatement::BoldText(Box::new(stmt_buf)))
    }

    /// A function to parse a single list 
    /// item. If the operation is successful,
    /// the `ListItem` variant of the 
    /// `AtomicStatement` enumeration is 
    /// returned. If the operation fails,
    /// an error is returned.
    pub fn parse_list_item(
        &mut self
    ) -> Result<AtomicStatement, JiraiErr>{
        let _open_italic: Token = self.expect(&TokenType::Squiggly)?;
        let mut stmt_buf: Vec<AtomicStatement> = Vec::new();
        while let Some(token) = self.stream.get(self.cursor){
            if token.token_type == TokenType::NewLine{
                self.advance();
                break;
            }
            else {
                stmt_buf.push(self.parse_atomic_statement()?);
            }
        }
        Ok(AtomicStatement::ListItem(Box::new(stmt_buf)))
    }

    /// A function to parse a single image
    /// link. If the operation is successful,
    /// the `ImageLink` variant of the 
    /// `AtomicStatement` enumeration is 
    /// returned. If the operation fails,
    /// an error is returned.
    pub fn parse_image_link(
        &mut self
    ) -> Result<AtomicStatement, JiraiErr>{
        let _open_angle: Token = self.expect(&TokenType::CloseAngle)?;
        let _open_bracket: Token = self.expect(&TokenType::OpenBracket)?;
        let _image_marker: Token = self.expect(&TokenType::ImageMarker)?;
        let _alt_open_square: Token = self.expect(&TokenType::OpenSquare)?;
        let alt_text_token: Token = self.expect(&TokenType::UserString)?;
        let _alt_close_square: Token = self.expect(&TokenType::CloseSquare)?;
        let _link_url_open_square: Token = self.expect(&TokenType::OpenSquare)?;
        let link_url_token: Token = self.expect(&TokenType::UserString)?;
        let _link_url_close_square: Token = self.expect(&TokenType::CloseSquare)?;
        let _close_bracket: Token = self.expect(&TokenType::CloseBracket)?;
        let _close_angle: Token = self.expect(&TokenType::OpenAngle)?;
        let alt_text: String = match alt_text_token.value{
            Some(alt_text) => alt_text,
            None => return Err::<AtomicStatement, JiraiErr>(
                JiraiErr::new("Expected \"alt\" text.")
            )
        };
        let url: String = match link_url_token.value{
            Some(url) => url,
            None => return Err::<AtomicStatement, JiraiErr>(
                JiraiErr::new("Expected a URL.")
            )
        };
        Ok(AtomicStatement::ImageLink(alt_text, url))
    }

    /// A function to parse a single
    /// link. If the operation is successful,
    /// the `Link` variant of the 
    /// `AtomicStatement` enumeration is 
    /// returned. If the operation fails,
    /// an error is returned.
    pub fn parse_link(
        &mut self
    ) -> Result<AtomicStatement, JiraiErr>{
        let _open_angle: Token = self.expect(&TokenType::CloseAngle)?;
        let _open_bracket: Token = self.expect(&TokenType::OpenBracket)?;
        let _link_marker: Token = self.expect(&TokenType::LinkMarker)?;
        let _alt_open_square: Token = self.expect(&TokenType::OpenSquare)?;
        let alt_text_token: Token = self.expect(&TokenType::UserString)?;
        let _alt_close_square: Token = self.expect(&TokenType::CloseSquare)?;
        let _link_text_open_square: Token = self.expect(&TokenType::OpenSquare)?;
        let link_text_token: Token = self.expect(&TokenType::UserString)?;
        let _link_text_close_square: Token = self.expect(&TokenType::CloseSquare)?;
        let _link_url_open_square: Token = self.expect(&TokenType::OpenSquare)?;
        let link_url_token: Token = self.expect(&TokenType::UserString)?;
        let _link_url_close_square: Token = self.expect(&TokenType::CloseSquare)?;
        let _close_bracket: Token = self.expect(&TokenType::CloseBracket)?;
        let _close_angle: Token = self.expect(&TokenType::OpenAngle)?;
        let alt_text: String = match alt_text_token.value{
            Some(alt_text) => alt_text,
            None => return Err::<AtomicStatement, JiraiErr>(
                JiraiErr::new("Expected \"alt\" text.")
            )
        };
        let link_text: String = match link_text_token.value{
            Some(link_text) => link_text,
            None => return Err::<AtomicStatement, JiraiErr>(
                JiraiErr::new("Expected link text.")
            )
        };
        let url: String = match link_url_token.value{
            Some(url) => url,
            None => return Err::<AtomicStatement, JiraiErr>(
                JiraiErr::new("Expected a URL.")
            )
        };
        Ok(AtomicStatement::Link(alt_text, link_text, url))
    }

    /// A function to parse simple text.
    /// If the operation is successful,
    /// the `Text` variant of the 
    /// `AtomicStatement` enumeration is 
    /// returned. If the operation fails,
    /// an error is returned.
    pub fn parse_text(
        &mut self
    ) -> Result<AtomicStatement, JiraiErr>{
        let text_token: Token = self.expect(&TokenType::UserString)?;
        let text: String = match text_token.value{
            Some(text) => text,
            None => return Err::<AtomicStatement, JiraiErr>(
                JiraiErr::new("Expected link text.")
            )
        };
        Ok(AtomicStatement::Text(text))
    }

    /// A function to parse an inline code
    /// section. If the operation is successful,
    /// the `InlineCode` variant of the 
    /// `AtomicStatement` enumeration is 
    /// returned. If the operation fails,
    /// an error is returned.
    pub fn parse_inline_code(
        &mut self
    ) -> Result<AtomicStatement, JiraiErr>{
        let _open_angle: Token = self.expect(&TokenType::OpenAngle)?;
        let code_text_token: Token = self.expect(&TokenType::UserString)?;
        let _closing_angle: Token = self.expect(&TokenType::CloseAngle)?;
        let code: String = match code_text_token.value{
            Some(code) => code,
            None => return Err::<AtomicStatement, JiraiErr>(
                JiraiErr::new("Expected code.")
            )
        };
        Ok(AtomicStatement::InlineCode(code))
    }
}
