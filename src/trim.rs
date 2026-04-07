use image::{DynamicImage, GenericImageView};

const VARIANCE_EPSILON: f64 = 30.0;

fn grayscale(r: u8, g: u8, b: u8) -> f64 {
    0.299 * r as f64 + 0.587 * g as f64 + 0.114 * b as f64
}

fn row_variance(image: &DynamicImage, y: u32) -> f64 {
    let w = image.width();
    if w == 0 {
        return 0.0;
    }
    let mut sum = 0.0;
    let mut sum_sq = 0.0;
    let mut count = 0u32;
    for x in 0..w {
        let px = image.get_pixel(x, y);
        if px[3] == 0 {
            continue;
        }
        let g = grayscale(px[0], px[1], px[2]);
        sum += g;
        sum_sq += g * g;
        count += 1;
    }
    if count == 0 {
        return 0.0;
    }
    let mean = sum / count as f64;
    sum_sq / count as f64 - mean * mean
}

fn col_variance(image: &DynamicImage, x: u32) -> f64 {
    let h = image.height();
    if h == 0 {
        return 0.0;
    }
    let mut sum = 0.0;
    let mut sum_sq = 0.0;
    let mut count = 0u32;
    for y in 0..h {
        let px = image.get_pixel(x, y);
        if px[3] == 0 {
            continue;
        }
        let g = grayscale(px[0], px[1], px[2]);
        sum += g;
        sum_sq += g * g;
        count += 1;
    }
    if count == 0 {
        return 0.0;
    }
    let mean = sum / count as f64;
    sum_sq / count as f64 - mean * mean
}

pub fn trim_image(image: &DynamicImage) -> DynamicImage {
    let (w, h) = (image.width(), image.height());
    if w == 0 || h == 0 {
        return image.clone();
    }

    let mut top = 0u32;
    for y in 0..h {
        if row_variance(image, y) >= VARIANCE_EPSILON {
            break;
        }
        top = y + 1;
    }

    let mut bottom = h;
    for y in (0..h).rev() {
        if row_variance(image, y) >= VARIANCE_EPSILON {
            break;
        }
        bottom = y;
    }

    let mut left = 0u32;
    for x in 0..w {
        if col_variance(image, x) >= VARIANCE_EPSILON {
            break;
        }
        left = x + 1;
    }

    let mut right = w;
    for x in (0..w).rev() {
        if col_variance(image, x) >= VARIANCE_EPSILON {
            break;
        }
        right = x;
    }

    if top >= bottom || left >= right {
        return image.clone();
    }

    image.crop_imm(left, top, right - left, bottom - top)
}
