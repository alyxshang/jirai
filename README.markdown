# JIRAI :ribbon: :black_heart:

![Jirai CI](https://github.com/alyxshang/jirai/actions/workflows/rust.yml/badge.svg)

***A compiler for a text-markup language inspired by Jirai Kei.***

## ABOUT :books:

This repository contains the source code for a compiler for a
text-markup language inspired by the [Jirai Kei](https://aesthetics.fandom.com/wiki/Jirai_Kei) aesthetic from Japan. This
compiler is written as a Rust crate containing functions to
compile Jirai code to HTML code.

## INSTALLATION :inbox_tray:

To use ***Jirai*** in your Rust project, add this line to your project's
`Cargo.toml`'s `[dependencies]` section:

```TOML
jirai = { git = "https://github.com/alyxshang/jirai" }
```

## THE FORMAT :balance_scale:

***Jirai*** is a text-markup format inspired by the [Jirai Kei](https://aesthetics.fandom.com/wiki/Jirai_Kei)
aesthetic from Japan. It is similar to the Markdown text-markup format.
***Jirai*** has block-level elements and some inline-elements that can
inifintely nest. 

- Jirai documents: Every document has to start and end with the `(^-^)` symbol.
- Block elements:
    - Paragraph: A paragraph has to start and end with a new line.
    - Heading: A heading is marked by the `<3` symbol. The number of these symbols at the start of a heading marks the level of the heading.
    - Unordered list: An unordered list is constituted by lines starting with the `~` character and ending with a new line symbol.

- Inline elements:
    - Link: A link is of the following format: `{#[A link to Wikipedia][a link to Wikipedia][https://wikipedia.org]}`. The first string enclosed by square brackets is the text for the `alt` attribute. The second string enclosed by square brackets is the text inside the `a` element. The third string enclosed by square brackets is the URL of the link.
    - Images: A link to an image is of the following format: `{@[][]}`. The first string enclosed by square brackets is the text for the `alt` attribute. The second string enclosed by square brackets is the link to the image.
    - Bold text: Any bold text is enclosed by the `*` character.
    - Italic text: Any italic text is enclosed by the `$` character.
    - Code: Any inline code is enclosed by angle brackets.
    - Block quotes: Quotes are enclosed by the following symbols: `>(QUOTE TEXT HERE)<`.

- Elements that can contain other elements inside them:
    - Paragraphs.
    - Headings.
    - Unordered Lists.
    - Italic test.
    - Bold text.

## API DOCUMENTATION :tada:

To find out which functions, structures, and other entities this library
contains, clone this repository and run the command `cargo doc --open`
from the repository's root to view the full API documentation.

## USAGE :hammer_and_pick:

To compile a string of Jirai source code into HTML using this crate, you
would do so in a fashion similar to this:

```Rust
use jirai::to_html;
use jirai::SourceType;
use std::fs::read_to_string;

fn main(){
    let document_contents: String = read_to_string("my_document.jirai")
        .expect("Error opening document.");
    let html: String = to_html(&document_contents, &SourceType::Document)
        .expect("Error compiling document to HTML.");
    println!("{}", &html);
}
```

## JIRAI SAMPLE :ribbon: :black_heart:

```Text
(^-^)
<3 $Heading I$
Lorem ipsum sit dolor amet. Lorem ipsum sit dolor amet.

<3<3 *Heading II*
Lorem ipsum sit dolor amet. Lorem ipsum sit dolor amet. This text
contains a {#[link][https://alyxshang.boo]}.

<3<3<3 $Heading III$
This paragraph contains my profile pic.
{@[my pfp][https://avatars.githubusercontent.com/u/179976644?v=4]}

<3<3 Heading IV

This is a list!

~ List item 1.
~ List item 2.
~ List item 3.
(^-^)
```

## CHANGELOG :black_nib:

### Version 0.1.0

- [x] Implemented a lexer.
- [x] Implemented a parser.
- [x] Implemented a HTML generator.
- [x] Implemented a Javascript generator.
- [ ] Fix the `<3` bug.
- [ ] Fix the unclosed-delimiter bug.
- [ ] Extensive testing.

## NOTE :scroll:

- *Jirai :ribbon: :black_heart:* by *Alyx Shang :black_heart:*.
- Licensed under the [FSL v1](https://github.com/alyxshang/fair-software-license).
