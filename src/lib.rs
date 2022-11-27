use image::{imageops::FilterType::Gaussian, GenericImageView, ImageError, Rgba};

use rgb2ansi256::rgb_to_ansi256;

use std::{env, io, process};

type XyA = Vec<((u32, u32), u8)>;
type XyRgb = Vec<((u32, u32), (u8, u8, u8))>;
type XyRgba = Vec<((u32, u32), (u8, u8, u8, u8))>;

pub trait Image {
    fn take_saturation(&self) -> XyA;
    fn take_color(&self) -> XyRgb;
    fn take_appearance(&self) -> XyRgba;
}

#[derive(Debug)]
pub struct Img {
    body: Vec<Pixel>,
}

/// # Implented for:
///
/// creating a custom data type that represents a pixel in an image
///
/// #Contains:
///
/// ((posx, posy), (r, g, b, a))
///
/// # Destruction:
///
/// You could destructure it with public structs or (I will create methods for this [I hope so :D])
#[derive(Debug)]
pub struct Pixel((u32, u32), (u8, u8, u8, u8));

/// # Implented for:
///
/// to be able to take inner struct values easily.
///
/// # Contains:
///
/// I thought that it would be better to have positions (x, y) in first element.
///
/// Look at the code! you'll understand better.
impl Image for Img {
    fn take_saturation(&self) -> Vec<((u32, u32), u8)> {
        let mut result = Vec::new();
        for pixel in self.body.as_slice() {
            let Pixel((x, y), (_, _, _, saturation)) = pixel;
            result.push(((*x, *y), *saturation));
        }
        result
    }
    fn take_color(&self) -> Vec<((u32, u32), (u8, u8, u8))> {
        let mut result = Vec::new();
        for pixel in self.body.as_slice() {
            let Pixel((x, y), (r, g, b, _)) = pixel;
            result.push(((*x, *y), (*r, *g, *b)));
        }
        result
    }
    fn take_appearance(&self) -> Vec<((u32, u32), (u8, u8, u8, u8))> {
        let mut result = Vec::new();
        for pixel in self.body.as_slice() {
            let Pixel((x, y), (r, g, b, saturation)) = pixel;
            result.push(((*x, *y), (*r, *g, *b, *saturation)));
        }
        result
    }
}

impl Img {
    pub fn new() -> Self {
        Img { body: Vec::new() }
    }
}

/// Allows you to take input from standard input.
pub fn take_input(reference: &mut String) -> Result<usize, io::Error> {
    io::stdin().read_line(reference)
}

/// # This function returns all pixels in the given path
///
/// Its Important to know `&path[..&path.len()-1]` does not include last 2 elements of path!
///
/// But why would we want to do this?
///
/// Because when you press <enter> on terminal to actually register the path, You are placing
///
/// an additional `\n` to input! This is the reason I am not including last 2 elements.
///
/// # Functionality:
///
/// This decodes image in the path. also also translates the image output to standard Vec.

