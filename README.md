<p align="center">
  <img src="https://user-images.githubusercontent.com/101834410/204127025-b98aaf39-778b-468b-8f41-36fd858708e8.png">
</p>

# RASCII

image to ascii art fully created with rust 🦀 🚀

## Features:

    * Multi-language support -> ✔
    * Custom char list -> ✔
    * pixelart support for terminal -> ✔
    ...

# Installation

## Cross-Platform (Cargo):

**RECOMMENDED**

`cargo install rascii_art`

## MANUAL

**(Not suggested!)**

**FOR LINUX:**

```
git clone https://github.com/KoBruhh/RASCII.git
cd RASCII
./INSTALL # Moves binaries to /usr/bin/
```

**FOR WİNDOWS:**

```
git clone https://github.com/KoBruhh/RASCII.git
cd RASCII
cargo build --release # You have to move your binary (target/release/) to your own $PATH manually (Your Program Files Directory)
```

# Usage:

```shell
~This program allows you to create ASCII art from any image (.png, .jpeg, .jpg ...)~
SubCommands (rascii <SubCommand>):
-p, --path <path/to/image> // given path is the image path that you want to convert.
-i, --invert // to be able to create images without white background. (if your image has a white plain and you want to ignore it use this command)
-c, --colored // to make ASCII art colored
-h, --help // to access this window
-r, --ratio // to resize img, It could be higher than hundred! but cant be lower than zero!
-bg, --background // to paint background with the color of the pixel!
-wc, --with-color <R> <G> <B> // to create custom colored ASCII arts! you could mix these with -bg !
-px, --pixelated // to convert output to pixelart
-l <your_char_list_or `Available lists at bottom appendix I`> --list <your_char_list_or `Appendix I`> // allows you to create custom lists! and use builtin char_lists!

|If you dont understand, Just ask your mom|
||This project is rusty, so it is fast! really!||

Appendix I
Dont forget that you could enter your own list by:
rascii -p <path/to/path> -l <your_char_list_or || names below!>
Exmpl:
rascii -p <path/to/path> -l bruh // creats your image by using ['b', 'r', 'u', 'h']
rascii -p <path/to/path> -l emoji // creates your image by using emojis -> bcuz list below contains a list named: "emoji"!
Builtin_lists: [
"japanese",
"slight",
"emoji",
"chinese",
"ansi",
"russian",
]
```

of course you could mix these commands

# Samples:

`rascii -p /photos/ferris.jpeg -i -c -l japanese >> ferris.txt` -> It will stores the output on ferris.txt,

If you want to see the results only (not store) you have to do: `rascii -p /photos/ferris.jpeg -i -c -l japanese` // both uses builtin japanese char list

# Japanese

<p align="center">
  <img src="https://user-images.githubusercontent.com/101834410/204259580-46ea59ae-e7d1-4f96-b14f-1d90f2376f6f.png">
</p>

# Emoji

Not that perfect, because emojis are pretty complicated by their look! thats why picture below is a bit noisy
![Screenshot_RASCII : fish_2](https://user-images.githubusercontent.com/101834410/204243964-f4cfdf8d-10b9-4a2c-8d3c-41182320c789.png)

# Chinese

![Screenshot_RASCII : fish_15](https://user-images.githubusercontent.com/101834410/204243902-4de1e10a-4e86-455d-8817-09b57ca2bc40.png)

# Custom ASCII list stored on environment variable!

I am using fish so I am using `set <var> <value>` to create an env var!

If you use bash or such you have to use `export <var>=<value>`
`set ANSI_CHAR_LIST ██▓▒░` -> to set an environment variable on terminal

Also dont forget that you could insert your list directly to -l ->

`rascii -p <path/to/img> -l 0123456789` creates an img by using 0..10 nums!

`rascii -p <path/to/img> -l $ANSI_CHAR_LIST` -> to use the variable (ANSI_CHAR_LIST is just an example name of the variable could be anything!)

Ansi based ($ANSI_CHAR_LIST)
![Screenshot_RASCII : fish_9](https://user-images.githubusercontent.com/101834410/204243768-4a15bb21-ba93-4979-bd4f-d8e8b1dc4112.png)

# Custom colored images!

![Screenshot_RASCII : fish_13](https://user-images.githubusercontent.com/101834410/204243664-749a1923-9284-4adf-a3a8-a8fcb9342791.png)

# Pixelart Convertion!

<p align="center">
  <img src="https://user-images.githubusercontent.com/101834410/204243571-f6697b6f-f27d-4da1-a75c-c2c51723978d.png">
</p>

<p align="center">
  <img src="https://user-images.githubusercontent.com/101834410/204244536-f1c3674a-2c96-4d00-a310-c5cff63d3348.png">
</p>

Or I could do an amogus! LOL

<p align="center">
  <img src="https://user-images.githubusercontent.com/101834410/204243525-ed62e0df-789d-4da8-a3a5-3919c548e050.png">
</p>

**If any error accurs please post it on issues section**
