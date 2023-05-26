<div align="start">
  <p style="margin-bottom: 0 !important;">
    <img alt="RASCII Logo" src="" width=200>
  </p>
  <h1 style="font-size: 3em">RASCII</h1>

Advanced ASCII art generator written in Rust 🦀🚀
</div>


## Features

* Colored ASCII art generation using ANSI color codes.
* Custom dimensions.
* Custom characters: Supply your own charset to generate ASCII art!
* Lots of pre-included charsets.

## Installation

## Via Cargo

> **Info** this is the reccomended way of installing RASCII.

```sh
cargo install rascii_art
```

## Manually

> **Warning** this installation method is discouraged and only works for GNU/Linux or any other POSIX compatible systems!

```
git clone https://github.com/KoBruhh/RASCII && cd RASCII
chmod +x install.sh
./install.sh
```

 ## Showcase

### Japanese Charset

![RASCII output of gigachad with the Japanese charset.](https://user-images.githubusercontent.com/101834410/204259580-46ea59ae-e7d1-4f96-b14f-1d90f2376f6f.png)

### Emoji Charset

> **Note** The emoji charset does not guarrantee your outputs color will match the color of your image,
> this is just a coincidence that happened with Ferris.

![RASCII output of ferris with the emoji charset.](https://user-images.githubusercontent.com/101834410/204243964-f4cfdf8d-10b9-4a2c-8d3c-41182320c789.png)

### Chinese Charset

![RASCII output with the Chinese charset.](https://user-images.githubusercontent.com/101834410/204243902-4de1e10a-4e86-455d-8817-09b57ca2bc40.png)

### Block Charset

![RASCII output of trollpepe with the block charset](https://user-images.githubusercontent.com/101834410/204243571-f6697b6f-f27d-4da1-a75c-c2c51723978d.png)

![RASCII output of trollpepe with the block charset](https://user-images.githubusercontent.com/101834410/204244536-f1c3674a-2c96-4d00-a310-c5cff63d3348.png)

### Custom ASCII Charset

You can use the `--charset` CLI option to provide a charset to use when generating some ASCII art.

The value of this must option must go from opaque to transparent, like so:

```sh
rascii --charset ██▓▒░ --color ferris.png
```

![RASCII output of Ferris with a custom charset](https://user-images.githubusercontent.com/101834410/204243768-4a15bb21-ba93-4979-bd4f-d8e8b1dc4112.png)

### Amogus

![Amogus](https://user-images.githubusercontent.com/101834410/204243525-ed62e0df-789d-4da8-a3a5-3919c548e050.png)
