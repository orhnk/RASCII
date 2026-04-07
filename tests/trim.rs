use image::{DynamicImage, Rgba, RgbaImage};
use rascii_art::RenderOptions;

fn make_bordered_image(content_w: u32, content_h: u32, border: u32, bg: Rgba<u8>) -> DynamicImage {
    let total_w = content_w + border * 2;
    let total_h = content_h + border * 2;
    let mut img = RgbaImage::from_pixel(total_w, total_h, bg);
    for y in border..(border + content_h) {
        for x in border..(border + content_w) {
            let r = ((x * 37 + y * 53) % 256) as u8;
            let g = ((x * 59 + y * 31) % 256) as u8;
            let b = ((x * 43 + y * 47) % 256) as u8;
            img.put_pixel(x, y, Rgba([r, g, b, 255]));
        }
    }
    DynamicImage::ImageRgba8(img)
}

#[test]
fn trim_removes_black_border() {
    let img = make_bordered_image(20, 20, 10, Rgba([0, 0, 0, 255]));
    let mut output = String::new();
    rascii_art::render_image_to(
        &img,
        &mut output,
        &RenderOptions::new().width(40).trim(true),
    )
    .unwrap();
    let mut no_trim_output = String::new();
    rascii_art::render_image_to(
        &img,
        &mut no_trim_output,
        &RenderOptions::new().width(40).trim(false),
    )
    .unwrap();
    let trim_blank = output.lines().filter(|l| l.trim().is_empty()).count();
    let no_trim_blank = no_trim_output
        .lines()
        .filter(|l| l.trim().is_empty())
        .count();
    assert!(
        trim_blank < no_trim_blank,
        "trim should remove empty border rows"
    );
}

#[test]
fn trim_preserves_content_only_image() {
    let img = make_bordered_image(20, 20, 0, Rgba([0, 0, 0, 255]));
    let mut trimmed = String::new();
    rascii_art::render_image_to(
        &img,
        &mut trimmed,
        &RenderOptions::new().width(20).trim(true),
    )
    .unwrap();
    let mut untrimmed = String::new();
    rascii_art::render_image_to(
        &img,
        &mut untrimmed,
        &RenderOptions::new().width(20).trim(false),
    )
    .unwrap();
    assert_eq!(trimmed.lines().count(), untrimmed.lines().count());
}

#[test]
fn trim_handles_fully_transparent_border() {
    let img = make_bordered_image(20, 20, 10, Rgba([0, 0, 0, 0]));
    let mut output = String::new();
    rascii_art::render_image_to(
        &img,
        &mut output,
        &RenderOptions::new().width(40).trim(true),
    )
    .unwrap();
    assert!(!output.is_empty());
}

#[test]
fn trim_handles_solid_image() {
    let img = DynamicImage::ImageRgba8(RgbaImage::from_pixel(20, 20, Rgba([50, 50, 50, 255])));
    let mut output = String::new();
    rascii_art::render_image_to(
        &img,
        &mut output,
        &RenderOptions::new().width(20).trim(true),
    )
    .unwrap();
    assert!(!output.is_empty());
}
