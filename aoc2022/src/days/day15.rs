use std::{
    collections::HashSet,
    fs,
    io::{self, BufRead},
};

fn read_file(path: &str) -> io::BufReader<fs::File> {
    let file = fs::File::open(path).unwrap();
    return io::BufReader::new(file);
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

struct Sensor {
    pos: Point,
    range: isize,
}

fn dist(p1: &Point, p2: &Point) -> Point {
    return Point {
        x: p1.x.abs_diff(p2.x) as isize,
        y: p1.y.abs_diff(p2.y) as isize,
    };
}

fn range(p1: &Point, p2: &Point) -> isize {
    let mdist = dist(&p1, &p2);
    return mdist.x + mdist.y;
}

#[allow(dead_code)]
pub fn solve() {
    let reader = read_file("./src/days/day15.txt");
    //let mut lpos: HashSet<isize> = HashSet::new();
    let mut lpos: HashSet<Point> = HashSet::new();
    let target = 2000000;

    //let max = 20;
    let min = 0;
    let max = 4000000;
    let mut sensors: Vec<Sensor> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(l) => {
                let x: Vec<_> = l
                    .replace("Sensor at x=", "")
                    .replace(" y=", "")
                    .replace(": closest beacon is at x=", ",")
                    .replace(" y=", "")
                    .split(',')
                    .map(|x| x.parse::<isize>().unwrap())
                    .collect();
                let beacon = Point { x: x[2], y: x[3] };
                let sensor = Point { x: x[0], y: x[1] };
                let beeg = range(&beacon, &sensor);
                sensors.push(Sensor {
                    pos: sensor,
                    range: beeg,
                })
                /* part 1
                if sensor.y == target {
                    lpos.insert(sensor.x);
                }
                if beacon.y == target {
                    lpos.remove(&beacon.x);
                }

                if sensor.y - beeg <= target && target <= sensor.y + beeg {
                    println!("in range {:?} {:?} - {beeg:?}", sensor.x, sensor.y);
                    let mut beg = 0;
                    if target >= sensor.y {
                        beg = beeg - (target - sensor.y);
                    } else if target < sensor.y {
                        beg = beeg - (sensor.y - target);
                    } else {
                        panic!("picke rick");
                    }
                    println!("beg: {:?} - {:?}", beg, beg * 2 + 1);
                    for i in sensor.x - beg..(sensor.x - beg) + (beg * 2 + 1) {
                        if beacon.y == target && i == beacon.x {
                            continue;
                        }
                        print!("({:?}, {:?}) - ", i, target);
                        lpos.insert(i);
                    }
                    println!();
                } */
            }
            Err(err) => {
                println!("{err}");
            }
        }
    }
    println!("{:?}", lpos.len());
    let mut distress = Point { x: -1, y: -1 };
    let mut i = min;
    let mut j = min;
    let mut found = false;
    let mut srange = 0;
    let mut jump = 0;
    let mut iters = 0;
    while i < max {
        while j < max {
            found = false;
            jump = 1;
            for sensor in &sensors {
                srange = range(&Point { x: i, y: j }, &sensor.pos);
                if srange <= sensor.range {
                    //jump = (sensor.range - srange).max(jump);
                    found = true;
                }
            }
            if !found {
                distress = Point { x: i, y: j };
                println!("scary {:?}", distress);
                j += 1;
            } else {
                j += jump;
            }
            iters += 1;
        }
        j = 0;
        i += 1;
    }
    println!("{distress:?}");
    println!("{iters:?}");
}
