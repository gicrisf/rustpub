# Rustpub
Create epub files from web content using rust.

## Installation
You can install rustpub through cargo with just a line command in your terminal:

```
$ cargo install rustpub
```

Rustpub can also be compiled from source:

```
$ git clone https://github.com/mr-chrome/rustpub.git && cd rustpub && cargo build --release
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

```
$ rustpub -u https://your-url.it
```

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
