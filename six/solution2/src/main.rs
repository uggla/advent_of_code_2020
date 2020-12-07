use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn check_result(answer_nb: usize, answer: &mut HashMap<String, String>) {
    if answer_nb > 1 {
        for (key, value) in answer.clone() {
            let ivalue: usize = value.parse().unwrap();
            if ivalue == 0 {
                answer.remove(&key);
            }
            if ivalue + 1 == answer_nb {
                answer.insert(key, "ok".to_string());
            }
        }
    }
}

fn parse(path: &str) -> Vec<HashMap<String, String>> {
    let mut answers: Vec<HashMap<String, String>> = Vec::new();
    let mut answer_nb = 0;
    // Parse
    let input = File::open(path).expect("File error");
    let buffered = BufReader::new(input);

    let mut answer: HashMap<String, String> = HashMap::new();
    for line in buffered.lines() {
        let linestr = line.unwrap();
        if linestr.is_empty() {
            check_result(answer_nb, &mut answer);
            answers.push(answer.clone());
            answer = HashMap::new();
            answer_nb = 0;
        } else {
            for key in linestr.chars() {
                if answer_nb == 0 {
                    answer.insert(key.to_string(), "0".to_string());
                }

                if answer.contains_key(&key.to_string()) && answer_nb != 0 {
                    let value = answer.get(&key.to_string()).unwrap();
                    let mut ivalue: usize = value.parse().unwrap();
                    ivalue += 1;
                    answer.insert(key.to_string(), ivalue.to_string());
                }
            }
            answer_nb += 1;
        }
    }
    // Fix last line if input file does not contain blank line at the end
    check_result(answer_nb, &mut answer);
    answers.push(answer.clone());
    answers
}

fn main() {
    let path = "input";
    let mut nb_valid_answers = 0;
    let answers: Vec<HashMap<String, String>>;

    answers = parse(path);

    println!("Valid answers: {:?}", answers);
    for answer in answers {
        for (_key, value) in answer {
            if value == "ok" || value == "0" {
                nb_valid_answers += 1;
            }
        }
        println!("Valid answers: {}", nb_valid_answers);
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
