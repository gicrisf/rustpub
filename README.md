# Rustpub
Create epub files from web content using rust.

Rustpub is a CLI program and a library for web-content epub generation.

## Installation
Installation is easy as launching a single command on your terminal:

```
$ git clone https://github.com/mr-chrome/rustpub && cargo install --path ./rustpub
```

I'm going to load this software on cargo really soon, but meanwhile git is the
way to go.

## Quickstart

Just give rustpub a URL and it will make the requested epub:

```
$ rustpub -u https://www.zwitterio.it/articoli/come-funziona-il-vaccino-azoxford.html
```

### Options

Optional arguments:
- `-o [FILENAME]` or `--output [FILENAME]` for choosing a custom epub file name;
- `-test` for developers, in order to test with a custom and pre-selected url.
- `--parser` for selecting python and javascript optional parsers.

Rustpub is totally written in Rust, but you can choose between three parsers:
- The default one, written in Rust: [Readability (Rs)](https://github.com/kumabook/readability);
- The Python one, based on the famous library Beautiful Soup 4: [ReadabiliPy](https://github.com/alan-turing-institute/ReadabiliPy);
- The Javascript one, written by Mozilla for reader mode on Firefox browser: [Readability (Js)](https://github.com/mozilla/readability).

Of course, the Python and Javascript parsers require respectively Python and Node
installed on your machine to be used by Rustpub for content rendering.
If you desire alternative parsers because you don't like the default one or
just because is always fun to try other ways, you don't need to panic:
[installing them is as easy as the main installation was](#alternative-parsers-installation).

## Features
- Rustpub is easy to use and to install;
- Rustpub creates epubs quickly and taking care of all the poor html formatting;
- Rustpub can be integrated in other applications and used as library. In fact,
it was born to be used by Kindle-pult, my GTK app to send web pages or epubs to
kindle with ease.

## How it works

Rustpub starts operations downloading the webpage from the user's URL. Then,
Rustpub passes the content to a HTML parser which is necessary in order to
clean the DOM from extra-content elements that are directly functional to your
article or book. Finally, Rustpub downloads all content images, optimize them
and zip everything in a epub file with the proper file hierarchy and metadata.

```
CLI -> HTML downloading -> HTML parsing -> Image downloading
-> Image optimization -> Epub file generation -> Your Epub 
```

## Alternative parsers installation
First of all, make sure you have Python installed on your machine, using this
command:
```
$ python --version
```

It should return something like this: `Python 3.8.5`. If it doesn't do that,
pleas [install Python](https://www.python.org/downloads/).

Is Python package manager installed too? Verify that with another command:
```
$ pip --version
```

If not, [install pip](https://pip.pypa.io/en/stable/installing/).

Finally, install ReadabiliPy, using pip:

```
$ pip install readabilipy
```

Now you're ready to go with `--parser py`.

> But I want Modilla Javascript parser! I want to be sure my ebooks will be like
reader-mode firefox version of my articles.

Then you need Node JS. If you haven't, [install it too](https://nodejs.org/).
If you already have Node, just use `--parser js` as argument and you will not
be disappointed.

## TODOS
- Image optimization and resizing;
- Mobi format support;
- Progress bar.

## Thanks to
This project was made possible by the wonderful Rust ecosystem and community.
Particularly, [Epub-builder](https://github.com/lise-henry/epub-builder) simplified
zipping and organizing files in the epub archive.
