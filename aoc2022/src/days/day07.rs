#![allow(clippy::needless_return)]
use std::{
    fs,
    io::{self, BufRead},
};

enum Command {
    Ls,
    Cd(String),
}

fn read_file(path: &str) -> io::BufReader<fs::File> {
    let file = fs::File::open(path).unwrap();
    return io::BufReader::new(file);
}

#[allow(dead_code)]
pub fn solve() {
    let reader = read_file("./src/days/day07.txt");

    let mut dstack: Vec<usize> = Vec::new();
    let mut totals: Vec<usize> = Vec::new();
    // stolen https://www.reddit.com/r/adventofcode/comments/10je75e/2022_day_7rust_working_with_trees_seems_like_a/j5pbj4w/
    for line in reader.lines() {
        match line {
            Ok(l) => {
                if l.starts_with("$ cd") {
                    let path = l.split_whitespace().nth(2).unwrap();
                    if path == ".." {
                        let total = dstack.pop().unwrap();
                        totals.push(total);
                        if let Some(top) = dstack.last_mut() {
                            *top += total;
                        }
                    } else {
                        dstack.push(0);
                    }
                } else {
                    let files_size = l.split_ascii_whitespace().next().unwrap();
                    if let Ok(v) = files_size.parse::<usize>() {
                        *dstack.last_mut().unwrap() += v;
                    }
                }
            }
            Err(err) => {
                println!("{err}");
            }
        }
    }
    while let Some(v) = dstack.pop() {
        if let Some(top) = dstack.last_mut() {
            *top += v;
        }
        totals.push(v);
    }
    let sum = totals.iter().filter(|v| *v <= &100_000).sum::<usize>();
    let space = 30_000_000 - (70_000_000 - totals.last().unwrap());
    let p2 = totals.iter().filter(|v| *v >= &space).min();
    println!("{dstack:?}");
    println!("{totals:?}");
    println!("{sum:?}");
    println!("{p2:?}");
}
