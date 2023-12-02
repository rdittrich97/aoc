use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    io::{self, BufRead},
    vec,
};

fn read_file(path: &str) -> io::BufReader<fs::File> {
    let file = fs::File::open(path).unwrap();
    return io::BufReader::new(file);
}

#[derive(Debug, Clone)]
struct Valve {
    id: String,
    flow: isize,
    leads: Vec<String>,
    distances: HashMap<String, isize>,
}

fn calc_steps(
    tunnels: &HashMap<String, Valve>,
    mem: &mut HashSet<String>,
    q: &mut VecDeque<(String, isize)>,
    res: &mut HashMap<String, isize>,
) {
    if q.is_empty() {
        return;
    }

    let (next, depth) = q.pop_front().expect("Queue is empty");
    res.insert(next.clone(), depth);
    mem.insert(next.clone());
    for lead in &tunnels
        .get(&next)
        .unwrap_or_else(|| panic!("Unable to get {next}"))
        .leads
    {
        if !mem.contains(lead) {
            q.push_back((lead.clone(), depth + 1));
        }
    }
    calc_steps(tunnels, mem, q, res);
}

fn dum(
    tunnels: &HashMap<String, Valve>,
    mem: &HashSet<String>,
    node: &String,
    time: isize,
    opts: &HashSet<String>,
) -> (isize, String) {
    let mut res = 0;
    let mut pp = String::new();
    if time <= 0 {
        return (0, String::new());
    }
    for lead in opts.difference(mem) {
        let mut mn: HashSet<_> = HashSet::new();
        mn.extend(mem.clone());
        mn.insert(lead.clone());

        let (t, p) = dum(
            tunnels,
            &mn,
            lead,
            time - tunnels[node].distances[lead],
            opts,
        );
        if t > res {
            res = t;
            pp = p.clone();
        }
    }
    return (res + time * tunnels[node].flow, node.clone() + &pp);
}

#[allow(dead_code)]
pub fn solve() {
    let reader = read_file("./src/days/day16.txt");
    let mut tunnels: HashMap<String, Valve> = HashMap::new();
    let start = "AA".to_owned();
    for line in reader.lines() {
        match line {
            Ok(l) => {
                let x: Vec<_> = l
                    .replace("Valve ", "")
                    .replace(" has flow rate=", ",")
                    .replace("; tunnels lead to valves ", ",")
                    .replace("; tunnel leads to valve ", ",")
                    .replace(' ', "")
                    .split(',')
                    .map(std::string::ToString::to_string)
                    .collect();
                tunnels.insert(
                    x[0].clone(),
                    Valve {
                        id: x[0].clone(),
                        flow: x[1].parse().unwrap(),
                        leads: x[2..x.len()]
                            .iter()
                            .map(std::string::ToString::to_string)
                            .collect::<Vec<String>>(),
                        distances: HashMap::new(),
                    },
                );
            }
            Err(err) => {
                println!("{err}");
            }
        }
    }

    let tunns: Vec<_> = tunnels.keys().map(std::clone::Clone::clone).collect();
    for tunnel in &tunns {
        let mut res = HashMap::new();
        calc_steps(
            &tunnels,
            &mut HashSet::new(),
            &mut VecDeque::from(vec![(tunnel.clone(), 1)]),
            &mut res,
        );
        tunnels.get_mut(tunnel).unwrap().distances = res.clone();
        res.clear();
    }
    let options: HashSet<String> = tunnels
        .iter()
        .filter(|(_, v)| v.flow > 0)
        .map(|(k, _)| k.clone())
        .collect();
    println!("{tunnels:?}");

    let tmp = tunnels.clone();
    for (k, v) in &tmp {
        for (n, d) in &v.distances {
            if *d < tunnels[n].distances[k] {
                tunnels.get_mut(n).unwrap().distances.insert(k.clone(), *d);
            }
        }
    }
    let x = dum(&tunnels, &HashSet::new(), &start, 30, &options);
    println!("{x:?}");

    let mut tot = 0;
    for i in 1..options.len() / 2 {
        println!("{i}");
        for c in options.iter().combinations(i) {
            let os = c
                .into_iter()
                .map(std::string::ToString::to_string)
                .collect::<HashSet<_>>();
            let s1 = dum(&tunnels, &HashSet::new(), &start, 26, &os);
            let s2 = dum(
                &tunnels,
                &HashSet::new(),
                &start,
                26,
                &(options
                    .difference(&os)
                    .map(std::string::ToString::to_string)
                    .collect::<HashSet<_>>()),
            );
            if s1.0 + s2.0 > tot {
                tot = s1.0 + s2.0;
            }
        }
    }
    println!("{tot}");
}
