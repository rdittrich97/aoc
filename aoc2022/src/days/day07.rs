use std::{
    collections::HashMap,
    fs,
    io::{self, BufRead, BufReader, Read},
};

enum Command {
    Ls,
    Cd(String),
}

fn read_file(path: &str) -> io::BufReader<fs::File> {
    let file = fs::File::open(path).unwrap();
    return io::BufReader::new(file);
}
/* fn rparse(lines: &Vec<&str>, dirs: &mut HashMap<String, usize>, dir: &String, idx: usize) -> usize {
    let line = lines.get(idx);
    let mut size: usize = 0;
    match line {
        Some(l) => {
            println!("{l}");
            if l.chars().nth(0).unwrap() == '$' {
                let cmds: Vec<_> = l.split_ascii_whitespace().collect();
                if cmds[1] == "cd" {
                    if cmds[2].to_string() == ".." {
                        println!("exiting {dir} -  {size}");
                        dirs.insert(dir.to_string(), size);
                        return size;
                    } else {
                        size += rparse(lines, dirs, &cmds[2].to_string(), idx + 1);
                    }
                }
            } else {
                let files: Vec<_> = l.split_ascii_whitespace().collect();
                if files[0] != "dir" {
                    size += files[0].parse::<usize>().unwrap();
                }
                println!("size: {size} - {dir}");
            }
        }
        None => {
            return 0;
        }
    }
    let ns = size + rparse(lines, dirs, dir, idx + 1);
    dirs.insert(dir.to_string(), ns);
    return ns;
}

fn parse(lines: Vec<&str>, dirs: &mut HashMap<String, usize>) -> usize {
    let root: Option<Vec<_>> = lines.get(0).map(|x| x.split_ascii_whitespace().collect());
    let p;
    match root {
        Some(v) => {
            p = v.get(2).unwrap().clone();
        }
        None => {
            return 0;
        }
    }
    return rparse(&lines, dirs, &p.to_string(), 1);
}
 */
pub fn solve() {
    let reader = read_file("./src/days/day07.txt");

    let mut dstack: Vec<usize> = Vec::new();
    let mut totals: Vec<usize> = Vec::new();
    // stolen https://www.reddit.com/r/adventofcode/comments/10je75e/2022_day_7rust_working_with_trees_seems_like_a/j5pbj4w/
    for line in reader.lines() {
        match line {
            Ok(l) => {
                if l.starts_with("$ cd") {
                    let path = l.split_whitespace().nth(2).unwrap();
                    if path == ".." {
                        let total = dstack.pop().unwrap();
                        totals.push(total);
                        if let Some(top) = dstack.last_mut() {
                            *top += total;
                        }
                    } else {
                        dstack.push(0);
                    }
                } else {
                    let files_size = l.split_ascii_whitespace().next().unwrap();
                    if let Ok(v) = files_size.parse::<usize>() {
                        *dstack.last_mut().unwrap() += v;
                    }
                }
                /*if l.chars().nth(0).unwrap() == '$' {
                    let cmds: Vec<_> = l.split_ascii_whitespace().collect();
                    if cmds[1] == "cd" {
                        if cmds[2].to_string() == ".." {
                            dir = dstack.remove(0);
                            match dirs.get(&dir.to_string()) {
                                Some(v) => {
                                    size += v;
                                    dirs.insert(dir.to_string(), size);
                                }
                                None => {
                                    dirs.insert(dir.to_string(), size);
                                }
                            }
                        } else {
                            dstack.insert(0, dir);
                            dir = cmds[2].to_string();
                            size = 0;
                        }
                    }
                    continue;
                } else {
                    let files: Vec<_> = l.split_ascii_whitespace().collect();
                    if files[0] != "dir" {
                        match dirs.get(&dir.to_string()) {
                            Some(v) => {
                                size = v + files[0].parse::<usize>().unwrap();

                                dirs.insert(
                                    dir.to_string(),
                                    v + files[0].parse::<usize>().unwrap(),
                                );
                            }
                            None => {
                                dirs.insert(dir.to_string(), files[0].parse::<usize>().unwrap());
                                size = files[0].parse::<usize>().unwrap();
                            }
                        }
                    }
                } */
            }
            Err(err) => {
                println!("{err}");
            }
        }
    }
    /*     while dstack.len() > 0 {
        dir = dstack.remove(0);
        match dirs.get(&dir) {
            Some(v) => {
                dirs.insert(dir.to_string(), v + size);
            }
            None => {
                dirs.insert(dir.to_string(), size);
            }
        }
    }
     */
    while let Some(v) = dstack.pop() {
        if let Some(top) = dstack.last_mut() {
            *top += v;
        }
        totals.push(v);
    }
    let sum = totals.iter().filter(|v| *v <= &100000).sum::<usize>();
    let space = 30000000 - (70000000 - totals.last().unwrap());
    let p2 = totals.iter().filter(|v| *v >= &space).min();
    println!("{:?}", dstack);
    println!("{:?}", totals);
    println!("{:?}", sum);
    println!("{:?}", p2);
}
