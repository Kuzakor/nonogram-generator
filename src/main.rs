use crate::pixel_brightness::get_pixel_brightness;

mod colors;
mod crop;
mod generate_instructions;
mod output;
mod palette;
mod pixel_brightness;
mod pixelate;
mod point;
mod range;
mod zealous_crop;

fn main() {
    //STOLEN CODE
    //let image = pixelate(10, "./images/c.png");
    // MY CODE
    let pixel_brightness = get_pixel_brightness(image::open("./images/c.png").unwrap());
    let cols = generate_instructions::get_columns(&pixel_brightness);
    let rows = generate_instructions::get_rows(&pixel_brightness);
    //pr(cols, rows)
}

/* TODOO
fn pr(cols: Vec<Vec<u32>>, rows: Vec<Vec<u32>>) {
    let offset = rows.iter().max_by_key(|vec| vec.len()).unwrap().len();
    let max_coll = cols.iter().max_by_key(|vec| vec.len()).unwrap().len();
}
*/
