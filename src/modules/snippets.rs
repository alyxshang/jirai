/// A data structure to
/// encapsulate data about
/// a heading parsed from
/// Jirai source code.
#[derive(PartialEq, Debug)]
pub struct Heading{
    pub level: usize,
    pub contents: Vec<Token>
}

/// A data structure to
/// encapsulate data about
/// a paragraph parsed from
/// Jirai source code.
#[derive(PartialEq, Debug)]
pub struct Paragraph{
    pub contents: Vec<Token>
}

/// A data structure to
/// encapsulate data about
/// an unordered list parsed 
/// from Jirai source code.
#[derive(PartialEq, Debug)]
pub struct UnorderedList{
    pub item_count: usize,
    pub list_items: Vec<Token>
}

/// A data structure
/// to encapsulate data
/// about a link item.
#[derive(PartialEq, Debug)]
pub struct LinkItem{
    pub alt: Option<String>,
    pub link: String
}

/// A data structure
/// to encapsulate data
/// about a link item.
#[derive(PartialEq, Debug)]
pub struct ImageItem{
    pub alt: Option<String>,
    pub link: String
}

/// A data structure
/// to encapsulate data
/// about an inline-code
/// item.
#[derive(PartialEq, Debug)]
pub struct InlineCode{
    pub code: String,
}
pub fn parse_block_element(
        &mut self
    ) -> Result<Statement, JiraiErr>{
        let peeked: Token = self.peek_ahead()?;
        match peeked.token_type{
            TokenType::ListMarker => Ok(self.parse_unordered_list()?),
            _ => Ok(self.parse_paragraph()?)
        }
    }
 
    /*pub fn parse_list_item(
        &mut self
    ) -> Result<InlineStatement, JiraiErr>{
        let _open_list_item: Token = self.expect(&TokenType::ListMarker)?;
        let mut stmts: Vec<InlineStatement> = Vec::new();
        while !self.is_done(){
            let peeked: Token = self.peek_ahead()?;
            if peeked.token_type == TokenType::NewLine{
                break;
            }
            else {
                let further: InlineStatement = self.parse_inline_statement()?; 
                stmts.push(further);
            }
        }
        let _close_bold: Token = self.expect(&TokenType::NewLine)?; 
        Ok(InlineStatement::ListItem(Box::new(stmts)))
    }*/

    /*pub fn parse_inline_statement(
        &mut self
    ) -> Result<InlineStatement, JiraiErr>{
        let peeked: Token = self.peek_ahead()?; 
        match peeked.token_type{
            TokenType::ItalicText => {
                let stmt: InlineStatement = self.parse_italic_text()?; 
                Ok(stmt)
            },
            TokenType::BoldText => {
                let stmt: InlineStatement = self.parse_bold_text()?; 
                Ok(stmt)
            },
            TokenType::OpenAngle => {
                let stmt: InlineStatement = self.parse_code_statement()?;
                Ok(stmt)
            },
            TokenType::OpenCurly => {
               let stmt: InlineStatement = self.parse_linked_statement()?;
                Ok(stmt)
            },
            TokenType::UserString => {
                let stmt: InlineStatement = self.parse_user_content()?;
                Ok(stmt)
            },
            _ => Err::<InlineStatement, JiraiErr>(
                JiraiErr::new("Unexpected token!")
            )
        }
    }*/

    /*pub fn parse_linked_statement(
        &mut self
    ) -> Result<InlineStatement, JiraiErr>{
        self.advance();
        let peeked: Token =  self.peek_ahead()?;
        match peeked.token_type{
            TokenType::LinkMarker => Ok(self.parse_link_statement()?),
            TokenType::ImageMarker => Ok(self.parse_image_statement()?),
            _ => Err::<InlineStatement, JiraiErr>(
                JiraiErr::new("Unexpected token.")
            )
        }
    }*/

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
        if level != 0{
            let mut statements: Vec<Token> = Vec::new();
            while let Some(token) = self.stream.get(self.cursor){
                if token.token_type == TokenType::NewLine{
                    self.advance();
                    break;
                }
                //let inline_statements = self.parse_inline_statement()?;
                statements.push(self.peek_ahead()?);
            }
            Ok(Statement::Heading( Heading { level: level, contents: statements }))
        }
        else {
            return Err::<Statement, JiraiErr>(
                JiraiErr::new("No heading token encountered!")
            );
        }
    }

    pub fn parse_unordered_list(
        &mut self
    ) -> Result<Statement, JiraiErr>{
        let peeked: Token = self.peek_ahead()?;
        let mut list_items: Vec<InlineStatement> = Vec::new();
        if peeked.token_type == TokenType::NewLine{
            while let Some(token) = self.stream.get(self.cursor){
                if token.token_type == TokenType::ListMarker{
                    list_items.push(self.parse_list_item()?);
                }
                else {
                    return Err::<Statement, JiraiErr>(
                        JiraiErr::new("Encountered an unexpected token.")
                    );
                }
            }
            Ok(
                Statement::UnorderedList( 
                    UnorderedList { item_count: list_items.len(), list_items: list_items }
                )
            )
        }
        else {
            return Err::<Statement, JiraiErr>(
                JiraiErr::new("Encountered an unexpected token.")
            );
        }
    }


    pub fn parse_paragraph(
        &mut self
    ) -> Result<Statement, JiraiErr>{
        let peeked: Token = self.peek_ahead()?;
        if peeked.token_type == TokenType::NewLine{
            let mut statements: Vec<InlineStatement> = Vec::new();
            while let Some(token) = self.stream.get(self.cursor){
                if token.token_type == TokenType::NewLine{
                    break;
                }
                let inline_statement: InlineStatement = self.parse_inline_statement()?;
                statements.push(inline_statement);
            }
        Ok(Statement::Paragraph( Paragraph { contents: statements }))
        }
        else {
            return Err::<Statement, JiraiErr>(
                JiraiErr::new("Encountered an unexpected token.")
            );
        }
    }

    // A function to parse an inline-link statement.
    // If the operation is successful, the `LinkItem`
    // variant of the `InlineStatement` enumeration is returned,
    // containing an instance of the `LinkItem` structure.
    // If the operation fails, an error is returned.
    /*pub fn parse_link_statement(
        &mut self
    ) -> Result<InlineStatement, JiraiErr>{
        let _link_open_curly: Token = self.expect(&TokenType::OpenCurly)?; 
        let _link_marker: Token = self.expect(&TokenType::LinkMarker)?;
        let _alt_open_square: Token = self.expect(&TokenType::OpenSquare)?;
        let alt_text: Token = self.expect(&TokenType::UserString)?;
        let _alt_close_square: Token = self.expect(&TokenType::CloseSquare)?;
        let _link_open_square: Token = self.expect(&TokenType::OpenSquare)?;
        let link_url: Token = self.expect(&TokenType::UserString)?;
        let _link_close_square: Token = self.expect(&TokenType::CloseSquare)?;
        let _link_close_curly: Token = self.expect(&TokenType::CloseCurly)?;
        let link_url_text: String = match link_url.value{
            Some(link_url_text) => link_url_text,
            None => return Err::<InlineStatement, JiraiErr>(
                JiraiErr::new("No link URL received.")
            )
        };
        Ok(InlineStatement::LinkItem(LinkItem { alt: alt_text.value, link: link_url_text }))
    }*/

    // A function to parse an inline-image statement.
    // If the operation is successful, the `ImageItem`
    // variant of the `InlineStatement` enumeration is returned,
    // containing an instance of the `ImageItem` structure.
    // If the operation fails, an error is returned.
    /*pub fn parse_image_statement(
        &mut self
    ) -> Result<InlineStatement, JiraiErr>{
        let _image_open_curly: Token = self.expect(&TokenType::OpenCurly)?;
        let _image_marker: Token = self.expect(&TokenType::ImageMarker)?;
        let _alt_open_square: Token = self.expect(&TokenType::OpenSquare)?;
        let alt_text: Token = self.expect(&TokenType::UserString)?;
        let _alt_close_square: Token = self.expect(&TokenType::CloseSquare)?;
        let _image_open_square: Token = self.expect(&TokenType::OpenSquare)?;
        let image_url: Token = self.expect(&TokenType::UserString)?;
        let _image_close_square: Token = self.expect(&TokenType::CloseSquare)?;
        let _image_close_curly: Token = self.expect(&TokenType::CloseCurly)?;
        let image_url_text: String = match image_url.value{
            Some(image_url_text) => image_url_text,
            None => return Err::<InlineStatement, JiraiErr>(
                JiraiErr::new("No image URL received.")
            )
        };
        Ok(
            InlineStatement::ImageItem(
                ImageItem { 
                    alt: alt_text.value, 
                    link: image_url_text 
                }
            )
        )
    }*/

    // A function to parse an inline-code statement.
    // If the operation is successful, the `CodeStatement`
    // variant of the `InlineStatement` enumeration is returned,
    // containing an instance of the `InlineCode` structure.
    // If the operation fails, an error is returned.
    /*pub fn parse_code_statement(
        &mut self
    ) -> Result<InlineStatement, JiraiErr>{
        let _code_open_angle: Token = self.expect(&TokenType::OpenAngle)?; 
        let code_text: Token = self.expect(&TokenType::UserString)?;
        let _code_close_angle: Token = self.expect(&TokenType::CloseAngle)?;
        let code: String = match code_text.value{
            Some(code) => code,
            None => return Err::<InlineStatement, JiraiErr>(
                JiraiErr::new("Expected code.")
            )
        };
        Ok(
            InlineStatement::CodeStatement(
                InlineCode{ 
                    code: code
                }
            )
        )
    }*/

    // A function to parse an inline statement containing text.
    // If the operation is successful, the `UserText`
    // variant of the `InlineStatement` enumeration is returned,
    // containing a string with the text. If the operation fails, 
    // an error is returned.
    /*pub fn parse_user_content(
        &mut self
    ) -> Result<InlineStatement, JiraiErr>{
        let user_content_token: Token = self.expect(&TokenType::UserString)?; 
        let user_text: String = match user_content_token.value{
            Some(user_text) => user_text,
            None => return Err::<InlineStatement, JiraiErr>(
                JiraiErr::new("Encountered an empty string.")
            )
        };
        Ok(
            InlineStatement::UserText(user_text)
        )
    }*/

    // A function to parse an inline statement containing italic 
    // text. If the operation is successful, the `ItalicText`
    // variant of the `InlineStatement` enumeration is returned,
    // containing a vector of other possible inline statements.
    // If the operation fails, an error is returned.
    /*pub fn parse_italic_text(
        &mut self
    ) -> Result<InlineStatement, JiraiErr>{
        let _open_italic: Token = self.expect(&TokenType::ItalicText)?;
        let mut stmts: Vec<InlineStatement> = Vec::new();
        while !self.is_done(){
            let peeked: Token = self.peek_ahead()?;
            if peeked.token_type == TokenType::ItalicText{
                break;
            }
            else {
                let further: InlineStatement = self.parse_inline_statement()?; 
                stmts.push(further);
            }
            self.advance();
        }
        let _close_bold: Token = self.expect(&TokenType::ItalicText)?; 
        Ok(
            InlineStatement::ItalicText(
                Box::new(stmts)
            )
        )
    }*/

    // A function to parse an inline statement containing bold 
    // text. If the operation is successful, the `BoldText`
    // variant of the `InlineStatement` enumeration is returned,
    // containing a vector of other possible inline statements.
    // If the operation fails, an error is returned.
    /*pub fn parse_bold_text(
        &mut self
    ) -> Result<InlineStatement, JiraiErr>{
        let _open_bold: Token = self.expect(&TokenType::BoldText)?;
        let mut stmts: Vec<InlineStatement> = Vec::new();
        while !self.is_done(){
            let peeked: Token = self.peek_ahead()?;
            if peeked.token_type == TokenType::BoldText{
                break;
            }
            else {
                let further: InlineStatement = self.parse_inline_statement()?; 
                stmts.push(further);
            }
            self.advance();
        }
        let _close_bold: Token = self.expect(&TokenType::BoldText)?; 
        Ok(
            InlineStatement::BoldText(
                Box::new(stmts)
            )
        )
    }*/

// An enumeration listing
// all possible types of
// atomic statements.
//#[derive(PartialEq, Debug)]
/*pub enum InlineStatement{
    UserText(String), // done.
    LinkItem(LinkItem), // done.
    ImageItem(ImageItem), // done.
    CodeStatement(InlineCode), // done.
    ListItem(Box<Vec<InlineStatement>>), // done.
    BoldText(Box<Vec<InlineStatement>>), // done.
    ItalicText(Box<Vec<InlineStatement>>), // done.
}*/
