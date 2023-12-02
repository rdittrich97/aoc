use std::{
    fs,
    io::{self, BufRead},
};

fn idx_fc(c: &char) -> usize {
    let num = *c as u8;
    match num {
        97..=122 => return (num - 97) as usize,
        65..=90 => return (num - 39) as usize,
        _ => unimplemented!(),
    }
}

#[allow(dead_code)]
pub fn solve() {
    let file = fs::File::open("./src/days/day03.txt").unwrap();
    let reader = io::BufReader::new(file);
    let mut search: [usize; 52] = [0; 52];
    let mut tempsearch: [usize; 52] = [0; 52];
    let mut tot = 0;
    let mut ln = 0;
    for line in reader.lines() {
        if ln % 3 == 0 {
            for (i, s) in search.iter().enumerate() {
                if *s == 3 {
                    tot += i + 1;
                }
            }
            search = [0; 52];
        }
        match line {
            Ok(l) => {
                l.chars().for_each(|c| tempsearch[idx_fc(&c)] = 1);
                for i in 0..search.len() {
                    search[i] += tempsearch[i];
                }
                tempsearch = [0; 52];
                /* p1
                let strs = l.split_at(l.len() / 2);
                 strs.0.chars().for_each(|c| search[idx_fc(&c)] += 1);
                 let mut idx;
                 for c in strs.1.chars() {
                     idx = idx_fc(&c);
                     if search[idx] > 0 {
                         tot += idx + 1;
                         search[idx] = false;
                     }
                 }
                 search = [false; 52];
                 */
            }
            Err(err) => println!("{:?}", err),
        }
        ln += 1;
    }
    for (i, s) in search.iter().enumerate() {
        if *s == 3 {
            tot += i + 1;
        }
    }
    println!("{:?}", tot);
}

// vJrwpWtwJgWr hcsFMMfFFhFp
