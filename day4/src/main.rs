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

fn check_passport(passport_line: String) -> bool {
    let fields_req = ["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"];

    for field in fields_req.iter() {
        if !passport_line.contains(field) {
            println!("{}\n--> no {}", passport_line, field);
            return false;
        }
    }
    check_entries(passport_line)
}

fn check_entries(passport_line: String) -> bool {

    // get passport entries by splitting the line at whitespace
    let entries: Vec<&str> = passport_line.split_whitespace().collect();
    for entry in entries.iter() {
        // collect key and value of the entry
        let entry: Vec<&str> = entry.split(':').collect();
        let key = entry[0];
        let value = entry[1];
        if check_values(key, value) == false {
            println!("pair {}:{} is false", key, value);
            return false;
        }
    }
    true
}

fn check_values(key: &str, value: &str) -> bool {
    if key == "byr" {
        let year = is_year(value);
        return 1920 <= year && year <= 2002;
    }
    else if key == "iyr" {
        let year = is_year(value);
        return 2010 <= year && year <= 2020;
    }
    else if key == "eyr" {
        let year = is_year(value);
        return 2020 <= year && year <= 2030;
    }
    else if key == "hgt" {
        return is_height(value);
    }
    else if key == "hcl" {
        return is_color_hex(value);
    }
    else if key == "ecl" {
        return ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value);
    }
    else if key == "pid" {
        return value.len() == 9 && is_num(value);
    }
    else if key == "cid" {
        return true;
    }
    false
}

fn is_height(value: &str) -> bool {
    let length = value.len();
    let hight = value[..(length - 2)].parse::<i32>().unwrap();
    let einheit = &value[(length - 2)..];
    // println!("hight is {} {}", hight, einheit);
    if einheit == "cm" {
        return 150 <= hight && hight <= 193;
    }
    else if einheit == "in" {
        return 59 <= hight && hight <= 76;
    }
    false
}

fn is_year(value: &str) -> i32 {
    if is_num(value) == false || value.len() != 4 {
        return 0
    }
    value.parse::<i32>().unwrap()
}

fn is_color_hex(value: &str) -> bool {
    if value.len() != 7 {
        return false;
    }
    let bytes = value.as_bytes();
    for (idx, &item) in bytes.iter().enumerate() {
        if idx == 0 && item != b'#' {
            return false
        }
        if idx != 0 && (item < b'0' || b'9' < item) &&
        (item < b'a' || b'f' < item) {
            return false;
        }
    }
    true
}

fn is_num(value: &str) -> bool {
    let bytes = value.as_bytes();

    for &item in bytes.iter() {
        if item < b'0' || b'9' < item {
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
