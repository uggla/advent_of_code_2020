use regex::Regex;
use std::collections::HashMap;
use std::fmt::Write;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
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

fn check_year_range(value: &String, min: usize, max: usize) -> bool {
    let re = Regex::new(r"^\d{4}$").unwrap();
    if re.is_match(value) {
        let year: usize = value.parse().unwrap();
        println!("year {}", year);
        if year <= max && year >= min {
            return true;
        }
    }
    false
}

fn check_byr(value: &String) -> bool {
    check_year_range(value, 1920, 2002)
}

fn check_iyr(value: &String) -> bool {
    check_year_range(value, 2010, 2020)
}

fn check_eyr(value: &String) -> bool {
    check_year_range(value, 2020, 2030)
}

fn check_hgt(value: &String) -> bool {
    let re = Regex::new(r"^(\d+)(cm)$").unwrap();
    if re.is_match(value) {
        let max: usize = 193;
        let min: usize = 150;
        let caps = re.captures(value).unwrap();
        println!("hgt: {}{}", &caps[1], &caps[2]);
        let size: usize = caps[1].parse().unwrap();
        if size <= max && size >= min {
            return true;
        }
    }
    let re = Regex::new(r"^(\d+)(in)$").unwrap();
    if re.is_match(value) {
        let max: usize = 76;
        let min: usize = 59;
        let caps = re.captures(value).unwrap();
        println!("hgt: {}{}", &caps[1], &caps[2]);
        let size: usize = caps[1].parse().unwrap();
        if size <= max && size >= min {
            return true;
        }
    }
    false
}

fn check_hcl(value: &String) -> bool {
    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    if re.is_match(value) {
        println!("hcl: {}", value);
        return true;
    }
    false
}

fn check_ecl(value: &String) -> bool {
    let re = Regex::new(r"^amb$|^blu$|^brn$|^gry$|^grn$|^hzl$|^oth$").unwrap();
    if re.is_match(value) {
        println!("ecl: {}", value);
        return true;
    }
    false
}

fn check_pid(value: &String) -> bool {
    let re = Regex::new(r"^\d{9}$").unwrap();
    if re.is_match(value) {
        println!("pid: {}", value);
        return true;
    }
    false
}

