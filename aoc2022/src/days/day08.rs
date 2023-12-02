use std::{
    collections::HashMap,
    fs,
    io::{self, BufRead, BufReader, Read},
};

fn read_file(path: &str) -> io::BufReader<fs::File> {
    let file = fs::File::open(path).unwrap();
    return io::BufReader::new(file);
}

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn rc_solve(trees: &Vec<Vec<char>>, tree: char, x: isize, y: isize, d: (isize, isize)) -> bool {
    if trees
        .get(x as usize)
        .and_then(|v| v.get(y as usize))
        .map_or(false, |t| tree <= *t)
    {
        return false;
    }
    if x < 0 || x >= trees.len() as isize {
        return true;
    }
    if y < 0 || y >= trees.len() as isize {
        return true;
    }
    return rc_solve(trees, tree, x + d.0, y + d.1, d);
}

fn rc_solve2(
    trees: &Vec<Vec<char>>,
    tree: char,
    x: isize,
    y: isize,
    d: (isize, isize),
    score: usize,
) -> usize {
    if trees
        .get(x as usize)
        .and_then(|v| v.get(y as usize))
        .map_or(false, |t| tree <= *t)
    {
        return score;
    }
    if x <= 0 || x >= trees.len() as isize - 1 {
        return score;
    }
    if y <= 0 || y >= trees.len() as isize - 1 {
        return score;
    }
    return rc_solve2(trees, tree, x + d.0, y + d.1, d, score + 1);
}

fn tree_solve2(trees: &Vec<Vec<char>>, x: isize, y: isize) -> usize {
    let tree = trees
        .get(x as usize)
        .and_then(|t| t.get(y as usize))
        .unwrap();
    let mut total = 1;
    for (dx, dy) in DIRECTIONS {
        let t = rc_solve2(trees, *tree, x + dx, y + dy, (dx, dy), 1);
        total *= t
    }
    return total;
}

fn tree_solve(trees: &Vec<Vec<char>>, x: isize, y: isize) -> bool {
    let tree = trees
        .get(x as usize)
        .and_then(|t| t.get(y as usize))
        .unwrap();
    for (dx, dy) in DIRECTIONS {
        if rc_solve(trees, *tree, x + dx, y + dy, (dx, dy)) {
            return true;
        }
    }
    return false;
}

#[allow(dead_code)]
pub fn solve() {
    let reader = read_file("./src/days/day08.txt");
    let mut trees: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(l) => {
                trees.push(l.chars().collect());
            }
            Err(err) => {
                println!("{err}");
            }
        }
    }
    let mut count: usize = 0;
    /*  part 1
    for x in 0..trees.len() {
        if x == 0 || x == trees.len() - 1 {
            count += trees.len();
            continue;
        }
        let tree = trees.get(x).unwrap();
        for y in 0..tree.len() {
            if y == 0 || y == tree.len() - 1 {
                count += 1;
                continue;
            }
            if tree_solve(&trees, x as isize, y as isize) {
                count += 1;
            }
        }
    }
    */
    let mut count: usize = 0;
    for x in 0..trees.len() {
        if x == 0 || x == trees.len() - 1 {
            continue;
        }
        let tree = trees.get(x).unwrap();
        for y in 0..tree.len() {
            if y == 0 || y == tree.len() - 1 {
                continue;
            }
            let t = tree_solve2(&trees, x as isize, y as isize);
            if t > count {
                count = t;
            }
        }
    }
    println!("{:?}", count)
}
