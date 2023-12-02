use itertools::Itertools;
use std::{
    fs,
    io::{self, BufRead},
    time::Instant,
};

fn read_file(path: &str) -> io::BufReader<fs::File> {
    let file = fs::File::open(path).unwrap();
    return io::BufReader::new(file);
}

//const MAX_SIZE: usize = 8088;
const MAX_SIZE: usize = 10_000;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

const HLINE: [Point; 4] = [
    Point { x: 2, y: 0 },
    Point { x: 3, y: 0 },
    Point { x: 4, y: 0 },
    Point { x: 5, y: 0 },
];

const PLUS: [Point; 5] = [
    Point { x: 3, y: 0 },
    Point { x: 2, y: 1 },
    Point { x: 3, y: 1 },
    Point { x: 4, y: 1 },
    Point { x: 3, y: 2 },
];

const EL: [Point; 5] = [
    Point { x: 2, y: 0 },
    Point { x: 3, y: 0 },
    Point { x: 4, y: 0 },
    Point { x: 4, y: 1 },
    Point { x: 4, y: 2 },
];

const VLINE: [Point; 4] = [
    Point { x: 2, y: 0 },
    Point { x: 2, y: 1 },
    Point { x: 2, y: 2 },
    Point { x: 2, y: 3 },
];

const SQUARE: [Point; 4] = [
    Point { x: 2, y: 0 },
    Point { x: 3, y: 0 },
    Point { x: 2, y: 1 },
    Point { x: 3, y: 1 },
];

const WIDTH: usize = 7;

enum Shapes {
    Hline,
    Plus,
    El,
    Vline,
    Square,
}

fn can_move(
    shape: &[Point],
    dirx: isize,
    diry: isize,
    offsetx: isize,
    offsety: isize,
    tetris: &[[char; 7]; MAX_SIZE],
) -> bool {
    for p in shape {
        let px: isize = p.x as isize + dirx + offsetx;
        let py: isize = p.y as isize + diry + offsety;
        if px >= WIDTH as isize || px < 0 {
            return false;
        }
        if py >= MAX_SIZE as isize || py < 0 {
            return false;
        }
        if tetris[py as usize][px as usize] == '#' {
            return false;
        }
    }
    return true;
}

fn compact(tetris: &mut [[char; WIDTH]; MAX_SIZE], mins: usize, max: usize) {
    let m = max + 1;
    for i in 0..m {
        for j in 0..WIDTH {
            if i < (m - mins) {
                tetris[i][j] = tetris[i + mins][j];
            } else {
                tetris[i][j] = '.';
            }
        }
    }
}

const SONIC_MODE: [[char; WIDTH]; MAX_SIZE] = [['.'; WIDTH]; MAX_SIZE];

fn fast_compact(tetris: &mut [[char; WIDTH]; MAX_SIZE], mins: usize, max: usize) {
    let m = max + 1;
    let temp = tetris[mins..m].to_vec();
    tetris[m - mins..m].clone_from_slice(&SONIC_MODE[..mins]);
    tetris[..m - mins].clone_from_slice(&temp);
}

fn can_move_1d(
    shape: &[Point],
    dirx: isize,
    diry: isize,
    offsetx: isize,
    offsety: isize,
    tetris: &[char; WIDTH * MAX_SIZE],
) -> bool {
    for p in shape {
        let px: isize = p.x as isize + dirx + offsetx;
        let py: isize = p.y as isize + diry + offsety;
        if px >= WIDTH as isize || px < 0 {
            return false;
        }
        if py >= MAX_SIZE as isize || py < 0 {
            return false;
        }
        if tetris[py as usize * WIDTH + px as usize] == '#' {
            return false;
        }
    }
    return true;
}

fn compact_1d(tetris: &mut [char; WIDTH * MAX_SIZE], mins: usize, max: usize) {
    let m = max + 1;
    for i in 0..m {
        for j in 0..WIDTH {
            if i < (m - mins) {
                tetris[i * WIDTH + j] = tetris[(i + mins) * WIDTH + j];
            } else {
                tetris[i * WIDTH + j] = '.';
            }
        }
    }
}

const SONIC_MODE_1D: [char; WIDTH * MAX_SIZE] = ['.'; WIDTH * MAX_SIZE];

fn fast_compact_1d(tetris: &mut [char; WIDTH * MAX_SIZE], mins: usize, max: usize) {
    let m = max + 1;
    let temp = tetris[mins * WIDTH..m * WIDTH].to_vec();
    tetris[(m - mins) * WIDTH..m * WIDTH].clone_from_slice(&SONIC_MODE_1D[..mins * WIDTH]);
    tetris[..(m - mins) * WIDTH].clone_from_slice(&temp);
}

