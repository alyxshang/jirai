/*
Jirai by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the structure
/// holding information on
/// a capture token.
use super::lexer::Token;

/// Importing the structure
/// to catch and handle errors.
use super::err::JiraiErr;

/// Importing the enumeration
/// containing all possible types
/// Jirai tokens.
use super::lexer::TokenType;

/// An enumeration
/// describing which
/// type of Jirai string
/// was received.
#[derive(Clone, PartialEq)]
pub enum SourceType{
    Slice,
    Document
}

/// An enumeration containing
/// every single type of block
/// element in Jirai source code.
/// Each variant can contain any
/// number of sub-statements
#[derive(PartialEq, Debug, Clone)]
pub enum Statement{
    Heading(usize, Vec<InlineStatement>),
    Paragraph(Vec<InlineStatement>),
    UnorderedList(Vec<InlineStatement>)
}

/// An enumeration containing
/// every possible type of inline 
/// statement that can be present
/// in Jirai source. Some of the 
/// variants represent terminals,
/// whereas others represent 
/// non-terminals.
#[derive(PartialEq, Debug, Clone)]
pub enum InlineStatement{
    Link(Link),
    Code(String),
    Text(String),
    Image(Image),
    BlockQuote(String),
    ListItem(Box<Vec<InlineStatement>>),
    BoldText(Box<Vec<InlineStatement>>),
    ItalicText(Box<Vec<InlineStatement>>)
}

/// A structure to encapsulate
/// information on a parsed link.
#[derive(PartialEq, Debug, Clone)]
pub struct Link{
    pub alt: Option<String>,
    pub url: String,
    pub link_text: String
}

/// A structure to encapsulate
/// information on a parsed image.
#[derive(PartialEq, Debug, Clone)]
pub struct Image{
    pub alt: Option<String>,
    pub url: String,
}

/// A structure to hold a stream
/// of tokens lexed from Jirai source
/// code and 
pub struct Parser{
    pub cursor: usize,
    pub stream: Vec<Token>,
    pub source_type: SourceType
}

/// Implementing functions
/// for the `Parser`
/// structure.
impl Parser {

    /// A function to create a new instance
    /// of the `Parser` structure and return
    /// it. If the stream of tokens is empty,
    /// an error is returned instead.
    pub fn new(
        source_type: &SourceType,
        stream: &Vec<Token>
    ) -> Result<Parser, JiraiErr> {
        if stream.len() == 0{
            return Err::<Parser, JiraiErr>(
                JiraiErr::new("Token stream cannot be empty.")
            );
        }
        else {
            Ok(
                Parser {
                    cursor: 0,
                    stream: stream.clone(),
                    source_type: source_type.clone()
                }
            )
        }
    }

    /// Advances the internal cursor
    /// to consume the current token.
    /// Nothing is returned.
    pub fn advance(
        &mut self
    ) -> () {
        self.cursor += 1;
    }

    /// A function returning
    /// a boolean to return information
    /// on whether the end of the token
    /// stream has been reached or not.
    pub fn is_done(
        &self
    ) -> bool {
        &self.cursor == &self.stream.len()
    }

    /// A function to "peek" ahead and retrieve the 
    /// current token and return it. If the end of
    /// the token stream has been reached unexpectedly,
    /// an error is returned.
    pub fn peek(
        &mut self,
    ) -> Result<Token, JiraiErr>{
        if !self.is_done(){
            let res: Token = match self.stream.get(self.cursor){
                Some(res) => res.clone(),
                None => return Err::<Token, JiraiErr>(
                    JiraiErr::new("Unexpected end of token stream.")
                )
            };
            Ok(res)
        }
        else {
            return Err::<Token, JiraiErr>(
                JiraiErr::new("End of token stream reached.")
            );
        }
    }

    /// A function to retrieve the current token,
    /// compare the type of the token to the expected type
    /// of token supplied as a parameter, and return it
    /// if they match. The cursor is also advanced along
    /// the token stream. If the token cannot be retrieved or
    /// the token is not of the expected type, an error is
    /// returned.
    pub fn expect(
        &mut self,
        token_type: &TokenType
    ) -> Result<Token, JiraiErr>{
        let current: Token = match self.stream.get(self.cursor){
            Some(current) => current.clone(),
            None => return Err::<Token, JiraiErr>(
                JiraiErr::new("Unexpected end of token stream.")
            )
        };
        if current.token_type == *token_type{
            self.advance();
            Ok(current)
        }
        else {
            Err::<Token, JiraiErr>(            
                JiraiErr::new(
                    &format!(
                        "Expected token of type \"{:?}\" at position {}!", 
                        &token_type, 
                        &current.start.to_string()
                    )
                )
            )
        }
    }

    /// A function to check whether the stream
    /// of tokens starts with the `DocumentLimiter`
    /// type of token. A boolean reflecting this
    /// is returned.
    pub fn starts_with(
        &mut self
    ) -> bool {
        match self.stream.get(0){
            Some(token) => token.token_type == TokenType::DocumentLimiter,
            None => false
        }
    }

    /// A function to check whether the stream
    /// of tokens ends with the `DocumentLimiter`
    /// type of token. A boolean reflecting this
    /// is returned.
    pub fn ends_with(
        &mut self
    ) -> bool {
        let last_idx: usize = self.stream.len() - 1;
        match self.stream.get(last_idx){
            Some(token) => token.token_type == TokenType::DocumentLimiter,
            None => false
        }
    }


    /// The main function to parse the token stream.
    /// If the operation is successful, a stream of
    /// statements is returned. If the operation
    /// fails, an error is returned.
    pub fn parse(
        &mut self
    ) -> Result<Vec<Statement>, JiraiErr>{
        let mut statements: Vec<Statement> = Vec::new();
        while !self.is_done(){
            let current: Token = self.peek()?;
            match current.token_type{
                TokenType::HeadingMarker => statements.push(self.parse_heading()?),
                _ => statements.push(self.parse_block_element()?),
            };
        }
        Ok(statements)
    }

    /// A function to parse the block element of the
    /// heading. If the operation is successful,
    /// the `Heading` variant of the `Statement`
    /// enumeration is returned. If the operation
    /// fails, an error is returned.
    pub fn parse_heading(
        &mut self
    ) -> Result<Statement, JiraiErr>{
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
            return Err::<Statement, JiraiErr>(
                JiraiErr::new(
                    &format!(
                        "Expected heading marker at position \"{}\"!", 
                        &self.stream[self.cursor].end.to_string()
                    )
                )
            );
        }
        else { 
            let mut stmt_vec: Vec<InlineStatement> = Vec::new();
            while let Some(token) = self.stream.get(self.cursor){
                if token.token_type == TokenType::NewLine{
                    self.advance();
                    break;
                }
                else {
                    stmt_vec.push(self.parse_inline_statement()?);
                }
            }
            Ok(Statement::Heading(level, stmt_vec))
        }
    }

    /// A function to parse block elements that
    /// are not headings. If the operation is
    /// successful, a variant of the `Statement`
    /// enumeration is returned. If the operation
    /// fails, an error is returned.
    pub fn parse_block_element(
        &mut self
    ) -> Result<Statement, JiraiErr>{
        let peeked: Token = self.peek()?;
        match peeked.token_type{
            TokenType::ListMarker => Ok(self.parse_unordered_list()?),
            _ => Ok(self.parse_paragraph()?)
        }
    }

    /// A function to parse the block element of the
    /// paragraph. If the operation is successful,
    /// the `Paragraph` variant of the `Statement`
    /// enumeration is returned. If the operation
    /// fails, an error is returned.
    pub fn parse_paragraph(
        &mut self
    ) -> Result<Statement, JiraiErr>{
        let mut stmt_vec: Vec<InlineStatement> = Vec::new();
        while let Some(token) = self.stream.get(self.cursor){
            if token.token_type == TokenType::NewLine{
                self.advance();
                break;
            }
            else {
                stmt_vec.push(self.parse_inline_statement()?);
            }
        }
        Ok(Statement::Paragraph(stmt_vec))
    }

    /// A function to parse the block element of the
    /// unordered list. If the operation is successful,
    /// the `UnorderedList` variant of the `Statement`
    /// enumeration is returned. If the operation
    /// fails, an error is returned.
    pub fn parse_unordered_list(
        &mut self
    ) -> Result<Statement, JiraiErr>{
        let mut stmt_vec: Vec<InlineStatement> = Vec::new();
        while let Some(token) = self.stream.get(self.cursor){
            if token.token_type == TokenType::NewLine{
                self.advance();
                break;
            }
            else {
                stmt_vec.push(self.parse_inline_statement()?);
            }
        }
        Ok(Statement::UnorderedList(stmt_vec))
    }

    /// A function to parse an inline block element.
    /// If the operation is successful, a variant 
    /// of the `InlineStatement` enumeration is 
    /// returned. If the operation
    /// fails, an error is returned.
    pub fn parse_inline_statement(
        &mut self
    ) -> Result<InlineStatement, JiraiErr>{
        let peeked: Token = self.peek()?;
        match peeked.token_type{
            TokenType::BoldText => Ok(self.parse_bold_text()?),
            TokenType::ListMarker => Ok(self.parse_list_item()?),
            TokenType::OpenAngle => Ok(self.parse_inline_code()?),
            TokenType::OpenCurly => Ok(self.parse_linked_item()?),
            TokenType::ItalicText => Ok(self.parse_italic_text()?),
            TokenType::CloseAngle => Ok(self.parse_block_quote()?),
            _ => Ok(self.parse_text()?)
        }
    }

    /// A function to parse inline markup for
    /// links or images. If the operation is
    /// successful either the `Image` or `Link`
    /// variant of the `InlineStatement` enumeration
    /// is returned. If the operation fails, an error
    /// is returned.
    pub fn parse_linked_item(
        &mut self
    ) -> Result<InlineStatement, JiraiErr>{
        let _open_curly: Token = self.expect(&TokenType::OpenCurly)?;
        let peeked: Token = self.peek()?;
        match peeked.token_type{
            TokenType::ImageMarker => Ok(self.parse_image_item()?),
            TokenType::LinkMarker => Ok(self.parse_link_item()?),
            _ => Err::<InlineStatement, JiraiErr>(
                JiraiErr::new(
                    &format!(
                        "Expected a link or image marker at position \"{}\"!", 
                        &peeked.start.to_string()
                    )
                )
            )
        }
    }

    /// A function to parse inline markup for
    /// bold text. If the operation is successful the
    /// `BoldText` variant of the `InlineStatement` 
    /// enumeration is returned. If the operation fails, 
    /// an error is returned.
    pub fn parse_bold_text(
        &mut self
    ) -> Result<InlineStatement, JiraiErr>{
        let _open_bold: Token = self.expect(&TokenType::BoldText)?;
        let mut contents: Vec<InlineStatement> = Vec::new();
        loop {
            let next: Token = self.peek()?;
            if next.token_type == TokenType::BoldText{
                break;
            }
            else {
                contents.push(self.parse_inline_statement()?);
            }
        }
        let _close_bold: Token = self.expect(&TokenType::BoldText)?;
        Ok(InlineStatement::BoldText(Box::new(contents)))
    }

    /// A function to parse inline markup for
    /// italic text. If the operation is successful the
    /// `ItalicText` variant of the `InlineStatement` 
    /// enumeration is returned. If the operation fails, 
    /// an error is returned.
    pub fn parse_italic_text(
        &mut self
    ) -> Result<InlineStatement, JiraiErr>{
        let _open_italic: Token = self.expect(&TokenType::ItalicText)?;
        let mut contents: Vec<InlineStatement> = Vec::new();
        loop {
            let next: Token = self.peek()?;
            if next.token_type == TokenType::ItalicText{
                break;
            }
            else {
                contents.push(self.parse_inline_statement()?);
            }
        }
        let _close_italic: Token = self.expect(&TokenType::ItalicText)?;
        Ok(InlineStatement::ItalicText(Box::new(contents)))
    }

    /// A function to parse inline markup for
    /// a list item. If the operation is successful the
    /// `ListItem` variant of the `InlineStatement` 
    /// enumeration is returned. If the operation fails, 
    /// an error is returned.
    pub fn parse_list_item(
        &mut self
    ) -> Result<InlineStatement, JiraiErr>{
        let _list_marker: Token = self.expect(&TokenType::ListMarker)?;
        let mut contents: Vec<InlineStatement> = Vec::new();
        loop {
            let next: Token = self.peek()?;
            if next.token_type == TokenType::NewLine || self.is_done(){
                break;
            }
            else {
                contents.push(self.parse_inline_statement()?);
            }
        }
        Ok(InlineStatement::ListItem(Box::new(contents)))
    }

    /// A function to parse inline markup for
    /// inline code. If the operation is successful the
    /// `Code` variant of the `InlineStatement` 
    /// enumeration is returned. If the operation fails, 
    /// an error is returned.
    pub fn parse_inline_code(
        &mut self
    ) -> Result<InlineStatement, JiraiErr>{
        let open_angle: Token = self.expect(&TokenType::OpenAngle)?;
        let code_text: Token = self.expect(&TokenType::UserString)?;
        let _close_angle: Token = self.expect(&TokenType::CloseAngle)?;
        let text_str: String = match code_text.value {
            Some(text_str) => text_str,
            None => return Err::<InlineStatement, JiraiErr>(
                JiraiErr::new(
                    &format!(
                        "Expected text at positon \"{}\"!",
                        open_angle.end.to_string()
                    )
                )
            )
        };
        Ok(InlineStatement::Code(text_str))
    }

    /// A function to parse inline markup for an
    /// inline block quote. If the operation is successful, 
    /// the `BlockQuote` variant of the `InlineStatement` 
    /// enumeration is returned. If the operation fails, 
    /// an error is returned.
    pub fn parse_block_quote(
        &mut self
    ) -> Result<InlineStatement, JiraiErr>{
        let _close_angle_bracket: Token = self.expect(&TokenType::CloseAngle)?;
        let open_bracket: Token = self.expect(&TokenType::OpenBracket)?;
        let quote: Token = self.expect(&TokenType::UserString)?;
        let _close_bracket: Token = self.expect(&TokenType::CloseBracket)?;
        let _open_angle_bracket: Token = self.expect(&TokenType::OpenAngle)?;
        let quote_text: String = match quote.value{
            Some(quote_text) => quote_text,
            None => return Err::<InlineStatement, JiraiErr>(
                JiraiErr::new(
                    &format!(
                        "Expected text at positon \"{}\"!",
                        open_bracket.end.to_string()
                    )
                )
            )
        };
        Ok(InlineStatement::BlockQuote(quote_text))
    }


    /// A function to parse inline markup for an
    /// inline link. If the operation is successful the
    /// `Link` variant of the `InlineStatement` 
    /// enumeration is returned. If the operation fails, 
    /// an error is returned.
    pub fn parse_link_item(
        &mut self
    ) -> Result<InlineStatement, JiraiErr>{
        let _link_marker: Token = self.expect(&TokenType::LinkMarker)?;
        let _alt_open_square: Token = self.expect(&TokenType::OpenSquare)?;
        let alt_text: Token = self.expect(&TokenType::UserString)?;
        let _alt_close_square: Token = self.expect(&TokenType::CloseSquare)?;
        let link_text_open_square: Token = self.expect(&TokenType::OpenSquare)?;
        let link_text: Token = self.expect(&TokenType::UserString)?;
        let _link_text_close_square: Token = self.expect(&TokenType::CloseSquare)?;
        let link_open_square: Token = self.expect(&TokenType::OpenSquare)?;
        let link_url_text: Token = self.expect(&TokenType::UserString)?;
        let _link_close_square: Token = self.expect(&TokenType::CloseSquare)?;
        let _link_close_curly: Token = self.expect(&TokenType::CloseCurly)?;
        let link_str: String = match link_text.value {
            Some(link_str) => link_str,
            None => return Err::<InlineStatement, JiraiErr>(
                JiraiErr::new(
                    &format!(
                        "Expected text at positon \"{}\"!",
                        link_text_open_square.end.to_string()
                    )
                )
            )
        };
        let url_str: String = match link_url_text.value {
            Some(url_str) => url_str,
            None => return Err::<InlineStatement, JiraiErr>(
                JiraiErr::new(
                    &format!(
                        "Expected text at positon \"{}\"!",
                        link_open_square.end.to_string()
                    )
                )
            )
        };
        Ok(InlineStatement::Link(Link{ alt: alt_text.value, url: url_str, link_text: link_str }))
    }

    /// A function to parse inline markup for an
    /// inline image. If the operation is successful the
    /// `Image` variant of the `InlineStatement` 
    /// enumeration is returned. If the operation fails, 
    /// an error is returned.
    pub fn parse_image_item(
        &mut self
    ) -> Result<InlineStatement, JiraiErr>{
        let _image_marker: Token = self.expect(&TokenType::ImageMarker)?;
        let _alt_open_square: Token = self.expect(&TokenType::OpenSquare)?;
        let alt_text: Token = self.expect(&TokenType::UserString)?;
        let _alt_close_square: Token = self.expect(&TokenType::CloseSquare)?;
        let image_open_square: Token = self.expect(&TokenType::OpenSquare)?;
        let image_text: Token = self.expect(&TokenType::UserString)?;
        let _image_close_square: Token = self.expect(&TokenType::CloseSquare)?;
        let _image_close_curly: Token = self.expect(&TokenType::CloseCurly)?;
        let url_str: String = match image_text.value {
            Some(url_str) => url_str,
            None => return Err::<InlineStatement, JiraiErr>(
                JiraiErr::new(
                    &format!(
                        "Expected text at positon \"{}\"!",
                        image_open_square.end.to_string()
                    )
                )
            )
        };
        Ok(InlineStatement::Image(Image{ alt: alt_text.value, url: url_str }))
    }

    /// A function to parse inline markup for
    /// inline text. If the operation is successful the
    /// `Text` variant of the `InlineStatement` 
    /// enumeration is returned. If the operation fails, 
    /// an error is returned.
    pub fn parse_text(
        &mut self
    ) -> Result<InlineStatement, JiraiErr>{
        let token: Token = self.expect(&TokenType::UserString)?;
        let text_str: String = match token.value {
            Some(text_str) => text_str,
            None => return Err::<InlineStatement, JiraiErr>(
                JiraiErr::new(
                    &format!(
                        "Expected text at positon \"{}\"!",
                        token.start.to_string()
                    )
                )
            )
        };
        Ok(InlineStatement::Text(text_str))
    }
}
