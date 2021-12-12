use std::fs;

fn main() {
    let filename = "input.txt";
    let mut horizontal = 0;
    let mut depth = 0;

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    for line in contents.lines() {
        let mut iter = line.splitn(2, ' ');
        let command = iter.next().unwrap();
         let distance_str  = iter.next().unwrap();
         let distance = distance_str.parse::<i32>().unwrap();

        match command {
            "down" => depth += distance,
            "up" => depth -= distance,
            "forward" => horizontal += distance,
            _ => println!("command not found: {}", command)
        }
    }

    println!("horizontal: {}", horizontal);
    println!("depth: {}", depth);
    println!("total: {}", horizontal * depth);
}
