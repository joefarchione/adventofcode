use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::fs;
use std::io::{prelude::*, BufReader};

fn part_one() -> u32 {
    let file = fs::File::open("./data/day2.txt");
    let reader = BufReader::new(file.unwrap());

    let mut bag = HashMap::new();
    bag.insert("red", 12u32);
    bag.insert("green", 13u32);
    bag.insert("blue", 14u32);

    let re_sample = Regex::new(r"(\d+) ((?:red|blue|green))").unwrap();

    return reader
        .lines()
        .map(|line| line.expect("something bad happened"))
        .enumerate()
        .filter(|(_, game)| {
            re_sample
                .captures_iter(&game)
                .map(|c| c.extract())
                .all(|(_, [num_str, color])| num_str.parse::<u32>().unwrap() <= bag[color])
        })
        .map(|(ii, _)| u32::try_from(ii).unwrap() + 1)
        .sum();
}

fn part_two() -> u32 {
    let file = fs::File::open("./data/day2.txt");
    let reader = BufReader::new(file.unwrap());

    let re_sample = Regex::new(r"(\d+) ((?:red|blue|green))").unwrap();

    return reader
        .lines()
        .map(|line| line.expect("something bad happened"))
        .enumerate()
        .map(|(_, game)| {
            let mut bag = HashMap::new();
            bag.insert("red", 0u32);
            bag.insert("blue", 0u32);
            bag.insert("green", 0u32);
            re_sample
                .captures_iter(&game)
                .map(|c| c.extract())
                .for_each(|(_, [num_str, color])| {
                    bag.insert(
                        color,
                        cmp::max(
                            **bag.get(color).as_ref().unwrap(),
                            num_str.parse::<u32>().unwrap(),
                        ),
                    );
                });

            return bag.values().cloned().reduce(|x, y| x * y).unwrap();
        })
        .sum();
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
