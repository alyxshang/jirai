/*
Jirai by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the structure
/// encapsulating information
/// on a parsed link element.
use super::parser::Link;

/// Importing the structure
/// encapsulating information
/// on a parsed image element.
use super::parser::Image;

/// Importing the structure
/// to catch and handle errors.
use super::err::JiraiErr;

/// Importing the enumeration
/// describing all possible types
/// of block-level statements
/// Jirai source code can contain.
use super::parser::Statement;

/// Importing the enumeration
/// describing all possible types
/// of inline statements Jirai source
/// code can contain.
use super::parser::InlineStatement;

/// A structure holding the
/// AST parsed and one to
/// recursively generate HTML
/// code.
pub struct HTMLCodeGenerator{
    pub minify: bool,
    pub cursor: usize,
    pub alt_enforcing: bool,
    pub statements: Vec<Statement>
}

/// Implementing functions
/// for the `HTMLCodeGenerator`
/// structure.
impl HTMLCodeGenerator{

    /// A function to create a new
    /// instance of the `HTMLCodeGenerator`
    /// structure and return it. If the 
    /// supplied AST is empty, an error 
    /// is returned.
    pub fn new(
        minify: &bool,
        alt_enforcing: &bool,
        ast: &Vec<Statement>
    ) -> Result<HTMLCodeGenerator, JiraiErr>{
        if ast.len() == 0{
            return Err::<HTMLCodeGenerator, JiraiErr>(
                JiraiErr::new("The AST cannot be empty.")
            );
        }
        else {
            Ok(HTMLCodeGenerator{
                cursor: 0,
                minify: *minify,
                alt_enforcing: *alt_enforcing,
                statements: ast.to_vec()
            })
        }
    }

    /// This function advances the
    /// internal cursor through the vector
    /// of statements constituting the AST.
    pub fn advance(
        &mut self
    ) -> () {
        self.cursor += 1;
    }

    /// This function checks whether
    /// the internal cursor has reached
    /// the end of the stream of statements
    /// contained in the AST.
    pub fn is_done(
        &self
    ) -> bool {
        &self.statements.len() == &self.cursor
    }

    /// This function attempts to retrieve the current
    /// statement in the stream of statements and return
    /// it. If the operation fails, an error is returned.
    pub fn current(
        &self
    ) -> Result<Statement, JiraiErr>{
        let stmt: Statement = match self.statements.get(self.cursor){
            Some(stmt) => stmt.clone(),
            None => return Err::<Statement, JiraiErr>(
                JiraiErr::new("Unexpected end of statement stream.")
            )
        };
        Ok(stmt)
    }

    /// This function generates HTML code
    /// from the statements inside the AST
    /// and returns it. If the operation fails,
    /// an error is returned.
    pub fn generate(
        &mut self
    ) -> Result<String, JiraiErr> {
        let mut lines: Vec<String> = Vec::new();
        while !self.is_done(){
            let current: Statement = self.current()?;
            match current {
                Statement::Paragraph(i_statements) => lines.push(
                    self.generate_paragraph_code(&i_statements)?),
                Statement::UnorderedList(i_statements) => lines.push(
                    self.generate_unordered_list_code(&i_statements)?),
                Statement::Heading(level, i_statements) => lines.push(
                    self.generate_heading_code(&level, &i_statements)?)
            };
        }
        if self.minify{
            Ok(lines.join("\n"))
        }
        else {
            Ok(lines.into_iter().collect::<String>())
        }
    }

    /// The function to generate the HTML code
    /// for a heading and nested elements 
    /// and return it. If the operation fails, an 
    /// error is returned.
    pub fn generate_heading_code(
        &mut self,
        level: &usize,
        inline_statements: &Vec<InlineStatement>
    ) -> Result<String, JiraiErr> {
        let mut lines: Vec<String> = Vec::new();
        for i_statement in inline_statements{
            lines.push(self.generate_inline_statement(&i_statement)?);
        }
        let joined: String;
        if self.minify{
            joined = lines.into_iter().collect::<String>();
        }
        else {
            joined = lines.join("\n");
        }
        Ok(format!("<h{}>{}</h{}>", level, joined, level))
    }

    /// The function to generate the HTML code
    /// for a paragraph and nested elements 
    /// and return it. If the operation fails, an 
    /// error is returned.
    pub fn generate_paragraph_code(
        &mut self,
        inline_statements: &Vec<InlineStatement>
    ) -> Result<String, JiraiErr> {
        let mut lines: Vec<String> = Vec::new();
        for i_statement in inline_statements{
            lines.push(self.generate_inline_statement(&i_statement)?);
        }
        let joined: String;
        if self.minify{
            joined = lines.into_iter().collect::<String>();
        }
        else {
            joined = lines.join("\n");
        }
        Ok(format!("<p>{}</p>", joined))
    }

    /// The function to generate the HTML code
    /// for an unordered list and nested elements 
    /// and return it. If the operation fails, an 
    /// error is returned.
    pub fn generate_unordered_list_code(
        &mut self,
        inline_statements: &Vec<InlineStatement>
    ) -> Result<String, JiraiErr> {
        let mut lines: Vec<String> = Vec::new();
        for i_statement in inline_statements{
            lines.push(self.generate_inline_statement(&i_statement)?);
        }
        let joined: String;
        if self.minify{
            joined = lines.into_iter().collect::<String>();
        }
        else {
            joined = lines.join("\n");
        }
        Ok(format!("<ul>{}</ul>", joined))
    }

    /// The function to generate the HTML code
    /// for an inline element and nested elements 
    /// and return it. If the operation fails, an 
    /// error is returned.
    pub fn generate_inline_statement(
        &mut self,
        inline_statement: &InlineStatement
    ) -> Result<String, JiraiErr> {
        match inline_statement{
            InlineStatement::Text(text) => Ok(text.to_string()),
            InlineStatement::Code(code) => Ok(self.generate_code_code(&code)),
            InlineStatement::Link(link) => Ok(self.generate_link_code(&link)?),
            InlineStatement::Image(image) => Ok(self.generate_image_code(&image)?),
            InlineStatement::BoldText(nested) => Ok(self.generate_italic_code(&nested)?),
            InlineStatement::ItalicText(nested) => Ok(self.generate_italic_code(&nested)?),
            InlineStatement::ListItem(nested) => Ok(self.generate_list_item_code(&nested)?),
            InlineStatement::BlockQuote(quote) => Ok(self.generate_block_quote_code(&quote))
        }
    }

    /// The function to generate the HTML code
    /// for an inline image and return it. If the
    /// operation fails, an error is returned.
    pub fn generate_image_code(
        &mut self,
        image: &Image
    ) -> Result<String, JiraiErr> {
        match &image.alt{
            Some(alt_text) => Ok(
                format!(
                    "<img alt=\"{}\" src=\"{}\"/>", 
                    alt_text, 
                    image.url
                )
            ),
            None => {
                if self.alt_enforcing{
                    Err::<String, JiraiErr>(
                        JiraiErr::new(
                            &format!(
                                "No \"alt\" text supplied to image \"{}\"!",
                                &image.url
                            )
                        )
                    )
                }
                else{
                    Ok(
                        format!(
                            "<img src=\"{}\"/>",
                            image.url
                        )
                    )
                }
            }
        }
    }

    /// The function to generate the HTML code
    /// for an inline link and return it. If the
    /// operation fails, an error is returned.
    pub fn generate_link_code(
        &mut self,
        link: &Link
    ) -> Result<String, JiraiErr> {
        match &link.alt{
            Some(alt_text) => Ok(
                format!(
                    "<a alt=\"{}\" href=\"{}\">{}</a>", 
                    alt_text, 
                    link.url,
                    link.link_text
                )
            ),
            None => {
                if self.alt_enforcing{
                    Err::<String, JiraiErr>(
                        JiraiErr::new(
                            &format!(
                                "No \"alt\" text supplied to link \"{}\"!",
                                &link.url
                            )
                        )
                    )
                }
                else {
                    Ok(
                        format!(
                            "<a href=\"{}\">{}</a>", 
                            link.url,
                            link.link_text
                        )
                    )
                }
            }   
        }
    }

    /// The function to generate the HTML code
    /// for inline code and return it. 
    pub fn generate_code_code(
        &mut self,
        code: &str
    ) -> String {
        format!("<code>{}</code>", code)
    }

    /// The function to generate the HTML code
    /// for block quote and return it. 
    pub fn generate_block_quote_code(
        &mut self,
        quote: &str
    ) -> String {
        format!("<blockquote>{}</blockquote>", quote)
    }

    /// The function to generate the HTML code
    /// for italic text and return it. 
    pub fn generate_italic_code(
        &mut self,
        inline_statements: &Vec<InlineStatement>
    ) -> Result<String, JiraiErr> {
        let mut lines: Vec<String> = Vec::new();
        for inline_statement in inline_statements {
            lines.push(self.generate_inline_statement(&inline_statement)?);
        }
        let joined: String;
        if self.minify{
            joined = lines.into_iter().collect::<String>();
        }
        else {
            joined = lines.join("\n");
        }
        Ok(format!("<i>{}</i>", joined))
    }

    /// The function to generate the HTML code
    /// for bold text and return it. 
    pub fn generate_bold_code(
        &mut self,
        inline_statements: &Vec<InlineStatement>
    ) -> Result<String, JiraiErr> {
        let mut lines: Vec<String> = Vec::new();
        for inline_statement in inline_statements {
            lines.push(self.generate_inline_statement(&inline_statement)?);
        }
        let joined: String;
        if self.minify{
            joined = lines.into_iter().collect::<String>();
        }
        else {
            joined = lines.join("\n");
        }
        Ok(format!("<b>{}</b>", joined))
    }

    /// The function to generate the HTML code
    /// for a list item and return it. 
    pub fn generate_list_item_code(
        &mut self,
        inline_statements: &Vec<InlineStatement>
    ) -> Result<String, JiraiErr> {
        let mut lines: Vec<String> = Vec::new();
        for inline_statement in inline_statements {
            lines.push(self.generate_inline_statement(&inline_statement)?);
        }
        let joined: String;
        if self.minify{
            joined = lines.into_iter().collect::<String>();
        }
        else {
            joined = lines.join("\n");
        }
        Ok(format!("<li>{}</li>", joined))
    }
}
