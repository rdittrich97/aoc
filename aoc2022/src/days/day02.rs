use std::fs;

#[derive(PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from_string(input: char) -> Self {
        match input {
            'A' | 'X' => return Shape::Rock,
            'B' | 'Y' => return Shape::Paper,
            'C' | 'Z' => return Shape::Scissors,
            _ => todo!(),
        }
    }
    fn value(&self) -> u8 {
        match self {
            Shape::Rock => return 1,
            Shape::Paper => return 2,
            Shape::Scissors => return 3,
        }
    }
}

#[derive(PartialEq)]
enum Game {
    Loss,
    Tie,
    Win,
}

impl Game {
    fn value(&self) -> u8 {
        match self {
            Game::Loss => return 0,
            Game::Tie => return 3,
            Game::Win => return 6,
        }
    }

    fn play(s1: &Shape, s2: &Shape) -> Self {
        if *s1 == *s2 {
            return Game::Tie;
        }

        if *s1 == Shape::Scissors && *s2 == Shape::Rock {
            return Game::Win;
        }
        if *s1 == Shape::Rock && *s2 == Shape::Paper {
            return Game::Win;
        }
        if *s1 == Shape::Paper && *s2 == Shape::Scissors {
            return Game::Win;
        }
        return Game::Loss;
    }

    fn from_char(c: char) -> Self {
        match c {
            'X' => return Game::Loss,
            'Y' => return Game::Tie,
            'Z' => return Game::Win,
            _ => unimplemented!(),
        }
    }

    fn from_strat(s1: Shape, o: &Self) -> Shape {
        if *o == Game::Win {
            match s1 {
                Shape::Scissors => return Shape::Rock,
                Shape::Rock => return Shape::Paper,
                Shape::Paper => return Shape::Scissors,
            }
        }

        if *o == Game::Loss {
            match s1 {
                Shape::Rock => return Shape::Scissors,
                Shape::Paper => return Shape::Rock,
                Shape::Scissors => return Shape::Paper,
            }
        }

        return s1;
    }
}
#[allow(dead_code)]
pub fn solve() {
    let strats = fs::read_to_string("./src/days/day02.txt").expect("Error in reading the file");
    let mut pts: usize = 0;
    let rounds: Vec<&str> = strats.split('\n').collect();
    for s in rounds {
        let s2 = match s.chars().nth(2) {
            Some(c) => Game::from_char(c),
            None => break,
        };
        pts += s2.value() as usize;
        pts +=
            Game::from_strat(Shape::from_string(s.chars().next().unwrap()), &s2).value() as usize;
    }
    println!("{pts:?}");
}
