use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // read input
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("input.txt") {

        let mut distinct_answers = Vec::new();
        let mut same_answers = Vec::new();
        let mut answers = String::from("");
        for input_line in lines {
            if let Ok(line) = input_line {
                if line == "" {
                    distinct_answers.push(count_distinct(answers.clone()));
                    same_answers.push(count_same(answers.clone()));
                    let buf = &mut answers;
                    buf.clear();
                }
                else {
                    let buf = &mut answers;
                    buf.push_str(" ");
                    buf.push_str(&line);
                }
            }
        }
        distinct_answers.push(count_distinct(answers.clone()));
        same_answers.push(count_same(answers.clone()));
        println!("the sum of all distinct answers is {}", sum_vector(distinct_answers));
        println!("the sum of all same answers is {}", sum_vector(same_answers));
    }
}

fn sum_vector(vector: Vec<i32>) -> i32 {
    let mut sum = 0;
    for idx in 0..(vector.len()) {
        sum += vector[idx];
    }
    sum
}

fn translate_answer(answer:&str) -> [i32; 26] {
    let mut answer_translated:[i32; 26] = [0; 26];
    static ASCII_LOWER:[&str; 26] = [
    "a", "b", "c", "d", "e", 
    "f", "g", "h", "i", "j", 
    "k", "l", "m", "n", "o",
    "p", "q", "r", "s", "t", 
    "u", "v", "w", "x", "y", 
    "z",
    ];
    for idx in 0..26 {
        if answer.contains(ASCII_LOWER[idx]) {
            answer_translated[idx] = 1;
        }
    }
    answer_translated
}

fn count_same(answers:String) -> i32 {
    let mut answers_same: [i32; 26] = [1; 26];
    let answers_in_group:Vec<&str> = answers.split_whitespace().collect();
    for &one_answer in answers_in_group.iter() {
        let answer_translated = translate_answer(one_answer);
        for idx in 0..26 {
            if answer_translated[idx] == 0 {
                answers_same[idx] = 0
            }
        }
    }
    let sum = answers_same.iter().sum();
    // println!("{} has {} same answers", answers, sum);
    sum
}

fn count_distinct(answers:String) -> i32 {
    let mut answers_distinct: [i32; 26] = [0; 26];
    let answers_in_group:Vec<&str> = answers.split_whitespace().collect();
    for &one_answer in answers_in_group.iter() {
        let answer_translated = translate_answer(one_answer);
        for idx in 0..26 {
            if answer_translated[idx] == 1 {
                answers_distinct[idx] = 1
            }
        }
    }
    let sum = answers_distinct.iter().sum();
    // println!("{} has {} distinct answers", answers, sum);
    sum
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
