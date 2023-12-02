use std::{
    fs,
    io::{self, BufRead},
};

fn read_file(path: &str) -> io::BufReader<fs::File> {
    let file = fs::File::open(path).unwrap();
    return io::BufReader::new(file);
}

#[allow(dead_code)]
pub fn solve() {
    let reader = read_file("./src/days/day04.txt");
    let mut epairs: [isize; 4] = [0; 4];
    let mut total: usize = 0;
    for line in reader.lines() {
        match line {
            Ok(l) => {
                let elves: Vec<_> = l.split(',').collect();
                let mut idx = 0;
                for elf in elves {
                    elf.split('-')
                        .map(|s| s.parse::<isize>().unwrap())
                        .enumerate()
                        .for_each(|(i, v)| epairs[idx + i] = v);
                    idx = 2;
                }
                if (epairs[0] <= epairs[3] && epairs[1] >= epairs[2])
                    || (epairs[0] >= epairs[3] && epairs[1] <= epairs[2])
                {
                    total += 1;
                }
                /* part 1
                if (epairs[0] >= epairs[2] && epairs[1] <= epairs[3])
                    || (epairs[0] <= epairs[2] && epairs[1] >= epairs[3])
                {
                    println!("{:?}", epairs);
                    total += 1;
                }
                */
            }
            Err(err) => println!("{err:?}"),
        }
    }
    println!("{total:?}");
}

// vJrwpWtwJgWr hcsFMMfFFhFp
