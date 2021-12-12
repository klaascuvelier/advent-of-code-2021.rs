use std::fs;

fn main() {
    let filename = "input.txt";
    let mut rotated: Vec<Vec<i32>> = Vec::new();
    let mut most_common: Vec<i32> = Vec::new();

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    for (line_index, line) in contents.lines().enumerate() {
        for (index, character) in line.chars().enumerate() {
            let mut val = 0;

            match character {
                '1' => val = 1,
                '0' => val = 0,
                _ => println!("invalid char {}", character)
            }

            if line_index == 0 {
                rotated.push(Vec::new());
            }
            rotated[index].push(val)
        }
    }

    for row in rotated.iter() {
        let mut zero_count = 0;
        let mut one_count = 0;

        for val in row.iter() {
            match val {
                0 => zero_count += 1,
                1 => one_count += 1,
                _ => println!("invalid bit {}", val)
            }
        }

        let most_common_char = if zero_count > one_count { 0 } else { 1 };

        most_common.push(most_common_char);
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    let base: i32 = 2;

    for (index, bit) in most_common.iter().rev().enumerate() {
        match bit {
            0 => epsilon += base.pow(index as u32),
            1 => gamma += base.pow(index as u32),
            _ => {}
        }
    }

    println!("gamma {}", gamma);
    println!("epsilon {}", epsilon);
    println!("power consumption {}", gamma * epsilon);
}
