use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // read input and create grid from it
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("input.txt") {

        // copy input lines into the grid, 0 for '.' and 1 for '#'
        let mut passport = String::from("");
        let mut count = 0;
        for input_line in lines {
            println!("start again");
            if let Ok(line) = input_line {
                if line == "" {
                    if check_passport(passport.clone()) == true {
                        count += 1;
                    }
                    let buf = &mut passport;
                    buf.clear();
                }
                let buf = &mut passport;
                buf.push_str(" ");
                buf.push_str(&line);
                // println!("string after push: {}", passport);
            }    
        }
        if check_passport(passport.clone()) == true {
            count += 1;
        }
        println!("there are {} correct passports", count);
    }
}

fn check_passport(passport: String) -> bool {
    let fields_req = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    // let fields_opt = ["cid"];
    for field in fields_req.iter() {
        if !passport.contains(field) {
            println!("{} is not in passport {}", field, passport);
            return false;
        }
    }
    true
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
