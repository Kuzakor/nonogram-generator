use std::cmp;

use image::GenericImageView;
use image::{DynamicImage, Pixel, Rgba};

// use usize (not u8) to be able to hold the cubed result
// e.g. 8^3 = 512 > u8::MAX
static PARTITIONS: usize = 3;
static OUTPUT_COLOR_COUNT: usize = 32;

pub fn palette(img: &DynamicImage) -> Vec<Rgba<u8>> {
    let mut color_space: Vec<Vec<Rgba<u8>>> = Vec::new();

    // PARTITIONS^3 for each value in rgb
    // equivalent to 3 nested for loops
    // e.g.
    //   for r in 0..PARTITIONS {
    //     for g in 0..PARTITIONS {
    //         for b in 0..PARTITIONS {
    for _ in 0..PARTITIONS.pow(3) {
        color_space.push(Vec::new());
    }

    let mut used_pixel_count = 0;

    for (_total_pixel_count, (_x, _y, pixel)) in img.pixels().enumerate() {
        if let Some(color_space_index) = get_pixel_index(&pixel) {
            used_pixel_count += 1;

            let space = color_space.get_mut(color_space_index).unwrap();
            space.push(pixel);
        }
    }

    // sort by number of pixels in each partition
    color_space.sort_by_key(|b| std::cmp::Reverse(b.len()));

    let output_count = cmp::min(OUTPUT_COLOR_COUNT, color_space.len());
    let mut output = vec![];

    for i in 0..output_count {
        let space = color_space.get(i).unwrap();

        let space_pixel_count = space.len() as u32;
        let (mut red, mut green, mut blue) = (0_u32, 0_u32, 0_u32);

        for pixel in space {
            if let [r, g, b, _] = pixel.channels() {
                red += *r as u32;
                green += *g as u32;
                blue += *b as u32;
            }
        }

        let (a_red, a_green, a_blue) = (
            average_color(red, space_pixel_count),
            average_color(green, space_pixel_count),
            average_color(blue, space_pixel_count),
        );

        let average_pixel = Rgba([a_red, a_green, a_blue, 255]);

        output.push(average_pixel);
    }

    output.push(Rgba::from([0, 0, 0, 0]));

    output
}

fn partition_len() -> u8 {
    (u8::MAX as f32 / PARTITIONS as f32).ceil() as u8
}

fn get_index(r: u8, g: u8, b: u8) -> usize {
    // println!("{},{},{}", r, g, b);
    (r as usize * PARTITIONS.pow(0))
        + (g as usize * PARTITIONS.pow(1))
        + (b as usize * PARTITIONS.pow(2))
}

fn get_pixel_index(pixel: &Rgba<u8>) -> Option<usize> {
    if let [r, g, b, alpha] = pixel.channels() {
        if *alpha == 0 {
            return None;
        }

        let index = get_index(
            // force line break
            get_partition(r),
            get_partition(g),
            get_partition(b),
        );

        return Some(index);
    };

    None
}

fn get_partition(color: &u8) -> u8 {
    // ensure color isn't max
    let mut color = *color;
    if color == u8::MAX {
        color -= 1
    }

    color / partition_len()
}

fn average_color(total: u32, count: u32) -> u8 {
    if count == 0 {
        return 0;
    }

    (total / count) as u8
}
