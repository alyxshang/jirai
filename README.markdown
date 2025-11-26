# JIRAI :ribbon: :black_heart:

![Jirai CI](https://github.com/alyxshang/jirai/actions/workflows/rust.yml/badge.svg)

***A compiler for a text-markup language inspired by Jirai Kei.***

## ABOUT :books:

This repository contains the source code for a compiler for a
text-markup language inspired by the [Jirai Kei](https://aesthetics.fandom.com/wiki/Jirai_Kei) aesthetic from Japan. This
compiler is in the format of a Rust crate containing functions to
compile Jirai source code into other formats of code.

## INSTALLATION :inbox_tray:

***COMING SOON!***

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
    - Images: An link to an image is of the following format: `{@[][]}` The first string enclosed by square brackets is the text for the alt attribute. The second string enclosed by square brackets is the link to the image.
    - Bold text: Any bold text is enclosed by the `*` characters.
    - Italic text: Any italic text is enclosed by the `$` character.
    - Code: Any inline code is enclosed by angle brackets.

## USAGE :hammer_and_pick:

***COMING SOON!***

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
- [ ] Implemented an IR generator.
- [ ] Implemented a HTML generator.
- [ ] Implemented a WASM generator.

## NOTE :scroll:

- *Jirai :ribbon: :black_heart:* by *Alyx Shang :black_heart:*.
- Licensed under the [FSL v1](https://github.com/alyxshang/fair-software-license).
