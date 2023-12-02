use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    io::{self, BufRead, BufReader, Read},
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
    fn new(x: isize, y: isize) -> Self {
        return Point { x: x, y: y };
    }
}

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn rc_path(
    map: &Vec<Vec<char>>,
    prev: char,
    x: isize,
    y: isize,
    len: usize,
    seen: &mut Vec<Vec<bool>>,
    path: &mut VecDeque<Point>,
) -> Option<usize> {
    let val = map.get(x as usize).and_then(|t| t.get(y as usize));
    if val.is_none() {
        return None;
    }
    if seen[x as usize][y as usize] == true {
        return None;
    }
    let mut ch = val.unwrap().clone();
    if ch == 'E' {
        return Some(len + 1);
    }
    let prevv;
    if ch == 'S' {
        ch = 'a';
    }
    let chv = ch as usize;
    if prev == 'S' {
        prevv = 'a' as usize;
    } else {
        prevv = prev as usize;
    }
    if chv > prevv + 1 || chv < prevv - 1 {
        return None;
    }
    path.push_back(Point::new(x, y));
    seen[x as usize][y as usize] = true;
    for (dx, dy) in DIRECTIONS {
        if let Some(v) = rc_path(map, ch, x + dx, y + dy, len + 1, seen, path) {
            return Some(v);
        }
    }
    path.pop_back();
    return None;
}

fn find_path(map: &Vec<Vec<char>>, x: isize, y: isize) -> Option<usize> {
    let prev = map.get(x as usize).and_then(|t| t.get(y as usize)).unwrap();
    let mut seen = vec![vec![false; map.get(0).unwrap().len()]; map.len()];
    let mut path = VecDeque::new();
    let t = rc_path(map, *prev, x, y, 1, &mut seen, &mut path);
    let mut s = std::iter::repeat('.').take(45).collect::<String>();
    let mut i = 0;
    for p in &path {
        let idx = (p.y * 5 + p.x) as usize;
        s.replace_range(idx..idx + 1, format!("{}", (i + 65 as u8) as char).as_str());
        i += 1;
    }

    return t;
}

fn can_move(map: &Vec<Vec<char>>, x: isize, y: isize, ox: isize, oy: isize) -> bool {
    let val = map.get(x as usize).and_then(|t| t.get(y as usize));
    if val.is_none() {
        return false;
    }
    let pre = map
        .get(ox as usize)
        .and_then(|t| t.get(oy as usize))
        .clone();
    if pre.is_none() {
        return false;
    }
    let mut ch = val.unwrap().clone();
    let prev = pre.unwrap().clone();
    let prevv;
    if ch == 'S' {
        ch = 'a';
    } else if ch == 'E' {
        ch = 'z';
    }
    let chv = ch as usize;
    if prev == 'S' {
        prevv = 'a' as usize;
    } else if prev == 'E' {
        prevv = 'z' as usize;
    } else {
        prevv = prev as usize;
    }
    if chv > prevv + 1 {
        return false;
    }
    return true;
}

fn solver(map: &Vec<Vec<char>>, start: Point, end: Point) -> Option<isize> {
    let mut que: VecDeque<Point> = VecDeque::new();
    que.push_front(start);
    let n = map.len();
    let m = map.get(0).and_then(|x| Some(x.len())).unwrap();
    let mut distances: Vec<Vec<isize>> = vec![vec![-1; m]; n];
    distances[start.x as usize][start.y as usize] = 0;
    while let Some(pp) = que.pop_front() {
        for (dx, dy) in DIRECTIONS {
            let nx = pp.x + dx;
            let ny = pp.y + dy;
            /*             println!(
                "{}-{}->{nx}-{ny} = {}",
                pp.x,
                pp.y,
                can_move(map, nx, ny, pp.x, pp.y)
            ); */
            if nx >= 0
                && ny >= 0
                && nx < n as isize
                && ny < m as isize
                && (distances[nx as usize][ny as usize] == -1
                    || distances[nx as usize][ny as usize]
                        > (distances[pp.x as usize][pp.y as usize] + 1))
                && can_move(map, nx, ny, pp.x, pp.y)
            {
                distances[nx as usize][ny as usize] = distances[pp.x as usize][pp.y as usize] + 1;
                que.push_front(Point::new(nx, ny))
            }
        }
    }

    println!("{:?}", distances[end.x as usize][end.y as usize]);
    return Some(distances[end.x as usize][end.y as usize]);
}

pub fn solve() {
    let reader = read_file("./src/days/day12.txt");
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut x = 0;
    let mut start: Point = Point::new(0, 0);
    let mut end: Point = Point::new(0, 0);
    let mut aas: Vec<Point> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(l) => {
                if let Some((y, _)) = l.char_indices().find(|(_, c)| *c == 'S') {
                    start = Point::new(x, y as isize);
                }
                if let Some((y, _)) = l.char_indices().find(|(_, c)| *c == 'E') {
                    end = Point::new(x, y as isize);
                }
                l.char_indices()
                    .filter(|(_, c)| *c == 'a')
                    .for_each(|(y, _)| aas.push(Point::new(x, y as isize)));
                map.push(l.chars().collect());
                x += 1;
            }
            Err(err) => {
                println!("{err}");
            }
        }
    }
    println!("{:?}", start);
    println!("{:?}", map);
    println!("{:?}", find_path(&map, start.x, start.y));
    solver(&map, start, end);
    let mut min = 1000;
    for a in &aas {
        if let Some(v) = solver(&map, *a, end) {
            if v != -1 && v < min {
                min = v;
            }
        }
    }
    println!("{:?}", min);
}
