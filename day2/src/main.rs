use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    // read input and create integer array from it
    // File hosts must exist in current path before this produces output
    let mut count: i32 = 0;
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(input_line) = line {
                println!("line '{}'", input_line);
                if check_password(input_line) == true {
                    count += 1;
                }
            }
        }
    }
    println!("there are {} correct passwords.", count);
}

fn check_password(line: String) -> bool {
    
    // parse relevant information from input string
    let mut line_split = line.split_whitespace();
    // if line_split.count() != 3 {
    //     panic!("line '{}' is incorrect format.", line);
    // }
    let first_entry: &str = line_split.next().unwrap();
    let second_entry: &str = line_split.next().unwrap();
    // let passwd: String = line_split[2].to_string();
    // let c: char = line_split[1].to_string().chars().nth(0).unwrap();
    // let line_split2: Vec<&str> = line_split[0].to_string().split("-").collect();
    let passwd: &str = line_split.next().unwrap();
    let character = second_entry.chars().nth(0).unwrap();
    let line_split2: Vec<&str> = first_entry.split("-").collect();
    if line_split2.len() != 2 {
        panic!("line '{}' is incorrect format, beginning is incorrect.", line);
    }
    let small: usize = line_split2[0].to_string().parse().unwrap();
    let big: usize = line_split2[1].to_string().parse().unwrap();
    
    // count how often the character appears in the password
    let mut count: usize = 0;
    for c in passwd.chars() {
        if character == c {
            count += 1;
        }
    }
    if small <= count && count <= big {
        println!("correct\n");
        return true
    }
    false
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
