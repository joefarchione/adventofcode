use std::fs;
use std::cmp;
use std::io::{prelude::*, BufReader};

pub fn part_one() -> u32 {
    let file = fs::File::open("./data/day1.txt");
    let reader = BufReader::new(file.unwrap());

    let sum = reader
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
    return sum;
}

pub fn part_two() -> u32 {
    let numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let file = fs::File::open("./data/day1.txt");
    let reader = BufReader::new(file.unwrap());

    let sum = reader
        .lines()
        .map(|line| {
            let iter = line
                .as_ref()
                .expect("something bad happened")
                .chars()
                .enumerate()
                .map(|(ii, c)| {
                    if c.is_digit(10) {
                        return c.to_digit(10).unwrap();
                    } else {
                        return numbers
                            .iter()
                            .enumerate()
                            .filter(|(jj, number)| {
                                number.to_string()
                                    == line.as_ref().expect("something")
                                        [ii..cmp::min(ii + number.chars().count(), line.as_ref().expect("something").chars().count())]
                            })
                            .map(|(jj, number)| return u32::try_from(jj).unwrap() + 1)
                            .next()
                            .unwrap_or(0u32);
                    }
                })
                .filter(|x| x != &0u32)
                .collect::<Vec<u32>>();
            return iter.first().unwrap() * 10u32 + iter.last().unwrap();
        })
        .sum();
    return sum;
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
