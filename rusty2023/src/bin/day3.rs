use itertools::{Itertools, TupleWindows};
use regex::Regex;
use std::fs;
use std::io::{prelude::*, BufReader};
use std::iter::once;

#[derive(Clone)]
struct NumberMatch {
    value: i32,
    position: i32,
    size: i32,
}

#[derive(Clone)]
struct Positions {
    numbers: Vec<NumberMatch>,
    symbols: Vec<i32>,
}

fn part_one() -> i32 {
    let file = fs::File::open("./data/day3.txt");
    let reader = BufReader::new(file.unwrap());

    let re_symbol = Regex::new(r"([^\.\d\n])").unwrap();
    let re_number = Regex::new(r"(\d+)").unwrap();

    let positions = reader
        .lines()
        .map(|line| line.expect("something bad happened"))
        .map(|line| {
            let numbers = re_number
                .captures_iter(&line)
                .map(|c| {
                    return NumberMatch {
                        value: c.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                        position: i32::try_from(c.get(1).unwrap().start()).unwrap(),
                        size: i32::try_from(c[1].chars().count()).unwrap(),
                    };
                })
                .collect::<Vec<NumberMatch>>();

            let symbols = re_symbol
                .captures_iter(&line)
                .map(|c| i32::try_from(c.get(1).unwrap().start()).unwrap())
                .collect::<Vec<i32>>();
            return Positions { numbers, symbols };
        });

    let empty_line = Positions {
        numbers: vec![NumberMatch {
            value: 0i32,
            position: 10000i32,
            size: 0i32,
        }],
        symbols: vec![10000i32],
    };
    let padded = once(empty_line.clone())
        .chain(positions)
        .chain(once(empty_line.clone()));

    let i: TupleWindows<_, (Positions, Positions, Positions)> = padded.into_iter().tuple_windows();

    let val = i
        .map(|(p1, p2, p3)| {
            return p2
                .numbers
                .iter()
                .filter(|n| {
                    p1.symbols
                        .iter()
                        .chain(p2.symbols.iter())
                        .chain(p3.symbols.iter())
                        .any(|s: &i32| (n.position - 1 <= *s) && (*s <= (n.position + n.size)))
                })
                .map(|n| n.value)
                .sum::<i32>();
        })
        .sum::<i32>();
    return val;
}

fn part_two() -> i32 {
    let file = fs::File::open("./data/day3.txt");
    let reader = BufReader::new(file.unwrap());

    let re_symbol = Regex::new(r"(\*)").unwrap();
    let re_number = Regex::new(r"(\d+)").unwrap();

    let positions = reader
        .lines()
        .map(|line| line.expect("something bad happened"))
        .map(|line| {
            let numbers = re_number
                .captures_iter(&line)
                .map(|c| {
                    return NumberMatch {
                        value: c.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                        position: i32::try_from(c.get(1).unwrap().start()).unwrap(),
                        size: i32::try_from(c[1].chars().count()).unwrap(),
                    };
                })
                .collect::<Vec<NumberMatch>>();

            let symbols = re_symbol
                .captures_iter(&line)
                .map(|c| i32::try_from(c.get(1).unwrap().start()).unwrap())
                .collect::<Vec<i32>>();
            return Positions { numbers, symbols };
        });

    let empty_line = Positions {
        numbers: vec![NumberMatch {
            value: 0i32,
            position: 10000i32,
            size: 0i32,
        }],
        symbols: vec![10000i32],
    };
    let padded = once(empty_line.clone())
        .chain(positions)
        .chain(once(empty_line.clone()));

    let i: TupleWindows<_, (Positions, Positions, Positions)> = padded.into_iter().tuple_windows();

    let val = i
        .map(|(p1, p2, p3)| {
            return p2
                .symbols
                .iter()
                .map(|s| {
                    let matches = p1
                        .numbers
                        .iter()
                        .chain(p2.numbers.iter())
                        .chain(p3.numbers.iter())
                        .filter(|n| {
                            (s - 1..s + 2)
                                .any(|s1| (n.position <= s1) && (n.position + n.size - 1 >= s1))
                        });
                    if matches.clone().count() == 2usize {
                        return matches.map(|m| m.value).reduce(|x, y| x * y).unwrap();
                    } else {
                        return 0i32;
                    }
                })
                .sum::<i32>();
        })
        .sum::<i32>();
    return val;
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
