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

fn calculate_seatid(seat: &str, max_rows: usize, max_columns: usize) -> usize {
    let mut minrow: usize = 0;
    let mut maxrow: usize = max_rows;
    let mut mincol: usize = 0;
    let mut maxcol: usize = max_columns;
    let mut row: usize = 0;
    let mut col: usize = 0;

    for character in seat.chars() {
        match character {
            'B' => {
                minrow = minrow + (maxrow - minrow) / 2;
                row = maxrow - 1;
            }
            'F' => {
                maxrow = minrow + (maxrow - minrow) / 2;
                row = minrow;
            }
            'R' => {
                mincol = mincol + (maxcol - mincol) / 2;
                col = maxcol - 1;
            }
            'L' => {
                maxcol = mincol + (maxcol - mincol) / 2;
                col = mincol;
            }
            _ => panic!("Improper entry"),
        }
        println!("row {} {} {}", minrow, maxrow, row);
        println!("col {} {} {}", mincol, maxcol, col);
    }
    row * 8 + col
}

fn main() {
    let path = "input";
    const MAX_ROWS: usize = 128;
    const MAX_COLUMNS: usize = 8;
    let mut seatids: Vec<usize> = Vec::new();
    let seats = parse(path);

    for seat in seats {
        let seatid = calculate_seatid(&seat, MAX_ROWS, MAX_COLUMNS);
        seatids.push(seatid);
    }
    seatids.sort();
    println!("seatids {:?}", seatids);
    println!("Highest siteid: {}", seatids.last().unwrap());

    for seatid in seatids.windows(2) {
        let current = seatid[0];
        let next = seatid[1];

        if current + 1 != next {
            println!("My seat is: {}", current + 1);
        }
    }
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
    fn test_seat() {
        assert_eq!(calculate_seatid("FBFBBFFRLR", 128, 8), 357);
        assert_eq!(calculate_seatid("FBFBBFBRLR", 128, 8), 365);
        assert_eq!(calculate_seatid("BFFFBBFRRR", 128, 8), 567);
        assert_eq!(calculate_seatid("FFFBBBFRRR", 128, 8), 119);
        assert_eq!(calculate_seatid("BBFFBBFRLL", 128, 8), 820);
    }
}
