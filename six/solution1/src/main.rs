use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse(path: &str) -> Vec<HashMap<String, String>> {
    let mut answers: Vec<HashMap<String, String>> = Vec::new();
    // Parse
    let input = File::open(path).expect("File error");
    let buffered = BufReader::new(input);

    let mut answer: HashMap<String, String> = HashMap::new();
    for line in buffered.lines() {
        let linestr = line.unwrap();
        // println!("{}", line.unwrap());
        if linestr.is_empty() {
            answers.push(answer.clone());
            answer = HashMap::new();
        } else {
            for key in linestr.chars() {
                answer.insert(key.to_string(), "".to_string());
            }
        }
    }
    // Fix last line if input file does not contain blank line at the end
    answers.push(answer.clone());
    answers
}

fn main() {
    let path = "input";
    let mut nb_valid_answers = 0;
    let answers: Vec<HashMap<String, String>>;

    answers = parse(path);

    for answer in answers {
        nb_valid_answers += answer.keys().len();
        println!("Valid passwords: {}", nb_valid_answers);
    }

    println!("Answers : {}", nb_valid_answers);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_fake() {
        assert_eq!(1, 1);
    }
}
