use std::{
    collections::VecDeque,
    fs,
    io::{self, BufRead},
};

fn read_file(path: &str) -> io::BufReader<fs::File> {
    let file = fs::File::open(path).unwrap();
    return io::BufReader::new(file);
}

#[allow(dead_code)]
pub fn solve() {
    let reader = read_file("./src/days/day10-test.txt");
    let mut x = 1;
    let mut adds: VecDeque<isize> = VecDeque::new();
    let mut ll: isize = 0;
    let mut lm: isize;
    for line in reader.lines() {
        match line {
            Ok(l) => {
                let toks: Vec<_> = l.split_whitespace().collect();
                let op = toks[0];
                lm = ll % 40;
                if lm == 0 {
                    println!();
                }
                if x - 1 <= lm && lm <= x + 1 {
                    print!("#");
                } else {
                    print!(".");
                }
                if let Some(v) = adds.pop_front() {
                    ll += 1;
                    x += v;
                    lm = ll % 40;
                    if lm == 0 {
                        println!();
                    }
                    if x - 1 <= lm && lm <= x + 1 {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }

                if op == "noop" {
                } else if op == "addx" {
                    adds.push_back(toks[1].parse::<isize>().unwrap());
                }
                ll += 1;
                /* f = false;
                ss = (ll - 20) % 40;
                if ss == 0 && ll <= 220 {
                    //println!("== {ll} ==");
                    //  println!("{x}");
                    tot += ll * x;
                    f = true;
                }
                adds.pop_front().map(|v| {
                    x += v;
                    ll += 1;
                    ss = (ll - 20) % 40;
                });
                if ss == 0 && ll <= 220 && !f {
                    //println!("== {ll} ==");
                    //println!("{x}");
                    tot += ll * x;
                }
                if op == "noop" {
                } else if op == "addx" {
                    adds.push_back(toks[1].parse::<isize>().unwrap());
                }
                ll += 1;
                f = false; */
            }
            Err(err) => {
                println!("{err}");
            }
        }
    }
    println!();
}
