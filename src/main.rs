use rascii::{
    convert,
    // Image, // Had to bring if not, rust wont see we are using the trait
};

fn main() {
    convert(); // second parameter takes a boolean to make the image colored, third one takes a boolean to reverse the image (prevent black background).
               // println!("{:#?}", raw_pxs.take_saturation()[0]); // To be able to use methods included in the trait,  we must use the trait!
               // println!("{:#?}", raw_pxs.take_appearance()[0]);
               // println!("{:#?}", raw_pxs.take_color()[0]);
}
