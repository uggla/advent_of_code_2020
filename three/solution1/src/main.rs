use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let path = "input";
    let mut map: Vec<Vec<char>> = Vec::new();
    const NB_PATTERNS: usize = 32;
    let mut trees = 0;
    let mut xpos = 0;

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    // Parse
    for line in buffered.lines() {
        let mut horiz_char = Vec::new();
        let s = line.unwrap();
        // Add patterns to the right
        for _ in 0..NB_PATTERNS {
            for c in s.chars() {
                horiz_char.push(c);
            }
        }
        map.push(horiz_char.clone());
    }

    for y in map {
        println!("{:?}", y);
        println!("{}", y[xpos]);
        if y[xpos] == '#' {
            trees += 1;
        }
        xpos += 3;
    }

    println!("Nb trees: {}", trees);
    Ok(())
}
