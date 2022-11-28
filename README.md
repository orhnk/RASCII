![Screenshot_Editing RASCII_README md at main · KoBruhh_RASCII - Brave_1](https://user-images.githubusercontent.com/101834410/204127025-b98aaf39-778b-468b-8f41-36fd858708e8.png)

# RASCII
image to ascii art fully created with rust 🦀 🚀

# Installation

FOR LINUX:

`git clone https://github.com/KoBruhh/RASCII.git`
`cd RASCII`
`./INSTALL`

`If any error accurs please post it on issues section`

FOR WİNDOWS:

`git clone https://github.com/KoBruhh/RASCII.git`

`cd RASCII`

`cargo build --release`

# Usage:

# Linux:
```shell
~This program allows you to create ASCII art from any image (.png, .jpeg, .jpg ...)~
SubCommands (rascii <SubCommand>):
-p, --path <path/to/image> // given path is the image path that you want to convert.
-i, --invert // to be able to create images without white background. (if your image has a white plain and you want to ignore it use this command)
-c, --colored // to make ASCII art colored
-h, --help // to access this window
-bg, --background // to paint background with the color of the pixel!
-wc, --with-color <R> <G> <B> // to create custom colored ASCII arts! you could mix these with -bg !
-px, --pixelated // to convert output to pixelart
-l <your_char_list_or `Available lists at bottom appendix I`> --list <your_char_list_or `Appendix I`> // allows you to create custom lists! and use builtin char_lists!

|If you dont understand, Just ask your mom|
||This project is rusty, so it is fast! really!||

Appendix I
Don't forget that you could enter your own list by:
rascii -p <path/to/path> -l <your_char_list_or || names below!>
Exmpl:
rascii -p <path/to/path> -l bruh // creats your image by using ['b', 'r', 'u', 'h']
rascii -p <path/to/path> -l emoji // creates your image by using emojis -> bcuz list below contains a list named: "emoji"!
Builtin_lists: [
"slight",
"russian",
"chinese",
"emoji",
"japanese",
"ansi",
]
```
of course you could mix these commands

# Windows:
```shell
`cd target/release/` //to be able to 
`./rascii --help` // to get help
 REST IS SAME WITH LINUX! but you have to do `./rascii` instead of `rascii`
```

# Samples:

Im on linux so:
`rascii -p /photos/ferris.jpeg -i -c >> ferris.txt` -> It just stores the output on ferris.txt, If you want to see the results only (not store) you're gonna do: `rascii -p /photos/ferris.jpeg -i -c`




Or I could do an amogus! LOL

![Screenshot_RASCII 1 : fish_2](https://user-images.githubusercontent.com/101834410/204127907-eddd1e68-4442-4eb7-bc58-28216ae68020.png)

# Read Documentation / Code

Go inside the directory you entered `./INSTALL`

`cargo doc --open`
