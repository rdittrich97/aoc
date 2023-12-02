use std::{error::Error, fs, io, time::Instant};

pub fn parse_args(args: &Vec<String>) -> Result<Vec<u8>, Box<dyn Error>> {
    if args.len() < 2 {
        return Err("specify the number of days".into());
    }
    return Ok(args[1..]
        .iter()
        .map(|x| x.parse::<u8>())
        .collect::<Result<Vec<_>, _>>()?);
}

pub fn time_function(day: u8, day_solver: fn()) -> f64 {
    println!("Day: {:02}", day);
    let start = Instant::now();
    day_solver();
    let end = start.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Took: {:.4} ms", end);
    return end;
}

pub fn read_file(path: &str) -> Vec<String> {
    let file = fs::read_to_string(path).unwrap();
    return file.split("\n").map(|x| x.to_string()).collect();
}
