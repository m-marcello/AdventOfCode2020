use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // read input and create grid from it
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("input.txt") {

        let mut plane: [i32; 1024] = [0; 1024];
        // copy input lines into the grid, 0 for '.' and 1 for '#'
        for input_line in lines {
            if let Ok(line) = input_line {
                let mut boarding_pass: [i32; 10] = [0; 10];
                for (idx, &item) in line.as_bytes().iter().enumerate() {
                    if item == b'B' || item == b'R' {
                        boarding_pass[idx] = 1;
                    }
                }
                let seat_id = get_seatid(boarding_pass) as usize;
                plane[seat_id as usize] = 1;
            }
        }
        println!("highest id is {}", get_highest(plane));
        println!("missing seats are:");
        print_missing(plane);
    }
}

fn print_missing(plane: [i32; 1024]) {
    for idx in 1..965 {
        if plane[idx] == 0 && plane[idx - 1] == 1 && plane[idx + 1] == 1 {
            println!("id = {}", idx);
        }
    }
}

fn get_highest(plane: [i32; 1024]) -> usize {
    let mut highest: usize = 0;
    for idx in 0..1024 {
        if plane[idx] == 1 {
            highest = idx;
        }
    }
    highest
}

fn get_position(area: &mut [i32], pass_pos: &[i32]) -> i32 {
    let mut area_slice = &area[..];
    for item in pass_pos {
        let half = (area_slice.len() / 2) as i32;
        let slice_start: usize = (item * half) as usize;
        let slice_end: usize = ((item + 1) * half) as usize;
        area_slice = &area_slice[slice_start..slice_end];
    }
    area_slice[0]
}

fn fill_rows() -> [i32; 128] {
    let mut rows: [i32; 128] = [0; 128];
    for i in 0..128 {
        rows[i] = i as i32;
    }
    rows
}

fn get_seatid(boarding_pass: [i32; 10]) -> i32 {
    let mut rows: [i32; 128] = fill_rows();
    let mut cols: [i32; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    let row = get_position(&mut rows, &boarding_pass[..7]);
    let col = get_position(&mut cols, &boarding_pass[7..]);
    row * 8 + col
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
