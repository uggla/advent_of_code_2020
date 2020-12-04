use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn calculate_trees_on_the_way(xpath: usize, ypath: usize, map: &Vec<Vec<char>>) -> usize {
    let mut trees = 0;
    let mut xpos = 0;
    for y in (0..map.len()).step_by(ypath) {
        println!("{:?}", map[y]);
        println!("{}", map[y][xpos]);
        if map[y][xpos] == '#' {
            trees += 1;
        }
        xpos += xpath;
    }
    trees
}

fn main() -> Result<(), Error> {
    let path = "input";
    let mut map: Vec<Vec<char>> = Vec::new();
    const NB_PATTERNS: usize = 73;
    let mut solution_trees = Vec::new();

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

    solution_trees.push(calculate_trees_on_the_way(1, 1, &map));
    solution_trees.push(calculate_trees_on_the_way(3, 1, &map));
    solution_trees.push(calculate_trees_on_the_way(5, 1, &map));
    solution_trees.push(calculate_trees_on_the_way(7, 1, &map));
    solution_trees.push(calculate_trees_on_the_way(1, 2, &map));

    println!("Trees: {:?}", solution_trees);
    let solution: usize = solution_trees.iter().fold(1, |acc, x| acc * x);
    println!("Nb trees: {}", solution);
    Ok(())
}
