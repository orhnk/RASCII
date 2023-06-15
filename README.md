<div align="center">

[![Crate Status](https://img.shields.io/crates/v/rascii_art.svg)](https://crates.io/crates/rascii_art)
[![Docs Status](https://docs.rs/rascii_art/badge.svg)](https://docs.rs/crate/rascii_art/)

  <p style="margin-bottom: 0 !important;">
    <img alt="RASCII Logo" src="https://user-images.githubusercontent.com/101834410/204127025-b98aaf39-778b-468b-8f41-36fd858708e8.png" width=600>
  </p>
</div>

<a name="reference">
  <pre>
Advanced ASCII Art Generator

Usage: rascii [OPTIONS] [FILENAME]

Arguments:
  [FILENAME]  Path to the image

Options:
  -q, --query <QUERY>
          &#9; Use AI to generate ascii art
  -n, --negative-query <NEGATIVE_QUERY>
          &#9; Use AI to generate ascii art, but with a negative query [default: ]
  -N, --num-image <NUMBER>
          &#9; Number of images to generate when using AI [1..9] [default: 9]
  -m, --model-type <MODEL_TYPE>
          Model to use in generation [default: general] [possible values: art, drawing, photo, general]
  -v <API_VERSION>
          &#9; Model API version [default: 3] [possible values: 1, 3]
  -a, --api-token <TOKEN>
          &#9; API token for premium users (Faster generation, No watermark)
  -w, --width <WIDTH>
          &#9; Width of the output image. Defaults to 128 if width and height are not specified
  -H, --height <HEIGHT>
          &#9; Height of the output image, if not specified, it will be calculated to keep the aspect ratio
  -c, --color
          &#9; Whether to use colors in the output image
  -i, --invert
          &#9; Inverts the weights of the characters. Useful for white backgrounds
  -C, --charset <CHARSET>
          &#9; Characters used to render the image, from transparent to opaque. Built-in charsets: block, emoji, default, russian, slight [default: default]
  -h, --help
          &#9; Print help
  -V, --version
          &#9; Print version
  </pre>  
</a>

## Features

- **Available as a crate:** RASCII has a very simple API allowing you to use RASCII from your programs without using the system shell.

- **AI generated ASCII art** RASCII uses craiyon API to generate ASCII art from text prompts.

- **Colored ASCII art generation**: RASCII uses ANSI color codes to generate colored ASCII art.
  > **Note**: Your terminal emulator has to support `truecolor`
  > (don't worry, almost all modern terminal emulators do).

- **Super efficient colored output**: RASCII never repeats the same ANSI color code if it is already active.
  > This makes a huge difference in images with little alternating
  > color, up to about 1800% reduction in output size. Woah!

- **Custom dimensions**: RASCII allows you to give custom dimensions to the outputted ASCII art while keeping the aspect ratio (unless both dimensions are provided).

- **Custom charsets:** RASCII allows you to use custom charsets to generate your ASCII art.
  > **Note**: The given charset must go from transparent to opaque.

- **Lots of pre-included charsets.**

## Installing The CLI

## Via Cargo

> **Note**: This is the recommended way of installing the RASCII CLI.

> **Warning**: You must have `~/.cargo/bin/` in your `PATH` to run `rascii` directly.

```sh
cargo install rascii_art
```

## Manually

> **Warning**: this installation method is discouraged and only works for GNU/Linux or any other POSIX compatible systems!

```sh
git clone https://github.com/KoBruhh/RASCII && cd RASCII
chmod +x install.sh
./install.sh
```

## Using The Crate

Instead of using the unreliable system shell to call RASCII,
you can add the `rascii_art` crate to your project and use it in Rust!

To do so, run `cargo add rascii_art` to add RASCII to your Cargo project.

### ASCII art from file content

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

### ASCII art from text prompt:

```rs
use rascii_art::{
    craiyon::Model,
    render_image_to,
    RenderOptions,
};

fn main() {
    let mut implements_io_write = String::new();

    let model = Model::new();
    let images = model.generate("A king crab", 1); // Generate 1 image

    // Current API generates 9 images per prompt
    for image in images {
        render_image_to(
            image,
            &mut implements_io_write,
            RenderOptions::new()
                .width(100)
                .colored(true)
                .charset(&[".", ",", "-", "*", "£", "$", "#"]),
        )
        .unwrap();
        println!("{implements_io_write}");
    }
}
```

## Showcase

### AI ascii art
Use `rascii -q <QUERY> [OPTIONS]` to generate ascii art from text prompt. See [other options.](#reference)
- `rascii -q "red camel" -c -C block`

![RASCII output of "red camel" prompt](https://github.com/UTFeight/RASCII/assets/101834410/67dcb0c9-8f98-4243-b7df-01373d3b3643)

### Japanese Charset

![RASCII output of chad with the Japanese charset](https://github.com/KoBruhh/RASCII/assets/101834410/c038edc9-cab3-4270-95df-0269203763fd)

### Emoji Charset

> **Note**: The emoji charset does not guarantee your outputs color will match the color of your image,
> this is just a coincidence that happened with Ferris.

![RASCII output of ferris with the emoji charset](https://user-images.githubusercontent.com/101834410/204243964-f4cfdf8d-10b9-4a2c-8d3c-41182320c789.png)

### Chinese Charset

![RASCII output with the Chinese charset](https://github.com/KoBruhh/RASCII/assets/101834410/357c084f-ea93-40f9-baa8-16e329b95a51)

### Block Charset

![RASCII output of TrollPepe with the block charset](https://github.com/KoBruhh/RASCII/assets/101834410/3ac7e920-7ab4-441d-886e-2028b108578d)

### Custom ASCII Charset

You can use the `--charset` (or `-C`) CLI option to provide a custom charset to use when generating some ASCII art.

The value of this must option must go from transparent to opaque, like so:

```sh
rascii --charset " ░▒▓█" --color ferris.png
```

Note that a charset similar to the above charset is available as a builtin named `block`.

![RASCII output of Ferris with a custom charset](https://user-images.githubusercontent.com/101834410/204243768-4a15bb21-ba93-4979-bd4f-d8e8b1dc4112.png)


### Contributors

![UTFeight](https://github.com/KoBruhh/RASCII/assets/101834410/2b06a6b0-9cb9-448e-8979-4a5182e2e4b2)
![RGBCube](https://github.com/KoBruhh/RASCII/assets/101834410/3e5b18c3-d7c8-4862-bee5-b5cf06c83994)
![felixonmars](https://github.com/KoBruhh/RASCII/assets/101834410/66914a48-a5c5-4619-a46d-b99c77b3cd77)
![fnordpig](https://github.com/fnordpig/RASCII/assets/1621189/5b3225f3-ae83-4ed3-a3fb-3d88de18f82e)

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=UTFeight/Rascii&type=Date)](https://star-history.com/#UTFeight/Rascii&Date)
