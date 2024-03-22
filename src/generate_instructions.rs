pub fn get_columns(pixels: &Vec<Vec<bool>>) -> Vec<Vec<u32>> {
    let mut instructions: Vec<Vec<u32>> = Vec::new();
    for line in pixels {
        instructions.push(counter_logic(line));
    }
    instructions
}

pub fn get_rows(pixels: &Vec<Vec<bool>>) -> Vec<Vec<u32>> {
    let mut rotated = pixels.clone();

    for i in 0..pixels.len() {
        for j in 0..pixels[0].len() {
            rotated[j][i] = pixels[i][j];
        }
    }

    get_columns(&rotated)
}

fn counter_logic(line: &Vec<bool>) -> Vec<u32> {
    let mut instruction: Vec<u32> = Vec::new();
    let mut counter = 0;
    for j in line {
        if *j {
            counter += 1;
            continue;
        }
        if counter > 0 {
            instruction.push(counter);
        }
        counter = 0;
    }
    if line[line.len() - 1] {
        instruction.push(counter);
    }
    instruction
}