fn check_passport(n: usize, passports: &Vec<HashMap<String, String>>) -> bool {
    match passports[n].get("byr") {
        Some(value) => {
            if !check_byr(value) {
                return false;
            }
        }
        None => return false,
    };

    match passports[n].get("iyr") {
        Some(value) => {
            if !check_iyr(value) {
                return false;
            }
        }
        None => return false,
    };

    match passports[n].get("eyr") {
        Some(value) => {
            if !check_eyr(value) {
                return false;
            }
        }
        None => return false,
    };

    match passports[n].get("hgt") {
        Some(value) => {
            if !check_hgt(value) {
                return false;
            }
        }
        None => return false,
    };

    match passports[n].get("hcl") {
        Some(value) => {
            if !check_hcl(value) {
                return false;
            }
        }
        None => return false,
    };

    match passports[n].get("ecl") {
        Some(value) => {
            if !check_ecl(value) {
                return false;
            }
        }
        None => return false,
    };

    match passports[n].get("pid") {
        Some(value) => {
            if !check_pid(value) {
                return false;
            }
        }
        None => return false,
    };

    true
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

    // #[test]
    // fn test_check_passport() {
    //     let path = "input";
    //     let passports: Vec<HashMap<String, String>>;
    //     passports = parse(path);
    //     let output = check_passport(0, &passports);
    //     assert_eq!(output, true);
    //     let output = check_passport(1, &passports);
    //     assert_eq!(output, true);
    //     let output = check_passport(2, &passports);
    //     assert_eq!(output, true);
    //     let output = check_passport(3, &passports);
    //     assert_eq!(output, true);
    //     let output = check_passport(4, &passports);
    //     assert_eq!(output, false);
    // }

    #[test]
    fn test_check_passport_byr() {
        let path = "input_byr_ok";
        let passports: Vec<HashMap<String, String>>;
        passports = parse(path);
        let output = check_passport(0, &passports);
        assert_eq!(output, true);
    }

    #[test]
    fn test_check_passport_byr_ko_1() {
        let path = "input_byr_ko_1";
        let passports: Vec<HashMap<String, String>>;
        passports = parse(path);
        let output = check_passport(0, &passports);
        assert_eq!(output, false);
    }

    #[test]
    fn test_check_passport_byr_ko_2() {
        let path = "input_byr_ko_2";
        let passports: Vec<HashMap<String, String>>;
        passports = parse(path);
        let output = check_passport(0, &passports);
        assert_eq!(output, false);
    }

    #[test]
    fn test_check_passport_byr_ko_3() {
        let path = "input_byr_ko_3";
        let passports: Vec<HashMap<String, String>>;
        passports = parse(path);
        let output = check_passport(0, &passports);
        assert_eq!(output, false);
    }

    #[test]
    fn test_check_passport_iyr() {
        let path = "input_iyr_ok";
        let passports: Vec<HashMap<String, String>>;
        passports = parse(path);
        let output = check_passport(0, &passports);
        assert_eq!(output, true);
    }

    #[test]
    fn test_check_passport_iyr_ko_1() {
        let path = "input_iyr_ko_1";
        let passports: Vec<HashMap<String, String>>;
        passports = parse(path);
        let output = check_passport(0, &passports);
        assert_eq!(output, false);
    }

    #[test]
    fn test_check_passport_iyr_ko_2() {
        let path = "input_iyr_ko_2";
        let passports: Vec<HashMap<String, String>>;
        passports = parse(path);
        let output = check_passport(0, &passports);
        assert_eq!(output, false);
    }

    #[test]
    fn test_check_passport_iyr_ko_3() {
        let path = "input_iyr_ko_3";
        let passports: Vec<HashMap<String, String>>;
        passports = parse(path);
        let output = check_passport(0, &passports);
        assert_eq!(output, false);
    }

    #[test]
    fn test_check_passport_eyr() {
        let path = "input_eyr_ok";
        let passports: Vec<HashMap<String, String>>;
        passports = parse(path);
        let output = check_passport(0, &passports);
        assert_eq!(output, true);
    }

    #[test]
    fn test_check_passport_eyr_ko_1() {
        let path = "input_eyr_ko_1";
        let passports: Vec<HashMap<String, String>>;
        passports = parse(path);
        let output = check_passport(0, &passports);
        assert_eq!(output, false);
    }

    #[test]
    fn test_check_passport_eyr_ko_2() {
        let path = "input_eyr_ko_2";
        let passports: Vec<HashMap<String, String>>;
        passports = parse(path);
        let output = check_passport(0, &passports);
        assert_eq!(output, false);
    }

    #[test]
    fn test_check_passport_eyr_ko_3() {
        let path = "input_eyr_ko_3";
        let passports: Vec<HashMap<String, String>>;
        passports = parse(path);
        let output = check_passport(0, &passports);
        assert_eq!(output, false);
    }

    #[test]
    fn test_check_passport_hgt() {
        let path = "input_hgt_ok";
        let passports: Vec<HashMap<String, String>>;
        passports = parse(path);
        let output = check_passport(0, &passports);
        assert_eq!(output, true);
    }

    #[test]
    fn test_check_hgt() {
        assert_eq!(check_hgt(&"150cm".to_string()), true);
        assert_eq!(check_hgt(&"193cm".to_string()), true);
        assert_eq!(check_hgt(&"149cm".to_string()), false);
        assert_eq!(check_hgt(&"194cm".to_string()), false);
        assert_eq!(check_hgt(&"59in".to_string()), true);
        assert_eq!(check_hgt(&"76in".to_string()), true);
        assert_eq!(check_hgt(&"58in".to_string()), false);
        assert_eq!(check_hgt(&"77in".to_string()), false);
    }

    #[test]
    fn test_check_passport_hcl() {
        let path = "input_hcl_ok";
        let passports: Vec<HashMap<String, String>>;
        passports = parse(path);
        let output = check_passport(0, &passports);
        assert_eq!(output, true);
    }

    #[test]
    fn test_check_hcl() {
        assert_eq!(check_hcl(&"#ceb3a1".to_string()), true);
        assert_eq!(check_hcl(&"ceb3a1".to_string()), false);
        assert_eq!(check_hcl(&"#cceb3a1".to_string()), false);
        assert_eq!(check_hcl(&"#Ceb3a1".to_string()), false);
        assert_eq!(check_hcl(&"#geb3a1".to_string()), false);
        assert_eq!(check_hcl(&"#Geb3a1".to_string()), false);
    }

    #[test]
    fn test_check_passport_ecl() {
        let path = "input_ecl_ok";
        let passports: Vec<HashMap<String, String>>;
        passports = parse(path);
        let output = check_passport(0, &passports);
        assert_eq!(output, true);
    }

    #[test]
    fn test_check_ecl() {
        assert_eq!(check_ecl(&"amb".to_string()), true);
        assert_eq!(check_ecl(&"blu".to_string()), true);
        assert_eq!(check_ecl(&"brn".to_string()), true);
        assert_eq!(check_ecl(&"gry".to_string()), true);
        assert_eq!(check_ecl(&"grn".to_string()), true);
        assert_eq!(check_ecl(&"hzl".to_string()), true);
        assert_eq!(check_ecl(&"oth".to_string()), true);
        assert_eq!(check_ecl(&"th".to_string()), false);
        assert_eq!(check_ecl(&"xxth".to_string()), false);
        assert_eq!(check_ecl(&"xxx".to_string()), false);
    }

    #[test]
    fn test_check_passport_pid() {
        let path = "input_pid_ok";
        let passports: Vec<HashMap<String, String>>;
        passports = parse(path);
        let output = check_passport(0, &passports);
        assert_eq!(output, true);
    }

    #[test]
    fn test_check_pid() {
        assert_eq!(check_pid(&"561068005".to_string()), true);
        assert_eq!(check_pid(&"000000000".to_string()), true);
        assert_eq!(check_pid(&"100000000".to_string()), true);
        assert_eq!(check_pid(&"00000000".to_string()), false);
        assert_eq!(check_pid(&"1100000000".to_string()), false);
        assert_eq!(check_pid(&"a00000000".to_string()), false);
        assert_eq!(check_pid(&"A00000000".to_string()), false);
    }
}
