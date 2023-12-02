#![allow(clippy::needless_return)]
use std::{
    fs,
    io::{self, BufRead},
};

fn read_file(path: &str) -> io::BufReader<fs::File> {
    let file = fs::File::open(path).unwrap();
    return io::BufReader::new(file);
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn funkywunky(start: usize, end: usize) -> impl Iterator<Item = usize> {
    if start < end {
        return start..=end;
    }
    return end..=start;
}

fn max<T: Ord>(v1: T, v2: T) -> T {
    if v1 > v2 {
        return v1;
    }
    return v2;
}

fn min<T: Ord>(v1: T, v2: T) -> T {
    if v1 < v2 {
        return v1;
    }
    return v2;
}

const DIRECTIONS: [(isize, isize); 3] = [(0, 1), (-1, 1), (1, 1)];

fn move_sand(sand: &Point, map: &[Vec<char>]) -> Option<(isize, isize)> {
    for dir in DIRECTIONS {
        let res = map
            .get((sand.y as isize + dir.1) as usize)
            .and_then(|f| f.get((sand.x as isize + dir.0) as usize));
        if res.is_none() {
            return Some((sand.x as isize + dir.0, sand.y as isize + dir.1));
        }
        let res = res.unwrap();
        if *res == '#' || *res == 'o' {
            continue;
        }
        return Some((sand.x as isize + dir.0, sand.y as isize + dir.1));
    }
    return None;
}

fn move_sandp2(sand: &Point, map: &[Vec<char>]) -> Option<(isize, isize)> {
    for dir in DIRECTIONS {
        let res = map
            .get((sand.y as isize + dir.1) as usize)
            .and_then(|f| f.get((sand.x as isize + dir.0) as usize));
        if res.is_none() {
            return Some((sand.x as isize + dir.0, sand.y as isize + dir.1));
        }
        let res = res.unwrap();
        if *res == '#' || *res == 'o' {
            continue;
        }
        return Some((sand.x as isize + dir.0, sand.y as isize + dir.1));
    }
    return None;
}

#[allow(dead_code)]
pub fn solve() {
    let reader = read_file("./src/days/day14.txt");
    let mut maxx: usize = 0;
    let mut maxy: usize = 0;
    let mut minx: usize = 1000;
    let mut miny: usize = 1000;
    let mut intrs: Vec<Vec<Point>> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(l) => {
                let toks: Vec<_> = l.split(" -> ").collect();
                intrs.push(Vec::new());
                for t in &toks {
                    let dirs: Vec<_> = t.split(',').map(|s| s.parse::<usize>().unwrap()).collect();
                    maxx = max(dirs[0], maxx);
                    minx = min(dirs[0], minx);
                    maxy = max(dirs[1], maxy);
                    miny = min(dirs[1], miny);
                    intrs.last_mut().unwrap().push(Point {
                        x: dirs[0],
                        y: dirs[1],
                    });
                }
            }
            Err(err) => {
                println!("{err}");
            }
        }
    }
    let so = Point { x: 500, y: 0 };
    maxx = max(so.x, maxx);
    minx = min(so.x, minx);
    maxy = max(so.y, maxy);
    miny = min(so.y, miny);
    maxy = max(maxy + 2, maxy);
    miny = min(miny + 2, miny);

    println!("{minx} - {maxx}");
    println!("{miny} - {maxy}");

    let xmul = 10;
    let ofsx = ((maxx - minx + 1) * xmul) / 2;
    let ofsxb = maxx - minx;
    let ofsy = maxy - miny;
    let bigx = (ofsxb + 1) * xmul;
    /*
    let mut map: Vec<Vec<char>> = vec![vec!['.'; ofsx + 1]; ofsy + 1];
    */
    let mut map: Vec<Vec<char>> = vec![vec!['.'; bigx]; ofsy + 1];

    for ins in &intrs {
        ins.windows(2).for_each(|wind| {
            for i in funkywunky(wind[1].y - miny, wind[0].y - miny) {
                for j in funkywunky(wind[1].x - minx + ofsx, wind[0].x - minx + ofsx) {
                    map[i][j] = '#';
                }
            }
        });
    }

    map[ofsy].iter_mut().for_each(|x| *x = '#');
    for m in &map {
        println!("{m:?}");
    }
    let mut tot = 0;
    'free: loop {
        let mut sand = Point {
            x: so.x - minx + ofsx,
            y: so.y - miny,
        };
        while let Some(np) = move_sand(&sand, &map) {
            if np.0 == (so.x - minx + ofsx) as isize && np.1 == so.y as isize {
                tot += 1;
                println!("fig");
                break 'free;
            }
            if np.0 < 0 || np.0 >= bigx as isize {
                println!("fig");
                break 'free;
            }
            if np.1 < 0 || np.1 >= (maxy + 1 + 1) as isize {
                println!("fig");
                break 'free;
            }
            sand.x = np.0 as usize;
            sand.y = np.1 as usize;
        }
        map[sand.y][sand.x] = 'o';
        if sand.x == (so.x - minx + ofsx) && sand.y == 0 {
            tot += 1;
            break 'free;
        }
        tot += 1;
        /*         for m in &map {
            println!("{m:?}");
        } */
    }
    for m in &map {
        println!("{m:?}");
    }
    println!("{tot:?}");
}
