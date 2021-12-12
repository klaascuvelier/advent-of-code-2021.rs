use std::fs;

fn main() {
    let filename = "input.txt";
    let mut is_first = true;
    let mut previous = 0;
    let mut increases = 0;
    let mut decreases = 0;


    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    for line in contents.lines() {
          if is_first {
            is_first = false;
          } else {
            let number = line.parse::<i32>().unwrap();

            if number > previous {
                increases += 1;
            } else if number < previous {
                decreases += 1;
            }

            previous = number;
        }
    }

    println!("increases: {}", increases);
    println!("decreases: {}", decreases);
}
