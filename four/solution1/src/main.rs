use std::collections::HashMap;
use std::fmt::Write;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_passport(n: usize, passports: &Vec<HashMap<String, String>>) -> String {
    let mut key_values: Vec<String> = Vec::new();
    for (key, value) in &passports[n] {
        let mut key_value = String::new();
        write!(&mut key_value, "{}:{}", key, value).unwrap();
        key_values.push(key_value.clone());
    }
    key_values.sort();
    key_values.join(" ")
}

fn parse(path: &str) -> Vec<HashMap<String, String>> {
    let mut passports: Vec<HashMap<String, String>> = Vec::new();
    // Parse
    let input = File::open(path).expect("File error");
    let buffered = BufReader::new(input);

    let mut passport: HashMap<String, String> = HashMap::new();
    for line in buffered.lines() {
        let linestr = line.unwrap();
        // println!("{}", line.unwrap());
        if linestr.is_empty() {
            passports.push(passport.clone());
            passport = HashMap::new();
        } else {
            let key_values: Vec<&str> = linestr.split(" ").collect();
            for key_value in key_values {
                let item: Vec<&str> = key_value.split(":").collect();
                passport.insert(item[0].to_string(), item[1].to_string());
            }
        }
    }
    passports
}

fn check_passport(n: usize, passports: &Vec<HashMap<String, String>>) -> bool {
    if passports[n].contains_key("byr")
        && passports[n].contains_key("hcl")
        && passports[n].contains_key("ecl")
        && passports[n].contains_key("iyr")
        && passports[n].contains_key("eyr")
        && passports[n].contains_key("pid")
        && passports[n].contains_key("hgt")
    {
        return true;
    }
    return false;
}

fn main() {
    let path = "input";
    let mut nb_valid_passports = 0;
    let passports: Vec<HashMap<String, String>>;

    passports = parse(path);

    for passport in 0..passports.len() {
        if check_passport(passport, &passports) {
            nb_valid_passports += 1;
        }
    }

    println!("Valid passwords: {}", nb_valid_passports);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_fake() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_0_entry() {
        let path = "input";
        let passports: Vec<HashMap<String, String>>;
        passports = parse(path);
        let output = get_passport(0, &passports);
        assert_eq!(
            output,
            "byr:1940 cid:139 ecl:blu eyr:2025 hcl:#ceb3a1 hgt:159cm iyr:2017 pid:561068005"
                .to_string()
        );
    }

    #[test]
    fn test_1_entry() {
        let path = "input";
        let passports: Vec<HashMap<String, String>>;
        passports = parse(path);
        let output = get_passport(1, &passports);
        assert_eq!(
            output,
            "byr:1986 ecl:hzl eyr:2025 iyr:2014 pid:960679613".to_string()
        );
    }

    #[test]
    fn test_check_passport() {
        let path = "input";
        let passports: Vec<HashMap<String, String>>;
        passports = parse(path);
        let output = check_passport(0, &passports);
        assert_eq!(output, true);
        let output = check_passport(1, &passports);
        assert_eq!(output, false);
        let output = check_passport(2, &passports);
        assert_eq!(output, true);
        let output = check_passport(3, &passports);
        assert_eq!(output, true);
        let output = check_passport(4, &passports);
        assert_eq!(output, false);
    }
}