#[allow(dead_code)]
pub fn solve() {
    let reader = read_file("./src/days/day17.txt");
    let mut iter: usize = 0;
    let mut board: [char; WIDTH * MAX_SIZE] = ['.'; WIDTH * MAX_SIZE];
    //let mut board: [[char; WIDTH]; MAX_SIZE] = [['.'; WIDTH]; MAX_SIZE];

    let mut mins: [usize; WIDTH] = [0; WIDTH];
    let mut curr = Shapes::Hline;
    let mut max: isize = -1;
    let mut tmax: usize = 0;
    let mves = reader.lines().exactly_one().unwrap().unwrap();
    let moves: Vec<_> = mves.chars().collect();
    println!("{moves:?}");
    let mut s: &[Point];
    let mut mv: usize = 0;
    let start = Instant::now();
    let total_iters = 1_000_000_000_000;
    //let total_iters = 2022;
    while iter < total_iters {
        match curr {
            Shapes::Hline => {
                curr = Shapes::Plus;
                s = HLINE.as_slice();
            }
            Shapes::Plus => {
                curr = Shapes::El;
                s = PLUS.as_slice();
            }
            Shapes::El => {
                curr = Shapes::Vline;
                s = EL.as_slice();
            }
            Shapes::Vline => {
                curr = Shapes::Square;
                s = VLINE.as_slice();
            }
            Shapes::Square => {
                curr = Shapes::Hline;
                s = SQUARE.as_slice();
            }
        }
        let mut offsety = max + 4;
        let mut offsetx = 0;
        /*
        loop {
            if moves[mv] == '>' {
                if can_move(s, 1, 0, offsetx, offsety, &foo) {
                    offsetx += 1;
                }
            } else if moves[mv] == '<' {
                if can_move(s, -1, 0, offsetx, offsety, &foo) {
                    offsetx -= 1;
                }
            }
            mv = (mv + 1) % moves.len();

            if can_move(s, 0, -1, offsetx, offsety, &foo) {
                offsety -= 1;
            } else {
                break;
            }
        }
        max = max.max(s.last().unwrap().y as isize + offsety);
        /*         s.iter().for_each(|p| {
            foo[(p.y as isize + offsety) as usize][(p.x as isize + offsetx) as usize] = '#';
            mins[(p.x as isize + offsetx) as usize] =
                mins[(p.x as isize + offsetx) as usize].max((p.y as isize + offsety) as usize);
        }); */
        s.iter().for_each(|p| {
            foo[(p.y as isize + offsety) as usize][(p.x as isize + offsetx) as usize] = '#';
            mins[(p.x as isize + offsetx) as usize] =
                mins[(p.x as isize + offsetx) as usize].max((p.y as isize + offsety) as usize);
        });

        if max > (MAX_SIZE - 500) as isize {
            let lm = *mins.iter().min().unwrap();
            fast_compact(&mut foo, lm, max as usize);
            mins.iter_mut().for_each(|m| *m -= lm);
            max -= lm as isize;
            tmax += lm;
        }
        */
        loop {
            if moves[mv] == '>' && can_move_1d(s, 1, 0, offsetx, offsety, &board) {
                offsetx += 1;
            } else if moves[mv] == '<' && can_move_1d(s, -1, 0, offsetx, offsety, &board) {
                offsetx -= 1;
            }
            mv = (mv + 1) % moves.len();

            if can_move_1d(s, 0, -1, offsetx, offsety, &board) {
                offsety -= 1;
            } else {
                break;
            }
        }
        max = max.max(s.last().unwrap().y as isize + offsety);
        /*         s.iter().for_each(|p| {
            foo[(p.y as isize + offsety) as usize][(p.x as isize + offsetx) as usize] = '#';
            mins[(p.x as isize + offsetx) as usize] =
                mins[(p.x as isize + offsetx) as usize].max((p.y as isize + offsety) as usize);
        }); */
        for p in s.iter() {
            board[(p.y as isize + offsety) as usize * WIDTH + (p.x as isize + offsetx) as usize] =
                '#';
            mins[(p.x as isize + offsetx) as usize] =
                mins[(p.x as isize + offsetx) as usize].max((p.y as isize + offsety) as usize);
        }

        if max > (MAX_SIZE - 500) as isize {
            let lm = *mins.iter().min().unwrap();
            fast_compact_1d(&mut board, lm, max as usize);
            mins.iter_mut().for_each(|m| *m -= lm);
            max -= lm as isize;
            tmax += lm;
        }
        iter += 1;
        if iter % 100_000_000 == 0 {
            let elapsed = start.elapsed();
            let iters_per_sec = iter / (elapsed.as_secs() as usize).max(1);
            let time_left = (total_iters - iter) as f64 / iters_per_sec as f64;
            println!("Iterations per sec: {iters_per_sec}");
            println!("Total iters: {iter}");
            let duration = time_left as usize;
            let days = duration / 86400;
            let hours = (duration % 86400) / 3600;
            let minutes = (duration % 3600) / 60;
            let seconds = duration % 60;
            println!(
                "Time left: {days:02}:{hours:02}:{minutes:02}:{seconds:02}  - {elapsed:.2?} elapsed",
            );
        }
    }
    for (i, row) in board[0..(max as usize) * WIDTH].iter().rev().enumerate() {
        if i % WIDTH == 0 {
            println!();
        }
        print!("{row:?}");
    }
    println!();
    println!("{max:?}");
    println!("{tmax:?}");
    println!("{:?}", max + 1);
    println!("{:?}", tmax + max as usize + 1);
}
