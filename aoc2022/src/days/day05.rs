#![allow(clippy::needless_return)]
use std::{
    fs,
    io::{self, BufRead},
};

fn read_file(path: &str) -> io::BufReader<fs::File> {
    let file = fs::File::open(path).unwrap();
    return io::BufReader::new(file);
}

#[allow(dead_code)]
pub fn solve() {
    let reader = read_file("./src/days/day05.txt");
    let mut mode = false;
    let mut stack: Vec<Vec<char>> = Vec::new();
    let mut first = true;
    for line in reader.lines() {
        match line {
            Ok(l) => {
                if l.is_empty() {
                    mode = true;
                    continue;
                }
                if mode {
                    let toks: Vec<_> = l.split_whitespace().collect();
                    let src = toks.get(3).unwrap().parse::<usize>().unwrap();
                    let dst = toks.get(5).unwrap().parse::<usize>().unwrap();
                    let mv = toks.get(1).unwrap().parse::<usize>().unwrap();
                    let mut mvd: Vec<_> = Vec::new();
                    println!("{l:?}");
                    println!("{stack:?}");
                    for _ in 0..mv {
                        mvd.push(stack.get_mut(src - 1).unwrap().remove(0));
                    }
                    stack.get_mut(dst - 1).unwrap().splice(0..0, mvd);
                    println!("{stack:?}");
                    /* part1
                    let toks: Vec<_> = l.split_whitespace().collect();
                    let src = toks.get(3).unwrap().parse::<usize>().unwrap();
                    let dst = toks.get(5).unwrap().parse::<usize>().unwrap();
                    for _ in 0..toks.get(1).unwrap().parse::<usize>().unwrap() {
                        let mv = stack.get_mut(src - 1).unwrap().remove(0);
                        stack.get_mut(dst - 1).unwrap().insert(0, mv);
                    }
                    */
                } else {
                    let tt: Vec<_> = l.chars().skip(1).step_by(4).collect();
                    if *tt.first().unwrap() == '1' {
                        continue;
                    }
                    if first {
                        stack.reserve(tt.len());
                        for _ in 0..tt.len() {
                            stack.push(Vec::new());
                        }
                        first = false;
                    }
                    tt.iter()
                        .enumerate()
                        .filter(|(_, v)| **v != ' ')
                        .for_each(|(i, v)| stack[i].push(*v));
                }
            }
            Err(err) => println!("{err:?}"),
        }
    }
    for s in &stack {
        print!("{:?}", s.first().unwrap());
    }
    println!();
    println!("{stack:?}");
}
