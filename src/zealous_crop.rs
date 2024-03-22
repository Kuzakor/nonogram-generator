use core::fmt::Debug;
use std::cmp;

use image::DynamicImage;
use image::GenericImage;
use image::GenericImageView;
use image::Pixel;

use crate::crop::Crop;
use crate::point::Point;
use crate::range::range;

// squared will output the crop into a square, centering content
pub fn zealous_crop(img: &DynamicImage, squared: bool) -> DynamicImage {
    let crop = get_zealous_crop(img);
    let mut x_start = 0;
    let mut y_start = 0;
    let mut width = crop.width();
    let mut height = crop.height();

    let (_img_width, _img_height) = img.dimensions();

    if squared {
        let delta = width.abs_diff(height);

        match width > height {
            true => y_start += delta / 2,
            false => x_start += delta / 2,
        }

        let size = cmp::max(width, height);
        width = size;
        height = size;
    }

    // copy source pixels to image buffer and save to view cropped image
    let mut cropped_img = DynamicImage::new_rgba8(width, height);

    for x in 0..crop.width() {
        for y in 0..crop.height() {
            // println!("pixel({:>3}, {:>3})", crop.left + x, crop.top + y);
            // grab from source
            let pixel = img.get_pixel(crop.left + x, crop.top + y);
            // place into cropped image buffer
            cropped_img.put_pixel(x_start + x, y_start + y, pixel);
        }
    }

    cropped_img
}

pub fn get_zealous_crop(img: &DynamicImage) -> Crop {
    let mut crop = Crop {
        ..Default::default()
    };

    // top edge
    // scan from top to bottom, going left to right
    crop.top = scan_edge(img, Scan::TopToBottom, is_pixel_not_alpha);

    // right edge
    // scan from right to left, going top to bottom
    crop.right = scan_edge(img, Scan::RightToLeft, is_pixel_not_alpha);

    // bottom edge
    // scan from bottom to top, going left to right
    crop.bottom = scan_edge(img, Scan::BottomToTop, is_pixel_not_alpha);

    // left edge
    // scan from left to right, going top to bottom
    crop.left = scan_edge(img, Scan::LeftToRight, is_pixel_not_alpha);

    crop
}

fn scan_edge(img: &DynamicImage, scan: Scan, test: fn(&DynamicImage, Point) -> bool) -> u32 {
    let (width, height) = img.dimensions();

    let (range_a_start, range_a_end, range_b_start, range_b_end) = match scan {
        Scan::LeftToRight => (0, width, 0, height),
        Scan::RightToLeft => (width, 0, 0, height),
        Scan::TopToBottom => (0, height, 0, width),
        Scan::BottomToTop => (height, 0, 0, width),
    };

    for a in range(range_a_start, range_a_end) {
        for b in range(range_b_start, range_b_end) {
            // put a and b into the correct x,y variables
            let (x, y) = match scan {
                Scan::LeftToRight | Scan::RightToLeft => (a, b),
                Scan::TopToBottom | Scan::BottomToTop => (b, a),
            };

            if test(img, Point { x, y }) {
                return a;
            }
        }
    }

    range_a_end
}

fn is_pixel_not_alpha(img: &DynamicImage, point: Point) -> bool {
    let pixel = img.get_pixel(point.x, point.y);

    if let [_, _, _, alpha] = pixel.channels() {
        if *alpha != 0 {
            return true;
        }
    }

    false
}

#[derive(Debug)]
enum Scan {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}
