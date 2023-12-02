use std::{
    collections::HashSet,
    fs,
    io::{self, BufRead},
};

fn read_file(path: &str) -> io::BufReader<fs::File> {
    let file = fs::File::open(path).unwrap();
    return io::BufReader::new(file);
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn lead(&mut self, x: isize, y: isize) {
        self.x += x;
        self.y += y;
    }
    fn tail(&mut self, target: &Self) {
        let diffx = target.x - self.x;
        let diffy = target.y - self.y;
        if diffx.abs() <= 1 && diffy.abs() <= 1 {
            return;
        }
        self.x += diffx.clamp(-1, 1);
        self.y += diffy.clamp(-1, 1);
    }
    fn new(x: isize, y: isize) -> Self {
        return Point { x: x, y: y };
    }
}

#[allow(dead_code)]
pub fn solve() {
    let reader = read_file("./src/days/day09.txt");
    let mut head = Point::new(0, 0);
    //let mut tail = Point::new(0, 0);
    let mut tails: [Point; 9] = [Point::new(0, 0); 9];
    let mut pos: HashSet<Point> = HashSet::new();
    for line in reader.lines() {
        match line {
            Ok(l) => {
                let toks: Vec<_> = l.split_whitespace().collect();
                let dir = toks[0];
                let mvs = toks[1].parse::<usize>().unwrap();
                for _ in 0..mvs {
                    /*                     for i in (-160..110).rev() {
                        for j in -30..310 {
                            if head.x == j && head.y == i {
                                print!("H");
                            } else if tail.x == j && tail.y == i {
                                print!("T");
                            } else if i == 0 && j == 0 {
                                print!("s");
                            } else {
                                print!(".");
                            }
                        }
                        println!("");
                    }
                    println!(); */
                    pos.insert(tails[8].clone());
                    if dir == "R" {
                        head.lead(1, 0);
                    } else if dir == "L" {
                        head.lead(-1, 0);
                    } else if dir == "U" {
                        head.lead(0, 1);
                    } else if dir == "D" {
                        head.lead(0, -1);
                    }
                    tails[0].tail(&head);
                    for i in 1..tails.len() {
                        let t = tails[i - 1];
                        tails[i].tail(&t);
                    }
                }
            }
            Err(err) => {
                println!("{err}");
            }
        }
    }
    pos.insert(tails[8].clone());

    println!("{:?}", pos.len());
}
