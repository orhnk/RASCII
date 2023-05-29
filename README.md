<div align="center">
  <p style="margin-bottom: 0 !important;">
    <img alt="RASCII Logo" src="https://user-images.githubusercontent.com/101834410/204127025-b98aaf39-778b-468b-8f41-36fd858708e8.png" width=600>
  </p>
</div>

```
Advanced ASCII Art Generator

Usage: rascii [OPTIONS] <FILENAME>

Arguments:
  <FILENAME>  Path to the image file.

Options:
  -w, --width <WIDTH>      Width of the output image. Defaults to 128 if width and height are not specified
  -H, --height <HEIGHT>    Height of the output image, if not specified, it will be calculated to keep the aspect ratio
  -c, --color              Whether to use colors in the output image
  -i, --invert             Inverts the weights of the characters. Useful for white backgrounds
  -C, --charset <CHARSET>  Characters used to render the image, from translucent to opaque. Built-in charsets: block, emoji, default, russian, slight [default: default]
  -h, --help               Print help
  -V, --version            Print version
```

## Features

- **Available as a crate:** RASCII has a very simple API allowing you to RASCII from your programs without using the system shell.

- **Colored ASCII art generation**: RASCII uses ANSI color codes to generate colored ASCII art.
  > **Note** Your terminal emulator has to support `truecolor`
  > (don't worry, almost all modern terminal emulators do).

- **Super efficient colored output**: RASCII never repeats the same ANSI color code if it is already active.
  > This makes a huge difference in images with little alternating
  > color, up to about 1800% reduction in output size. Woah!

- **Custom dimensions**: RASCII allows you to give custom dimensions to the outputted ASCII art while keeping the aspect ratio (unless both dimensions are provided).

- **Custom charsets:** RASCII allows you to use custom charsets to generate your ASCII art.
  > **Note** The given charset must go from transparent to opaque.

- **Lots of pre-included charsets.**

## Installing The CLI

## Via Cargo

> **Note** this is the recommended way of installing the RASCII CLI.

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

## Using The Crate

Instead of using the unreliable system shell to call RASCII,
you can add the `rascii_art` crate to your project and use it in Rust!.

To do so, run `cargo add rascii_art` to add RASCII to your Cargo project.

Here is a code example:

```rs
use rascii_art::{
    render_to,
    RenderOptions,
};

fn main() {
    let mut implements_io_write = String::new();

    render_to(
        "ferris.png",
        &mut implements_io_write,
        RenderOptions::new()
            .width(100)
            .colored(true)
            .charset(&[".", ",", "-", "*", "£", "$", "#"]),
    )
    .unwrap();

    println!("{implements_io_write}");
}
```

## Showcase

### Japanese Charset

![RASCII output of chad with the Japanese charset.](https://github.com/KoBruhh/RASCII/assets/101834410/c038edc9-cab3-4270-95df-0269203763fd)

### Emoji Charset

> **Note** The emoji charset does not guarantee your outputs color will match the color of your image,
> this is just a coincidence that happened with Ferris.

![RASCII output of ferris with the emoji charset.](https://user-images.githubusercontent.com/101834410/204243964-f4cfdf8d-10b9-4a2c-8d3c-41182320c789.png)

### Chinese Charset

![RASCII output with the Chinese charset.](https://github.com/KoBruhh/RASCII/assets/101834410/357c084f-ea93-40f9-baa8-16e329b95a51)

### Block Charset

![RASCII output of TrollPepe with the block charset](https://github.com/KoBruhh/RASCII/assets/101834410/3ac7e920-7ab4-441d-886e-2028b108578d)

![RASCII output of Ferris with the block charset](https://github.com/KoBruhh/RASCII/assets/101834410/5122c5ba-8707-489e-a720-caf2e183b026)

### Custom ASCII Charset

You can use the `--charset` CLI option to provide a charset to use when generating some ASCII art.

The value of this must option must go from transparent to opaque, like so:

```sh
rascii --charset " ░▒▓█" --color ferris.png
```

Note that the given charset is available as a builtin named `block`, so the command here is the same as:

```sh
rascii --charset block --color ferris.png
```

![RASCII output of Ferris with a custom charset](https://user-images.githubusercontent.com/101834410/204243768-4a15bb21-ba93-4979-bd4f-d8e8b1dc4112.png)

### Amogus

![Amogus](https://user-images.githubusercontent.com/101834410/204243525-ed62e0df-789d-4da8-a3a5-3919c548e050.png)
