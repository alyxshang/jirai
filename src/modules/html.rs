/*
Jirai by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the structure
/// to catch and handle errors.
use super::err::JiraiErr;

/// Importing the enumeration
/// "listing" all the possible
/// block statements in
/// Jirai source code.
use super::parser::BlockStatement;

/// Importing the enumeration
/// "listing" all the possible
/// inline statements in
/// Jirai source code.
use super::parser::AtomicStatement;

/// A data structure to hold
/// the parsed syntax tree
/// and keep an internal
/// cursor to keep track
/// of the current node in
/// the syntax tree.
pub struct HTMLGenerator{
    pub cursor: usize,
    pub stmts: Vec<BlockStatement>
}

/// Implementing functions
/// for the `HTMLGenerator`
/// structure.
impl HTMLGenerator{

    /// A function to create a new
    /// instance of the `HTMLGenerator`
    /// structure and return it.
    /// If the stream of block statements 
    /// is empty, an error is returned.
    pub fn new(
        stmts: &Vec<BlockStatement>
    ) -> Result<HTMLGenerator, JiraiErr>{
        if stmts.is_empty(){
            Err::<HTMLGenerator, JiraiErr>(
                JiraiErr::new("Statement vector cannot be empty.")
            )
        }
        else {
            Ok(HTMLGenerator{
                cursor: 0,
                stmts: stmts.clone()
            })
        }
    }

    /// A function to check whether
    /// the cursor has reached the 
    /// end of the stream of block-
    /// statements. A boolean is
    /// returned to reflect this.
    pub fn is_done(
        &self
    ) -> bool {
        self.cursor == self.stmts.len()
    }

    /// A function to advance
    /// the internal cursor 
    /// by one.
    pub fn advance(
        &mut self
    ){
        self.cursor += 1;
    }

    /// A function to generate HTML
    /// code from the internal vector
    /// of instances of the `BlockStatement`
    /// enumeration. If the operation is
    /// successful, a string containing this
    /// HTML code is returned. If the operation
    /// fails, an error is returned.
    pub fn generate(
        &mut self
    ) -> Result<String, JiraiErr>{
        let mut strings: Vec<String> = Vec::new();
        while !self.is_done(){
            strings.push(
                self.generate_block_statement_code()?
            );
            self.advance();
        }
        Ok(strings.into_iter().collect::<String>())
    }

    /// A function to generate HTML
    /// code for a single variant
    /// of the `BlockStatement` 
    /// enumeration. The HTML code is 
    /// returned as a string. If the 
    /// operation fails, an error is
    /// returned.
    pub fn generate_block_statement_code(
        &mut self
    ) -> Result<String, JiraiErr>{
        let current: BlockStatement = match self.stmts.get(
            self.cursor
        ){
            Some(current) => current.clone(),
            None => return Err::<String, JiraiErr>(
                JiraiErr::new("Unexpected end of statement stream.")
            )
        };
        match current{
            BlockStatement::Comment => Ok("".to_string()),
            BlockStatement::Heading(level, contents) => Ok(
                self.generate_heading_code(&level, &contents)
            ),
            BlockStatement::Paragraph(contents) => Ok(
                self.generate_paragraph_code(&contents)
            ),
            BlockStatement::UnorderedList(contents) => Ok(
                self.generate_unordered_list_code(&contents)?
            ), 
        }
    }

    /// A function to generate HTML
    /// code for the `Heading` 
    /// variant of the `BlockStatement` 
    /// enumeration. The HTML code is 
    /// returned as a string.
    pub fn generate_heading_code(
        &self,
        level: &usize,
        contents: &Vec<AtomicStatement>
    ) -> String {
        let mut strings: Vec<String> = Vec::new();
        for a_stmt in contents{
            strings.push(self.generate_atomic_code(a_stmt));
        }
        let joined: String = strings
            .into_iter()
            .collect::<String>();
        format!("<h{}>{}</h{}>", level, joined, level)
    }

    /// A function to generate HTML
    /// code for the `Paragraph` 
    /// variant of the `BlockStatement` 
    /// enumeration. The HTML code is 
    /// returned as a string.
    pub fn generate_paragraph_code(
        &self,
        contents: &Vec<AtomicStatement>
    ) -> String {
        let mut strings: Vec<String> = Vec::new();
        for a_stmt in contents{
            strings.push(self.generate_atomic_code(a_stmt));
        }
        let joined: String = strings
            .into_iter()
            .collect::<String>();
        format!("<p>{}</p>", joined)
    }

    /// A function to generate HTML
    /// code for the `UnorderedList` 
    /// variant of the `BlockStatement` 
    /// enumeration. If the operation
    /// is successful, the HTML code is 
    /// returned as a string. If the 
    /// operation fails, an error is
    /// returned.
    pub fn generate_unordered_list_code(
        &self,
        contents: &Vec<AtomicStatement>
    ) -> Result<String, JiraiErr>{
        let mut strings: Vec<String> = Vec::new();
        for a_stmt in contents{
            match a_stmt{
                AtomicStatement::ListItem(_contents) => strings.push(
                    self.generate_atomic_code(a_stmt)
                ),
                _ => return Err::<String, JiraiErr>(
                    JiraiErr::new("Unexpected statement type.")
                )
            };
        }
        let joined: String = strings
            .into_iter()
            .collect::<String>();
        Ok(format!("<ul>{}</ul>", joined)) 
    } 

    /// A function to generate HTML
    /// code for a single variant of the
    /// `AtomicStatement` enumeration.
    /// The HTML code is returned as a string. 
    pub fn generate_atomic_code(
        &self,
        statement: &AtomicStatement
    ) -> String{
        match statement{
            AtomicStatement::Link(alt,link,url) => 
                self.generate_link_code(alt, link, url),
            AtomicStatement::InlineCode(contents) =>
                self.generate_inline_code_code(contents),
            AtomicStatement::ImageLink(alt,url) => 
                self.generate_image_code(alt, url),
            AtomicStatement::ListItem(contents) => 
                self.generate_list_item_code(contents),
            AtomicStatement::Text(contents) => 
                contents.clone(),
            AtomicStatement::BoldText(contents) => 
                self.generate_bold_code(contents),
            AtomicStatement::ItalicText(contents) => 
                self.generate_italic_code(contents),
        }
    }

    /// A function to generate HTML
    /// code for the `InlineCode` variant
    /// of the `AtomicStatement` enumeration.
    /// The HTML code is returned as a string. 
    pub fn generate_inline_code_code(
        &self,
        code: &str
    ) -> String {
        format!("<code>{}</code>", code)
    }

    /// A function to generate HTML
    /// code for the `BoldText` variant
    /// of the `AtomicStatement` enumeration.
    /// The HTML code is returned as a string. 
    pub fn generate_bold_code(
        &self,
        contents: &Box<Vec<AtomicStatement>>
    ) -> String {
        let mut strings: Vec<String> = Vec::new();
        for a_stmt in contents.iter(){
            strings.push(self.generate_atomic_code(a_stmt));
        }
        let joined: String = strings
            .into_iter()
            .collect::<String>();
        format!("<strong>{}</strong>", joined)
    }

    /// A function to generate HTML
    /// code for the `ItalicText` variant
    /// of the `AtomicStatement` enumeration.
    /// The HTML code is returned as a string. 
    pub fn generate_italic_code(
        &self,
        contents: &Box<Vec<AtomicStatement>>
    ) -> String {
        let mut strings: Vec<String> = Vec::new();
        for a_stmt in contents.iter(){
            strings.push(self.generate_atomic_code(a_stmt));
        }
        let joined: String = strings
            .into_iter()
            .collect::<String>();
        format!("<i>{}</i>", joined)
    }

    /// A function to generate HTML
    /// code for the `Link` variant
    /// of the `AtomicStatement` enumeration.
    /// The HTML code is returned as a string. 
    pub fn generate_link_code(
        &self,
        alt: &str,
        link: &str,
        url: &str
    ) -> String {
        format!(
            "<a alt=\"{}\" href=\"{}\">{}</a>",
            alt,
            url,
            link
        )
    }

    /// A function to generate HTML
    /// code for the `ImageLink` variant
    /// of the `AtomicStatement` enumeration.
    /// The HTML code is returned as a string. 
    pub fn generate_image_code(
        &self,
        alt: &str,
        url: &str
    ) -> String {
        format!(
            "<img alt=\"{}\" src=\"{}\"/>",
            alt,
            url,
        )
    }

    /// A function to generate HTML
    /// code for the `ListItem` variant
    /// of the `AtomicStatement` enumeration.
    /// The HTML code is returned as a string. 
    pub fn generate_list_item_code(
        &self,
        contents: &Box<Vec<AtomicStatement>>
    ) -> String {
        let mut strings: Vec<String> = Vec::new();
        for a_stmt in contents.iter(){
            strings.push(self.generate_atomic_code(a_stmt));
        }
        let joined: String = strings
            .into_iter()
            .collect::<String>();
        format!("<li>{}</li>", joined)
    }
}
