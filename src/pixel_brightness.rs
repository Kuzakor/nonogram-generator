use image::{DynamicImage, GenericImageView};

pub fn get_pixel_brightness(img: DynamicImage) -> Vec<Vec<bool>> {
    let mut result: Vec<Vec<bool>> = empty_vec(img.height(), img.width());
    let gray = img.grayscale();
    for y in 0..img.height() {
        for x in 0..img.width() {
            if gray.get_pixel(x, y).0[0] == 0 {
                result[x as usize][y as usize] = true
            }
        }
    }

    result
}

fn empty_vec(x: u32, y: u32) -> Vec<Vec<bool>> {
    let mut v: Vec<Vec<bool>> = Vec::new();
    for _ in 0..y {
        let mut mv: Vec<bool> = Vec::new();
        for _ in 0..x {
            mv.push(false)
        }
        v.push(mv)
    }
    v
}
