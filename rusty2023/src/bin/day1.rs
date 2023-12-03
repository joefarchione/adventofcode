use std::cmp;
use std::fs;
use std::io::{prelude::*, BufReader};

 fn part_one() -> u32 {
    let file = fs::File::open("./data/day1.txt");
    let reader = BufReader::new(file.unwrap());

    return reader
        .lines()
        .map(|line| {
            let iter = line
                .expect("something bad happened")
                .chars()
                .filter(|c| c.is_numeric())
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<u32>>();
            return iter.first().unwrap() * 10u32 + iter.last().unwrap();
        })
        .sum::<u32>();
}

fn part_two() -> u32 {
    let numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let file = fs::File::open("./data/day1.txt");
    let reader = BufReader::new(file.unwrap());

    return reader
        .lines()
        .map(|line| line.expect("something bad happened"))
        .map(|line| {
            let iter = line
                .chars()
                .enumerate()
                .map(|(ii, c)| {
                    if c.is_digit(10) {
                        return c.to_digit(10).unwrap();
                    } else {
                        return numbers
                            .iter()
                            .enumerate()
                            .filter(|(_, number)| {
                                number.to_string()
                                    == line[ii..cmp::min(
                                        ii + number.chars().count(),
                                        line.chars().count(),
                                    )]
                            })
                            .map(|(jj, _)| return u32::try_from(jj).unwrap() + 1)
                            .next()
                            .unwrap_or(0u32);
                    }
                })
                .filter(|x| x != &0u32)
                .collect::<Vec<u32>>();
            return iter.first().unwrap() * 10u32 + iter.last().unwrap();
        })
        .sum();
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
