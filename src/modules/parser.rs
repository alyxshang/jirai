use super::lexer::Token;
use super::err::JiraiErr;
use super::lexer::TokenType;
#[derive(PartialEq, Debug)]
pub enum Statement{
    Heading(Vec<InlineStatement>),
    Paragraph(Vec<InlineStatement>),
    UnorderedList(Vec<InlineStatement>)
}
#[derive(PartialEq, Debug)]
pub enum InlineStatement{
    Code(Token),
    Text(Token),
    Link(Token, Token),
    Image(Token, Token),
    ListItem(Vec<InlineStatement>),
    BoldText(Box<InlineStatement>),
    ItalicText(Box<InlineStatement>),
}
pub struct Parser{
    pub cursor: usize,
    pub stream: Vec<Token>,
}
impl Parser {
    pub fn new(
        stream: &Vec<Token>
    ) -> Parser {
        Parser {
            cursor: 0,
            stream: stream.clone()
        }
    }
    pub fn advance(
        &mut self
    ) -> () {
        self.cursor += 1;
    }
    pub fn is_done(
        &self
    ) -> bool {
        &self.cursor == &self.stream.len()
    }
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
                self.advance();
            }
            Ok(Statement::Heading(stmt_vec))
        }
    }
    pub fn parse_block_element(
        &mut self
    ) -> Result<Statement, JiraiErr>{
        let peeked: Token = self.peek()?;
        match peeked.token_type{
            TokenType::ListMarker => Ok(self.parse_unordered_list()?),
            _ => Ok(self.parse_paragraph()?)
        }
    }
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
            self.advance();
        }
        Ok(Statement::Paragraph(stmt_vec))
    }
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
            self.advance();
        }
        Ok(Statement::UnorderedList(stmt_vec))
    }
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
            _ => Ok(self.parse_text()?)
        }
    }
    pub fn parse_linked_item(
        &mut self
    ) -> Result<InlineStatement, JiraiErr>{
        let peeked: Token = self.peek()?;
        match peeked.token_type{
            TokenType::ImageMarker => Ok(self.parse_image_item()?),
            TokenType::LinkMarker => Ok(self.parse_link_item()?),
            _ => Err::<InlineStatement, JiraiErr>(
                JiraiErr::new(
                    &format!(
                        "Expected a link or image marker at position \"{}\"!", 
                        &peeked.end.to_string()
                    )
                )
            )
        }
    }
    /*pub fn parse_bold_text(
        &mut self
    ) -> Result<InlineStatement, JiraiErr>{

    }
    pub fn parse_italic_text(
        &mut self
    ) -> Result<InlineStatement, JiraiErr>{

    } 
    pub fn parse_list_item(
        &mut self
    ) -> Result<InlineStatement, JiraiErr>{

    }*/
    pub fn parse_inline_code(
        &mut self
    ) -> Result<InlineStatement, JiraiErr>{
        let _open_angle: Token = self.expect(&TokenType::OpenAngle)?;
        let code_text: Token = self.expect(&TokenType::UserString)?;
        let _close_angle: Token = self.expect(&TokenType::CloseAngle)?;
        Ok(InlineStatement::Code(code_text))

    }
    pub fn parse_link_item(
        &mut self
    ) -> Result<InlineStatement, JiraiErr>{
        let _link_marker: Token = self.expect(&TokenType::LinkMarker)?;
        let _alt_open_square: Token = self.expect(&TokenType::OpenSquare)?;
        let alt_text: Token = self.expect(&TokenType::UserString)?;
        let _alt_close_square: Token = self.expect(&TokenType::CloseSquare)?;
        let _link_open_square: Token = self.expect(&TokenType::OpenSquare)?;
        let link_text: Token = self.expect(&TokenType::UserString)?;
        let _link_close_square: Token = self.expect(&TokenType::CloseSquare)?;
        let _link_close_curly: Token = self.expect(&TokenType::CloseCurly)?;
        Ok(InlineStatement::Link(alt_text, link_text))
    }
    pub fn parse_image_item(
        &mut self
    ) -> Result<InlineStatement, JiraiErr>{
        let _image_marker: Token = self.expect(&TokenType::ImageMarker)?;
        let _alt_open_square: Token = self.expect(&TokenType::OpenSquare)?;
        let alt_text: Token = self.expect(&TokenType::UserString)?;
        let _alt_close_square: Token = self.expect(&TokenType::CloseSquare)?;
        let _image_open_square: Token = self.expect(&TokenType::OpenSquare)?;
        let image_text: Token = self.expect(&TokenType::UserString)?;
        let _image_close_square: Token = self.expect(&TokenType::CloseSquare)?;
        let _image_close_curly: Token = self.expect(&TokenType::CloseCurly)?;
        Ok(InlineStatement::Image(alt_text, image_text))
    }
    pub fn parse_text(
        &mut self
    ) -> Result<InlineStatement, JiraiErr>{
        let token: Token = self.expect(&TokenType::UserString)?;
        Ok(InlineStatement::Text(token))
    }

}
