use image::{DynamicImage, GenericImage, GenericImageView};
use rayon::iter::IntoParallelRefMutIterator;

use rayon::prelude::*;

pub fn pixelate(output_size: u32, filename: &str) -> DynamicImage {
    let img = image::open(filename).unwrap();

    // do zealous square crop for images with lots of uneven transparency on edges
    // for example, pokemon pixel art often has this
    // otherwise, just do the pixelation on original
    let squared = false;
    let squared_output = false;

    let img = crate::zealous_crop::zealous_crop(&img, squared);
    // pixel_art_image::output(&img, "./output/cropped.png")?;

    // draw image to cli
    // pixel_art_image::print(&img);

    let palette = crate::palette::palette(&img);

    let palette_size = palette.len();

    let (width, height) = img.dimensions();

    let ratio = width as f32 / height as f32;
    let output_width;
    let output_height;

    match height > width {
        true => {
            output_width = (output_size as f32 * ratio) as u32;
            output_height = output_size;
        }
        false => {
            output_width = output_size;
            output_height = (output_size as f32 / ratio) as u32;
        }
    }

    let grid_scalar_width = width as f32 / output_width as f32;
    let grid_scalar_height = height as f32 / output_height as f32;
    let grid_width = width / output_width;
    let grid_height = height / output_height;

    // initialize vector for each palette color for each grid cell
    // e.g. [0, 0, 0] maps to [color_1, color_2, color_3]
    // we will increment the value at idnex when a pixel is closest to a particular color
    // end result will allow us to determine most representative palette color for a grid cell
    let mut color_counts = vec![];
    for _ in 0..output_width {
        let mut column = vec![];

        for _ in 0..output_height {
            let colors = vec![0; palette_size];
            column.push(colors);
        }

        color_counts.push(column);
    }

    let mut pixelated = DynamicImage::new_rgba8(output_width, output_height);

    color_counts
        .par_iter_mut()
        .enumerate()
        .for_each(|(grid_x, grid_column)| {
            grid_column
                .par_iter_mut()
                .enumerate()
                .for_each(|(grid_y, grid_cell)| {
                    let x_start = (grid_x as f32 * grid_scalar_width).floor() as u32;
                    let y_start = (grid_y as f32 * grid_scalar_height).floor() as u32;
                    let x_end = x_start + grid_width;
                    let y_end = y_start + grid_height;

                    for x in x_start..x_end {
                        for y in y_start..y_end {
                            let pixel = img.get_pixel(x, y);
                            let closest_index = crate::colors::closest_rgb(&palette, &pixel);
                            grid_cell[closest_index] += 1;
                        }
                    }
                });
        });

    // println!("[color_counts={:?}]", color_counts);

    // let mut pixelated = DynamicImage::new_rgba8(output_size, output_size);
    // let mut pixelated = RgbaImage::new(output_size, output_size);

    for y in 0..pixelated.height() {
        for x in 0..pixelated.width() {
            let grid_cell = color_counts
                .get(x as usize)
                .unwrap()
                .get(y as usize)
                .unwrap();

            // walk each palette color count in grid_cell vector
            // discover the highest count and color this pixel that color
            let mut found_max = false;
            let mut max_index = 0;
            let mut max_count = &0;
            for index in 0..grid_cell.len() {
                let count = grid_cell.get(index).unwrap();

                if count > max_count {
                    found_max = true;
                    max_index = index;
                    max_count = count;
                }
            }

            // debugging horse photo at 32x32 with zealous crop
            // had uneven borders due to square grid cell not aligning with landscape image
            // if y == 27 {
            //     println!("({},{}) = {:?}", x, y, grid_cell);
            // }

            // color pixel the palette color of max_index
            if found_max {
                let pixel = *palette.get(max_index).unwrap();
                // println!("({},{}) = {:?}", x, y, pixel);
                pixelated.put_pixel(x, y, pixel);
            }
        }
    }

    // try zealous cropping at this point once we are finished?
    // pixel_art_image::print(&pixelated);
    if squared_output {
        pixelated = crate::zealous_crop::zealous_crop(&pixelated, true);
    }

    pixelated
}
