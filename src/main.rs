use crate::pixel_brightness::get_pixel_brightness;

use crate::pixelate::pixelate;

mod colors;
mod crop;
mod generate_instructions;
mod palette;
mod pixel_brightness;
mod pixelate;
mod point;
mod range;
mod zealous_crop;

fn main() {
    println!("Input filename: ");
    let filename = input();
    println!("Input size (default: 10): ");
    let size:u32 = input().as_str().parse().unwrap_or(10);

    //STOLEN CODE
    let image = pixelate(size, filename.as_str());
    // MY CODE


    let pixel_brightness = get_pixel_brightness(image);
    let cols = generate_instructions::get_columns(&pixel_brightness);
    let rows = generate_instructions::get_rows(&pixel_brightness);
    println!("{}", pr(cols, rows));

    //let printers = printers::get_printers();


}


fn pr(cols: Vec<Vec<u32>>, rows: Vec<Vec<u32>>) -> String{
    let mut output = String::new();

    let offset = rows.iter().max_by_key(|vec| vec.len()).unwrap().len() * 2 + 1;
    let longest_row = get_line(&rows.iter().max_by_key(|vec| get_line(vec).len()).unwrap()).len();
    output += print_cols(cols.clone(), offset).as_str();

    for i in rows {
        output.push('\n');
        let line = get_line(&&i);
        output += line.as_str();
        for _ in 0..(longest_row - line.len() + 1) {
            output.push(' ')
        }
        for _ in &cols {
            output += "⬚ ";
        }
    }

    output
}

fn get_line(line: &&Vec<u32>) -> String{
    let mut str = String::new();
    for i in 0..line.len() {
        str += line[i].to_string().as_str();
        str.push(' ');
    }
    str
}

fn print_cols(cols: Vec<Vec<u32>>, offset: usize) -> String{
    let mut output = String::new();
    let mut sorted = cols.clone();
    loop {

        if sorted[0].is_empty() {
            break
        }

        output.push('\n');
        for _ in 0..offset {
            output.push(' ')
        }

        sorted.sort_by_key(|vec| vec.len());
        let mut biggest:Vec<Vec<u32>> = sorted.iter().filter(|vec| vec.len() == sorted[0].len()).cloned().collect();
        for i in &mut biggest{
            let last_index = i.len() - 1;
            output += format!("{} ", i[last_index]).as_str();
            i.remove(last_index);
        }
        for i in 0..biggest.len() {
            sorted[i] = biggest[i].to_vec()
        }
    }
    output
}


fn input() -> String{
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line).unwrap();
    line.pop();
    line
}