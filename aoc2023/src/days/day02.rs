#![allow(clippy::needless_return)]
#[allow(dead_code)]
pub fn solve() {
    let lines = common::read_file("./src/days/data/day02.txt");
    let mut tot = 0;
    let mut imp = 0;
    let mut min = [0, 0, 0];
    let mut power = 0;
    let mut bad = false;
    for line in &lines {
        let game: Vec<_> = line
            .replace("Game ", "")
            .replace(": ", ";")
            .replace("; ", ";")
            .replace(", ", ",")
            .split(";")
            .map(std::string::ToString::to_string)
            .collect();
        let id = game[0].parse::<u32>().unwrap();
        tot += id;
        for pull in game[1..].iter() {
            for cubes in pull.split(",") {
                let data: Vec<_> = cubes.split(" ").collect();
                let num = data[0].parse::<u32>().unwrap();
                let color = data[1];
                let idx: usize = match color {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    _ => unreachable!(),
                };
                if num > idx as u32 + 12 {
                    bad = true;
                }
                min[idx] = min[idx].max(num);
            }
        }
        if bad {
            imp += id;
            bad = false;
        }
        power += min.iter().product::<u32>();
        min = [0, 0, 0];
    }
    println!("p1: {:?} - {:?} = {:?}", tot, imp, tot - imp);
    println!("p2: {power:?}");
}
