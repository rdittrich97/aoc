use std::{
    collections::{VecDeque},
    fs,
    io::{self, BufRead},
};

fn read_file(path: &str) -> io::BufReader<fs::File> {
    let file = fs::File::open(path).unwrap();
    return io::BufReader::new(file);
}

#[derive(Debug, Clone)]
enum Op {
    Add(isize),
    Mul(isize),
    Pow,
}

impl Op {
    fn apply(&self, new: isize) -> isize {
        match self {
            Self::Add(val) => {
                return new + val;
            }
            Self::Mul(val) => {
                return new * val;
            }
            Self::Pow => {
                return new * new;
            }
        }
    }
}

#[derive(Debug)]
struct Test {
    pos: usize,
    neg: usize,
    disc: isize,
}

impl Test {
    fn test(&self, val: isize, print: bool) -> usize {
        if val % self.disc == 0 {
            if print {
                print!("true");
            }
            return self.pos;
        }

        if print {
            print!("false");
        }
        return self.neg;
    }
    fn new(pos: usize, neg: usize, disc: isize) -> Self {
        return Self {
            pos: pos,
            neg: neg,
            disc: disc,
        };
    }
}

#[derive(Debug)]
struct Monke {
    items: VecDeque<isize>,
    operation: Op,
    test: Test,
    trouble: usize,
}

impl Monke {
    fn new(items: VecDeque<isize>, operation: Op, test: Test) -> Self {
        return Monke {
            items: items,
            operation: operation,
            test: test,
            trouble: 0,
        };
    }

    fn throw(&mut self, lcm: isize) -> Option<(isize, usize)> {
        /*         self.items.front().map(|v| {
            print!("{v} -> {} -> ", self.operation.apply(*v) % 9699690,);
            self.test.test(self.operation.apply(*v) % 9699690, true);
            println!("");
        }); */
        return self.items.pop_front().map(|old| {
            self.trouble += 1;
            let nv = self.operation.apply(old) % lcm;
            (nv, self.test.test(nv, false))
        });
    }

    fn receive(&mut self, val: isize) {
        self.items.push_back(val);
    }
}

#[allow(dead_code)]
pub fn solve() {
    let reader = read_file("./src/days/day11.txt");
    let mut monkes: Vec<Monke> = Vec::new();
    let mut midx = 0;
    let mut mitems: VecDeque<isize> = VecDeque::new();
    let mut operation: Op = Op::Mul(0);
    let mut disc: isize = 0;
    let mut pos: usize = 0;
    let mut neg: usize = 0;
    for line in reader.lines() {
        match line {
            Ok(l) => {
                if l.is_empty() {
                    monkes.insert(
                        midx,
                        Monke::new(
                            mitems.to_owned(),
                            operation.to_owned(),
                            Test::new(pos, neg, disc),
                        ),
                    );
                    continue;
                }
                let toks: Vec<_> = l.split_whitespace().collect();
                if l.starts_with("Monkey") {
                    midx = toks[1].chars().nth(0).unwrap().to_digit(10).unwrap() as usize;
                } else if let Some(items) = l.strip_prefix("  Starting items: ") {
                    mitems = items
                        .split(',')
                        .map(|x| x.trim().parse::<isize>().unwrap())
                        .collect();
                } else if let Some(op) = l.strip_prefix("  Operation: new = old ") {
                    let vals: Vec<_> = op.split_whitespace().collect();
                    if vals[1] == "old" {
                        operation = Op::Pow;
                    } else if vals[0] == "*" {
                        operation = Op::Mul(vals[1].parse::<isize>().unwrap());
                    } else if vals[0] == "+" {
                        operation = Op::Add(vals[1].parse::<isize>().unwrap());
                    } else {
                        panic!("stinker");
                    }
                } else if let Some(t) = l.strip_prefix("  Test: divisible by ") {
                    disc = t.trim().parse::<isize>().unwrap();
                } else if let Some(p) = l.strip_prefix("    If true: throw to monkey ") {
                    pos = p.trim().parse::<usize>().unwrap();
                } else if let Some(p) = l.strip_prefix("    If false: throw to monkey ") {
                    neg = p.trim().parse::<usize>().unwrap();
                }
            }
            Err(err) => {
                println!("{err}");
            }
        }
    }
    monkes.insert(
        midx,
        Monke::new(
            mitems.to_owned(),
            operation.to_owned(),
            Test::new(pos, neg, disc),
        ),
    );
    println!("{:?}", monkes.len());
    let lcm = monkes.iter().fold(1, |acc, m| acc * m.test.disc);
    for _ in 0..10000 {
        for m in 0..monkes.len() {
            while let Some((val, idx)) = monkes[m].throw(lcm) {
                monkes[idx].receive(val);
            }
        }
    }
    for m in &monkes {
        println!("{:?}", m);
    }
    println!("---------");
    monkes.sort_by_key(|monke| monke.trouble);
    let mut tot = 1;
    for i in monkes.len() - 2..monkes.len() {
        println!("{:?}", monkes[i]);
        tot *= monkes[i].trouble;
    }
    println!("{tot}");
}
