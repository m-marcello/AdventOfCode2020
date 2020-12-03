use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    // read input and create grid from it
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("input.txt") {
        let width: usize = 31;
        let mut hight: usize = 0;
        let mut grid = vec![vec![0; width]; 1];

        // copy input lines into the grid, 0 for '.' and 1 for '#'
        for line in lines {
            if let Ok(grid_line) = line {
                for idx in 0..width {
                    if grid_line.chars().nth(idx).unwrap() == '#' {
                        grid[hight][idx] = 1;
                    }
                }
                hight += 1;
                let fresh_line = vec![0; width];
                grid.push(fresh_line);
            }
        }
        println!("there are {} trees", count_trees(grid, width, hight));
    }
}

fn count_trees(grid: Vec<Vec<i32>>, width: usize, hight: usize) -> i32 {
    let off_width: usize = 1;
    let off_hight: usize = 2;
    let mut count: i32 = 0;
    let mut idx_hight: usize = 0;
    let mut idx_width: usize = 0;
    while idx_hight <= hight {
        count = count + grid[idx_hight][idx_width % width];
        idx_hight = idx_hight + off_hight;
        idx_width = idx_width + off_width;
    }
    count
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



// Right 1, down 1. 60
// Right 3, down 1. 191
// Right 5, down 1. 64
// Right 7, down 1. 63
// Right 1, down 2. 32
