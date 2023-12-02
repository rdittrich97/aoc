#![allow(clippy::needless_return)]
#[allow(dead_code)]
pub fn solve() {
    let lines = common::read_file("./src/days/data/day01.txt");
    let mut tot = 0;
    let numbs = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for line in lines.iter() {
        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;
        let mut buf = String::new();
        for c in line.chars() {
            let v: u32 = match c.to_digit(10) {
                Some(val) => val,
                None => {
                    buf.push(c);
                    let mut ret: u32 = 0;
                    for i in 0..numbs.len() {
                        if buf.contains(numbs[i]) {
                            buf.clear();
                            buf.push(c);
                            ret = i as u32 + 1;
                            break;
                        }
                    }
                    ret
                }
            };
            if v == 0 {
                continue;
            }
            if first.is_none() {
                first = Some(v);
            } else {
                last = Some(v);
            }
        }
        if last.is_none() {
            tot += (first.expect("first to be set") * 10) + first.expect("first to be set");
        } else {
            tot += (first.expect("first to be set") * 10) + last.expect("last to be set");
        }
    }
    println!("{:?}", tot);
}
