<div align="center">

[![Crate Status](https://img.shields.io/crates/v/rascii_art.svg)](https://crates.io/crates/rascii_art)
[![Docs Status](https://docs.rs/rascii_art/badge.svg)](https://docs.rs/crate/rascii_art/)

<p style="margin-bottom: 0 !important;">
    <img alt="RASCII Logo" src="https://user-images.githubusercontent.com/101834410/204127025-b98aaf39-778b-468b-8f41-36fd858708e8.png" width=600>
  </p>
</div>

```
Advanced ASCII Art Generator

Usage: rascii [OPTIONS] <FILENAME>

Arguments:
  <FILENAME>  Path to the image

Options:
  -w, --width <WIDTH>      Width of the output image. Defaults to 128 if width and height are not specified
  -H, --height <HEIGHT>    Height of the output image, if not specified, it will be calculated to keep the aspect ratio
  -c, --color              Whether to use colors in the output image
  -i, --invert             Inverts the weights of the characters. Useful for white backgrounds
  -C, --charset <CHARSET>  Characters used to render the image, from transparent to opaque. Built-in charsets: block, emoji, default, russian, slight [default: default]
  -h, --help               Print help
  -V, --version            Print version
```

## Features

- **Available as a crate:** RASCII has a very simple API allowing you to use
  RASCII from your programs without using the system shell.

- **Colored ASCII art generation**: RASCII uses ANSI color codes to generate
  colored ASCII art.

  <!-- deno-fmt-ignore -->
  > [!NOTE]
  > Your terminal emulator has to support `truecolor` (don't worry,
  > almost all modern terminal emulators do).

- **Super efficient colored output**: RASCII never repeats the same ANSI color
  code if it is already active.
  > This makes a huge difference in images with little alternating color, up to
  > about 1800% reduction in output size. Woah!

- **Custom dimensions**: RASCII allows you to give custom dimensions to the
  outputted ASCII art while keeping the aspect ratio (unless both dimensions are
  provided).

- **Custom charsets:** RASCII allows you to use custom charsets to generate your
  ASCII art.

<!-- deno-fmt-ignore -->
> [!NOTE]
> The given charset must go from transparent to opaque.

- **Lots of pre-included charsets.**

## Installing The CLI

## Via Cargo

<!--deno-fmt-ignore-->
> [!NOTE]
> This is the recommended way of installing the RASCII CLI.

<!--deno-fmt-ignore-->
> [!WARNING]
> You must have `~/.cargo/bin/` in your `PATH` to run `rascii`
> directly.

```sh
cargo install rascii_art
```

## Manually

<!--deno-fmt-ignore-->
> [!WARNING]
> this installation method is discouraged and only works for
> GNU/Linux or any other POSIX compatible systems!

```sh
curl -sL https://raw.githubusercontent.com/orhnk/RASCII/refs/heads/master/install.sh | sh
```

## Using The Crate

Instead of using the unreliable system shell to call RASCII, you can add the
`rascii_art` crate to your project and use it in Rust!

To do so, run `cargo add rascii_art` to add RASCII to your Cargo project.

Here is a code example:

```rs
use rascii_art::{
    render_to,
    RenderOptions,
};
                                                            
fn main() {
    let mut buffer = String::new();
                                                            
    render_to(
        r"/path/to/image.png",
        &mut buffer,
        &RenderOptions::new()
            .width(100)
            .colored(true)
            .charset(&[".", ",", "-", "*", "£", "$", "#"]),
    )
    .unwrap();
}
```

## Showcase

### Japanese Charset

<p align="center">
    <img src="https://github.com/KoBruhh/RASCII/assets/101834410/c038edc9-cab3-4270-95df-0269203763fd">
</p>

### Emoji Charset

<!--deno-fmt-ignore-->
> [!NOTE]
> The emoji charset does not guarantee your outputs color will match
> the color of your image, this is just a coincidence that happened with Ferris.

<p align="center">
    <img src="https://user-images.githubusercontent.com/101834410/204243964-f4cfdf8d-10b9-4a2c-8d3c-41182320c789.png">
</p>

### Chinese Charset

<p align="center">
    <img src="https://github.com/KoBruhh/RASCII/assets/101834410/357c084f-ea93-40f9-baa8-16e329b95a51">
</p>

### Block Charset

<p align="center">
    <img src="https://github.com/KoBruhh/RASCII/assets/101834410/3ac7e920-7ab4-441d-886e-2028b108578d">
</p>

<!-- ![RASCII output of Ferris with the block charset](https://github.com/KoBruhh/RASCII/assets/101834410/5122c5ba-8707-489e-a720-caf2e183b026) -->

### Custom ASCII Charset

You can use the `--charset` (or `-C`) CLI option to provide a custom charset to
use when generating some ASCII art.

The value of this must option must go from transparent to opaque, like so:

```sh
rascii --charset " ░▒▓█" --color ferris.png
```

Note that a charset similar to the above charset is available as a builtin named
`block`.

<p align="center">
    <img src="https://user-images.githubusercontent.com/101834410/204243768-4a15bb21-ba93-4979-bd4f-d8e8b1dc4112.png">
</p>

### Contributors

![KoBruhh](https://github.com/KoBruhh/RASCII/assets/101834410/2b06a6b0-9cb9-448e-8979-4a5182e2e4b2)
![RGBCube](https://github.com/KoBruhh/RASCII/assets/101834410/3e5b18c3-d7c8-4862-bee5-b5cf06c83994)
![felixonmars](https://github.com/KoBruhh/RASCII/assets/101834410/66914a48-a5c5-4619-a46d-b99c77b3cd77)
![fnordpig](https://github.com/fnordpig/RASCII/assets/1621189/5b3225f3-ae83-4ed3-a3fb-3d88de18f82e)

---

<!--deno-fmt-ignore-->
> [!NOTE]
> There is a python script at repository root that can be used to generate
> the above contributor ASCII Art.
>
> ```sh
> python contributors.py
> ```
