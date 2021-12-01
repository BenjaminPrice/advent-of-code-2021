use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut previous = 0;
    let mut count = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap().parse().unwrap();
        if index != 0 && line > previous {
            count = count + 1;
        }
        previous = line;
    }

    println!("{}", count);
    // 1477
}
