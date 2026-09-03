# JIRAI

![Jirai CI](https://github.com/alyxshang/jirai/actions/workflows/rust.yml/badge.svg)

***A compiler for a markup language inspired by Jirai Kei.***

## ABOUT

This repository contains the source code for a compiler for a
Markdown-like language with a syntax inspired by [Jirai Kei](https://aesthetics.fandom.com/wiki/Jirai_Kei). The compiler is a Rust crate written without any external 
dependencies and was made to be light and fast. The
language itself was designed by me. Some optional features of this crate do use
external libraries. These libraries are: [JMU](https://github.com/alyxshang/jmu), [Yue](https://github.com/alyxshang/yue), and [Serde](https://serde.rs)

## SPECIFICATION

A specification of the Jirai Language can be found [here](https://alyxshang.boo/jirai).

## USAGE

### CLI Usage

The Jirai compiler can be installed using Cargo with the following command:

```bash
cargo install --git https://github.com/alyxshang/jirai --tag v.0.1.0 --features=cli
```

The compiler's CLI offers the following options:

- `--help`: Displays usage information.
- `--version`: Displays the current version of the compiler.
- `--lint FILE_PATH`: Lints a Jirai file located at the supplied path for correctness.
- `--build FILE_PATH`: Compiles a Jirai file located at the supplied path into an HTML file with the same name prefix.
- `--build FILE_PATH --out CUSTOM_OUTPUT_PATH`: Compiles a Jirai file located at the supplied path into an HTML file at the supplied output path.


### Crate Usage

To add the ***Jirai*** crate to your Rust project and to compile
text written in the ***Jirai*** format into HTML, add the following 
line to the `dependencies` section of your project's `Cargo.toml`:

```TOML
jirai = { git = "https://github.com/alyxshang/jirai", tag = "v.0.1.0" }
```

If you want to use the `ejirai` feature this crate offers, you would add
the `ejirai` feature to the `features` section of the line above.

You would use the ***Jirai*** crate in your code in a manner similar to
the one outlined in the code samples below. These code samples assume that 
you have a file called `sample.jirai` in the same directory as your 
`Cargo.toml`.

```text
# sample.jirai

<3 Heading 1
# This is a comment.
This is normal text with a very long sentence.
This is a sentence containing *italic* and _bold text_.
This is a link to an image >($[alt][https://alyxshang.boo])<.
This text contains some _bold text_.
This text contains some *italic text*.

<3<3 Heading 2

This paragraph contains some _nested *italic* text_.
And this is a >(%[link to my site][link to my site][https://alyxshang.boo])<.

~ This is item one of an unordered list.
~ This is item two of an unordered list.
```

The sample below illustrates how to use the `to_html` function to
compile the Jirai code above into HTML code.

```Rust
use jirai::to_html;
use std::path::PathBuf;
use std::fs::read_to_string;

fn main(){
    let mut jirai_file_path: PathBuf = PathBuf::new();
    jirai_file_path.push(env!("CARGO_MANIFEST_DIR"));
    jirai_file_path.push("sample.jirai");
    let file_contents: String = read_to_string(
        &jirai_file_path.display().to_string()
    ).expect("Could not read file.");
    let code: String = to_html(&file_contents)
        .expect("Could not compile Jirai code to HTML.");
    println!("{:?}", &code); // should output HTML code.
}
```

If you want to use ***Extended Jirai*** in your project, you would use the 
`jirai` crate in your code in the following manner. These code samples assume
 that you have a file called `sample.ejirai` in the same directory as your 
`Cargo.toml`.

```text
(^-^)
# sample.ejirai
"layout" >~< "page"
"title" >~< "Page Title"
(^-^)
<3 Heading 1
# This is a comment.
This is normal text with a very long sentence.
This is a sentence containing *italic* and _bold text_.
This is a link to an image >($[alt][https://alyxshang.boo])<.
This text contains some _bold text_.
This text contains some *italic text*.

<3<3 Heading 2

This paragraph contains some _nested *italic* text_.
And this is a >(%[link to my site][link to my site][https://alyxshang.boo])<.

~ This is item one of an unordered list.
~ This is item two of an unordered list.
(^-^)
```

The Rust code sample below illustrates how you would use this crate and
`serde` to deserialize an ***Extended Jirai*** document. You must have the `ejirai`
feature enabled on the `jirai`crate.

```Rust
use serde::Deserialize;
use jirai::from_ejirai;
use std::path::PathBuf;
use jirai::ExtendedJirai;
use std::fs::read_to_string;

#[derive(Deserialize, Debug)]
pub struct PageData{
    pub title: String,
    pub layout: String
}

fn main(){
    let mut ejirai_file_path: PathBuf = PathBuf::new();
    ejirai_file_path.push(env!("CARGO_MANIFEST_DIR"));
    ejirai_file_path.push("sample.ejirai");
    let file_contents: String = read_to_string(
        &ejirai_file_path.display().to_string()
    ).expect("Could not open file.");
    let deserialized: ExtendedJirai<PageData> = from_ejirai(&file_contents)
        .expect("Could not deserialize Extended Jirai code.");
    println!("{:?}", &deserialized.data); // should output "PageData"
    println!("{:?}", &deserialized.code); // should output HTML code
}
```

More information on the entities inside this crate can be obtained 
by cloning this repository and running the command `cargo doc --open`
from the root of the repository.

## CHANGELOG

### Version 0.1.0

- Initial release.
- Initial upload to GitHub.

## NOTE

- *Jirai* by *Alyx Shang*.
- Licensed under the [FSL v1](https://alyxshang.boo/fair-software-license).
