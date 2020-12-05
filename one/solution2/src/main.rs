use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse(path: &str) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    // Parse
    let input = File::open(path).expect("File error");
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        let linestr = line.unwrap();
        output.push(linestr);
    }
    output
}

fn main() {
    let path = "input";
    let mut solution = (0, 0, 0);
    const RESULT: usize = 2020;
    let expenses = parse(path);
    let expenses2 = expenses.clone();
    let expenses3 = expenses.clone();

    for expense in &expenses {
        for expense2 in &expenses2 {
            for expense3 in &expenses3 {
                let expense_num: usize = expense.parse().unwrap();
                let expense2_num: usize = expense2.parse().unwrap();
                let expense3_num: usize = expense3.parse().unwrap();
                if expense_num + expense2_num + expense3_num == RESULT {
                    solution = (expense_num, expense2_num, expense3_num);
                }
            }
        }
    }
    println!("Expenses found {:?}", solution);
    println!("Solution {}", solution.0 * solution.1 * solution.2);
}