pub fn convert() -> String {
    let sub_commands = read_env_args();
    if sub_commands.len() < 2 {
        println!(
            "
   'l_?]]?<:     .;~?]]]]-<;.                                 ':i+-?]]]]~   ![~   ']]\"
  :(/-!;I>[f?.  I(/[>l;;l>[/);                              ^-(1->lI;;;;\"   ]c(.  \"rul
 ]n>     `|xI  ,tfl        !j/\"                            ^(j<'            ]c(.  \"rul
'/n!     `|xI  +u?          [u<     '           `\"\"^      |~u[              ]c(.  \"rul
'/z/~;:;i[f?.  [u>          ~u?   `_)>       ,~[[}}1)~o    |-v~              ]c(.  \"rul
'/u]][)xf-:    [ui          <u]  `)j<.     :]/[!^  \")xi   %~u[              ]c(.  \"rul
'/r:  ^]f>     [v[i!!!!!!!!i]v[  ln('    ,?/1!.    '[n>    ^(j<'            ]c(.  \"rul
'/r,   '[/i    [c(?--------?|c]  ^(f+IIi]|1i.     ^?t?'     ^-(1->lI;;;;\"   ]c(.  \"rul
 _]`    '+_'   >[:          ;]i   `i?]]-<:        ,+;        `:i+-?]]]]~    ![~   ']]\"
"
        );
        process::exit(1);
    }
    if sub_commands.iter().any(|i| i == "-h" || i == "--help") {
        print_help();
    }
    let reverse = sub_commands.iter().any(|i| i == "-i" || i == "--invert");
    let colored = sub_commands
        .iter()
        .any(|i| i == "-c" || i == "--colored" || i == "--color");
    let path = if sub_commands.iter().any(|i| i == "-p" || i == "--path") {
        &sub_commands[&sub_commands
            .iter()
            .position(|i| i == "-p" || i == "--path")
            .unwrap_or_else(|| {
                eprint!(
                    "
            You have to enter: 
            rascii -p <path>
            /* or */
            rascii --path <path>
            -> to be able to convert an image in path!
            -> Program has to know where is the image you are looking!
                "
                );
                process::exit(1);
            })
            + 1]
    } else {
        eprintln!(
            "Help adding:\n
    rascii --path <path>\n
    rascii -p <path>\n
        to your command
    for more help type:
        rascii --help or -h"
        );
        process::exit(1);
    };

    let open_img = |path: &str| {
        let img = image::open(path)?;
        let img = img.resize_exact(img.width(), img.height() / 2, Gaussian);
        let pixels: Vec<(u32, u32, Rgba<u8>)> = img.pixels().into_iter().collect();
        let mut result: Img = Img::new();
        // let mut result:Vec<Pixel> = Vec::new();

        // let Rgba([x,y,z,a]) = result[0].2;
        for i in pixels.iter() {
            let (ax, ay, Rgba([x, y, z, v])) = i;
            result.body.push(Pixel((*ax, *ay), (*x, *y, *z, *v)));
        }

        // println!("{:?}", result); // To use it while debugging!
        Ok::<Img, ImageError>(result)
    };

    let img = open_img(path).unwrap_or_else(|_| {
        eprint!("Couldn't open image file in path!");
        process::exit(1);
    });

    let sat = img.take_appearance();

    // Multiple white_spaces to be able to capture slight grayish white colors!
    let char_list = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'.    "; // currently 70 elements. But you can modify this list and code will adabt to it.
    let mut regulated: Vec<((u32, u32), f64, u8)> = Vec::with_capacity(sat.len());
    for i in sat.iter() {
        let ((x, y), (r, g, b, a)) = i;
        let fine = (
            (*x, *y),
            *r as f64 / 1020_f64
                + *g as f64 / 1020_f64
                + *b as f64 / 1020_f64
                + *a as f64 / 1020_f64,
            (rgb_to_ansi256(*r, *g, *b)),
        ); // 765 is 255*3 , when i sum them out,  I want to use them as 0-1 decimal numbers.
        regulated.push(fine);
    }
    // println!("{regulated:?}");

    let mut y_init = 0; // newline counter
    let regulated: String = regulated
        .iter()
        .map(|i| {
            let index = if !reverse {
                (i.1 * (char_list.len() - 1) as f64) as usize
            } else {
                (char_list.len() - 1) - (i.1 * (char_list.len() - 1) as f64) as usize
            };
            let ascii_char = char_list
                .chars()
                .nth(index)
                .unwrap_or_else(|| {
                    eprintln!(
                        "
                You entered an unvalid path!
                If your path has white_spaces, you have to change the name of the file!
                Exmpl:
                    rascii -p Img (1).jpeg -> unvalid!
                    rascii -p Img(1).jpeg -> valid!
                    "
                    );
                    process::exit(1);
                })
                .to_string();
            // print!("{}", &char_list.chars().nth(index).expect("This cannot raise an error (I guess you did not entered a valid path)"));
            if i.0 .1 > y_init {
                /* the y value of pixel */
                y_init = i.0 .1;
                if !colored {
                    print!("\n{}", ascii_char);
                    return stringify!("\n{}", ascii_char);
                } else {
                    let color_code = i.2;
                    let result = format!("\n{}{}", color_code, ascii_char);
                    print!("{result}");
                    return stringify!(result);
                }
            };
            if !colored {
                print!("{ascii_char}");
                stringify!(ascii_char)
            } else {
                let color_code = i.2;
                let result = format!("{}{}", color_code, ascii_char);
                print!("{result}");
                stringify!(result)
            }
        })
        .collect();
    regulated
}

pub fn read_env_args() -> Vec<String> {
    let mut ret = Vec::new();
    for arg in env::args() {
        ret.push(arg);
    }
    ret
}

fn print_help() {
    println!("This program allows you to create ASCII art from any image (.png, .jpeg, .jpg ...)
Commands (rascii <Command>):
    -p, --path <path/to/image> // given path is the image path that you want to convert.
    -i, --invert // to be able to create images without white background. (if your image has a white plain and you want to ignore it use this command)
    -c, --colored // (not implemented yet!)
    -h, --help // to acces this window

    |If you dont understand, Just ask your mom|
    ||This project is rusty, so it is fast! really!||
    ");
    process::exit(1);
}
