# Rustpub
Create epub files from web content using rust.

## Installation
You can install rustpub with just a couple of lines in your terminal:

```
$ pip install readabilipy
$ cargo install rustpub
```

Rustpub can also be compiled from source:

```
$ git clone https://github.com/mr-chrome/rustpub.git
&& cd rustpub
&& cargo build --release
```

You will find your binary in `rustpub/target/release`.

## Remember!
Rustpub is totally written in Rust, but the sanitation functions are inherited
by Node or Python code; in the future Rustpub is going to integrate a pure-rust
solution for this part of the process, but now you need at least Python and pip
installed on your machine.

Node, instead, is an optional: you need it only if you want to use Mozilla's
Readability.js library, the one which is integrated in Firefox browser for reader
mode.

## Quickstart

Launch a simple command like this
```
$ rustpub -u https://www.zwitterio.it/articoli/come-funziona-il-vaccino-azoxford.html
```

Optional arguments:
- `-o filename` in order to choose epub file name (gives `filename.epub`);
- `-t` for developers, in order to test with a custom and pre-selected url.

## Features
- Rustpub is easy to use and to install;
- Rustpub creates epubs quickly and taking care of all the poor html formatting;
- Rustpub can be integrated in other applications and used as library. In fact,
it was born to be used by Kindle-pult, my GTK app to send web pages or epubs to
kindle with ease.

## TODOS
- Image optimization and resizing;
- Rust sanitation functions for web pages;
- Mobi format support;
- CLI arguments to select sanitation mode;
- Progress bar.

## Thanks to
- [epub-css-starter-kit](https://github.com/mattharrison/epub-css-starter-kit) is where CSS is get from;
- [ReadabiliPy](https://github.com/alan-turing-institute/ReadabiliPy) is used for sanitation of the HTML code;
- [Epub-builder](https://github.com/lise-henry/epub-builder) for building actual epub archive.
