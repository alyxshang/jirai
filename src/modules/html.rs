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
    /// and returns it.
    pub fn generate(
        &mut self
    ) -> Result<String, JiraiErr> {
        let mut lines: Vec<String> = Vec::new();
        while !self.is_done(){
            let current: Statement = self.current()?;
            match current {
                Statement::Paragraph(i_statements) => lines.push(
                    self.generate_paragraph_code(&i_statements)),
                Statement::UnorderedList(i_statements) => lines.push(
                    self.generate_unordered_list_code(&i_statements)),
                Statement::Heading(level, i_statements) => lines.push(
                    self.generate_heading_code(&level, &i_statements))
            };
        }
        if self.minify{
            Ok(lines.join("\n"))
        }
        else {
            Ok(lines.into_iter().collect::<String>())
        }
    }

    pub fn generate_heading_code(
        &mut self,
        level: &usize,
        inline_statements: &Vec<InlineStatement>
    ) -> String {
        let mut lines: Vec<String> = Vec::new();
        for i_statement in inline_statements{
            lines.push(self.generate_inline_statement(&i_statement));
        }
        let joined: String;
        if self.minify{
            joined = lines.into_iter().collect::<String>();
        }
        else {
            joined = lines.join("\n");
        }
        format!("<h{}>{}</h{}>", level, joined, level)
    }

    pub fn generate_paragraph_code(
        &mut self,
        inline_statements: &Vec<InlineStatement>
    ) -> String {
        let mut lines: Vec<String> = Vec::new();
        for i_statement in inline_statements{
            lines.push(self.generate_inline_statement(&i_statement));
        }
        let joined: String;
        if self.minify{
            joined = lines.into_iter().collect::<String>();
        }
        else {
            joined = lines.join("\n");
        }
        format!("<p>{}</p>", joined)
    }

    pub fn generate_unordered_list_code(
        &mut self,
        inline_statements: &Vec<InlineStatement>
    ) -> String {
        let mut lines: Vec<String> = Vec::new();
        for i_statement in inline_statements{
            lines.push(self.generate_inline_statement(&i_statement));
        }
        let joined: String;
        if self.minify{
            joined = lines.into_iter().collect::<String>();
        }
        else {
            joined = lines.join("\n");
        }
        format!("<ul>{}</ul>", joined)
    }

    pub fn generate_inline_statement(
        &mut self,
        inline_statement: &InlineStatement
    ) -> String {
        match inline_statement{
            InlineStatement::Text(text) => text.to_string(),
            InlineStatement::Code(code) => self.generate_code_code(&code),
            InlineStatement::Link(link) => self.generate_link_code(&link),
            InlineStatement::Image(image) => self.generate_image_code(&image),
            InlineStatement::BoldText(nested) => self.generate_italic_code(&nested),
            InlineStatement::ItalicText(nested) => self.generate_italic_code(&nested),
            InlineStatement::ListItem(nested) => self.generate_list_item_code(&nested),
            InlineStatement::BlockQuote(quote) => self.generate_block_quote_code(&quote)
        }
    }

    pub fn generate_image_code(
        &mut self,
        image: &Image
    ) -> String {
        match &image.alt{
            Some(alt_text) => format!(
                "<img alt=\"{}\" src=\"{}\"/>", 
                alt_text, 
                image.url
            ),
            None => format!(
                "<img src=\"{}\"/>",
                image.url
            )
        }
    }

    pub fn generate_link_code(
        &mut self,
        link: &Link
    ) -> String {
        match &link.alt{
            Some(alt_text) => format!(
                "<a alt=\"{}\" href=\"{}\">{}</a>", 
                alt_text, 
                link.url,
                link.link_text
            ),
            None => format!(
                "<a href=\"{}\">{}</a>", 
                link.url,
                link.link_text
            )
        }
    }

    pub fn generate_code_code(
        &mut self,
        code: &str
    ) -> String {
        format!("<code>{}</code>", code)
    }

    pub fn generate_block_quote_code(
        &mut self,
        quote: &str
    ) -> String {
        format!("<blockquote>{}</blockquote>", quote)
    }

    pub fn generate_italic_code(
        &mut self,
        inline_statements: &Vec<InlineStatement>
    ) -> String {
        let mut lines: Vec<String> = Vec::new();
        for inline_statement in inline_statements {
            lines.push(self.generate_inline_statement(&inline_statement));
        }
        let joined: String;
        if self.minify{
            joined = lines.into_iter().collect::<String>();
        }
        else {
            joined = lines.join("\n");
        }
        format!("<i>{}</i>", joined)
    }

    pub fn generate_bold_code(
        &mut self,
        inline_statements: &Vec<InlineStatement>
    ) -> String {
        let mut lines: Vec<String> = Vec::new();
        for inline_statement in inline_statements {
            lines.push(self.generate_inline_statement(&inline_statement));
        }
        let joined: String;
        if self.minify{
            joined = lines.into_iter().collect::<String>();
        }
        else {
            joined = lines.join("\n");
        }
        format!("<b>{}</b>", joined)
    }

    pub fn generate_list_item_code(
        &mut self,
        inline_statements: &Vec<InlineStatement>
    ) -> String {
        let mut lines: Vec<String> = Vec::new();
        for inline_statement in inline_statements {
            lines.push(self.generate_inline_statement(&inline_statement));
        }
        let joined: String;
        if self.minify{
            joined = lines.into_iter().collect::<String>();
        }
        else {
            joined = lines.join("\n");
        }
        format!("<li>{}</li>", joined)
    }



}
