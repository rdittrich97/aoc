use std::fs;

pub fn solve() {
    let cals = fs::read_to_string("./src/days/day01.txt").expect("Error in reading the file");
    let elves: Vec<&str> = cals.split("\n\n").collect();
    let mut big: [isize; 3] = [0, 0, 0];
    for elf in &elves {
        let v = elf
            .split('\n')
            .map(|s| s.parse::<isize>().unwrap())
            .fold(0, |a, b| a + b);
        let min = big.iter().min().unwrap();
        if v > *min {
            let midx = big.iter().position(|&x| x == *min).unwrap();
            big[midx] = v;
        }
    }
    println!("{:?}", big.iter().sum::<isize>());
    println!("{:?}", big);
}
