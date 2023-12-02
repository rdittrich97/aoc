use std::{
    fs,
    io::{self, BufRead},
};

fn read_file(path: &str) -> io::BufReader<fs::File> {
    let file = fs::File::open(path).unwrap();
    return io::BufReader::new(file);
}

#[derive(Debug, Clone)]
enum EPacket {
    Number(isize),
    Array(Vec<EPacket>),
}

fn rparse(s: &str, idx: usize) -> (EPacket, usize) {
    let mut i: usize = idx;
    let mut temp: String = String::new();
    let mut root = EPacket::Array(Vec::new());
    while i < s.len() {
        let ch = s.chars().nth(i).unwrap();
        match ch {
            '[' => {
                if let EPacket::Array(arr) = &mut root {
                    arr.push(EPacket::Number(temp.parse::<isize>().unwrap()));
                }
            }
            ']' => {
                if !temp.is_empty() {
                    if let EPacket::Array(arr) = &mut root {
                        arr.push(EPacket::Number(temp.parse::<isize>().unwrap()));
                    }
                }
                return (root, i + 1);
            }
            ',' => {
                if !temp.is_empty() {
                    if let EPacket::Array(arr) = &mut root {
                        arr.push(EPacket::Number(temp.parse::<isize>().unwrap()));
                    }
                }
                temp = String::new();
            }
            '0'..='9' => temp.push(ch),
            _ => {
                panic!("stinky");
            }
        }

        i += 1;
    }

    return (root, i);
}

fn is_rpacket_ordered(p1: &EPacket, p2: &EPacket) -> Option<bool> {
    match (p1, p2) {
        (EPacket::Number(num), EPacket::Number(num2)) => {
            if num < num2 {
                return Some(true);
            }
            if num > num2 {
                return Some(false);
            }
            if num == num2 {
                return None;
            }
        }
        (EPacket::Array(arr), EPacket::Array(arr2)) => {
            let mut i = 0;
            let mut j = 0;
            while i < arr.len() && j < arr2.len() {
                let res = is_rpacket_ordered(arr.get(i).unwrap(), arr2.get(j).unwrap());
                if res.is_some() {
                    return res;
                }
                i += 1;
                j += 1;
            }

            if i < arr.len() && j >= arr2.len() {
                return Some(false);
            } else if i >= arr.len() && j < arr2.len() {
                return Some(true);
            }
            return None;
        }
        (EPacket::Array(arr), EPacket::Number(num)) => {
            return is_rpacket_ordered(
                &EPacket::Array(arr.clone()),
                &EPacket::Array(vec![EPacket::Number(*num)]),
            );
        }
        (EPacket::Number(num), EPacket::Array(arr)) => {
            return is_rpacket_ordered(
                &EPacket::Array(vec![EPacket::Number(*num)]),
                &EPacket::Array(arr.clone()),
            );
        }
    }
    return Some(false);
}

fn is_epacket_ordered(p1: &EPacket, p2: &EPacket) -> bool {
    if let Some(res) = is_rpacket_ordered(p1, p2) {
        return res;
    }
    return false;
}

fn epparse(s: &str) -> EPacket {
    return rparse(s, 1).0;
}

fn sort(packets: &mut Vec<EPacket>, div1: usize, div2: usize) -> (usize, usize) {
    let mut d1 = div1;
    let mut d2 = div2;
    for i in 0..packets.len() {
        for j in i..packets.len() {
            if !is_epacket_ordered(&packets[i], &packets[j]) {
                if i == d1 {
                    d1 = j;
                } else if i == d2 {
                    d2 = j;
                }
                if j == d1 {
                    d1 = i;
                } else if j == d2 {
                    d2 = i;
                }
                packets.swap(i, j);
            }
        }
    }
    return (d1, d2);
}

#[allow(dead_code)]
pub fn solve() {
    let reader = read_file("./src/days/day13.txt");
    let mut first = true;
    let mut r1: EPacket = EPacket::Number(0);
    let mut r2: EPacket = EPacket::Number(0);
    let mut idx: usize = 1;
    let mut rtot: usize = 0;
    let mut packets: Vec<EPacket> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(l) => {
                if l.is_empty() {
                    if is_epacket_ordered(&r1, &r2) {
                        rtot += idx;
                    }
                    idx += 1;
                    first = true;
                    continue;
                }
                if first {
                    r1 = epparse(&l);
                    packets.push(epparse(&l));
                    first = false;
                } else {
                    r2 = epparse(&l);
                    packets.push(epparse(&l));
                }
            }
            Err(err) => {
                println!("{err}");
            }
        }
    }
    if is_epacket_ordered(&r1, &r2) {
        rtot += idx;
    }
    packets.push(epparse("[[2]]"));
    let d1 = packets.len() - 1;
    packets.push(epparse("[[6]]"));
    let d2 = packets.len() - 1;
    let (div1, div2) = sort(&mut packets, d1, d2);
    println!("{}", (div1 + 1) * (div2 + 1));
    println!("rtot: {rtot}");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_order_epacket() {
        assert_eq!(
            is_epacket_ordered(&mut epparse("[1,1,3,1,1]"), &mut epparse("[1,1,5,1,1]")),
            true
        );
        assert_eq!(
            is_epacket_ordered(&mut epparse("[[1],[2,3,4]]"), &mut epparse("[[1],4]")),
            true
        );
        assert_eq!(
            is_epacket_ordered(&mut epparse("[9]"), &mut epparse("[[8,7,6]]")),
            false
        );
        assert_eq!(
            is_epacket_ordered(&mut epparse("[[4,4],4,4]"), &mut epparse("[[4,4],4,4,4]")),
            true
        );
        assert_eq!(
            is_epacket_ordered(&mut epparse("[7,7,7,7]"), &mut epparse("[7,7,7]")),
            false
        );
        assert_eq!(
            is_epacket_ordered(&mut epparse("[]"), &mut epparse("[3]")),
            true
        );
        assert_eq!(
            is_epacket_ordered(&mut epparse("[[[]]]"), &mut epparse("[[]]")),
            false
        );
        assert_eq!(
            is_epacket_ordered(
                &mut epparse("[1,[2,[3,[4,[5,6,7]]]],8,9]"),
                &mut epparse("[1,[2,[3,[4,[5,6,0]]]],8,9]")
            ),
            false
        );
        assert_eq!(
            is_epacket_ordered(&mut epparse("[5,6,6,7,3]"), &mut epparse("[5,6,6,7]")),
            false
        );
        assert_eq!(
            is_epacket_ordered(
                &mut epparse("[[7,6,5],[9,1]]"),
                &mut epparse("[[[3,2,[1,0,9,2,7],4,2],[[4,10,3,4],6,[0,4]],[],[9,[1,0],[]]],[],[5,[4,[4,10,9,6,3],3],[[8,2,8],[10,7,7,1],10,[],5],[9],9],[0,7,3,5,10]]")
            ),
            false
        );
        assert_eq!(
            is_epacket_ordered(
                &mut epparse("[[[0,4],[[10,8,6,3],7],3],[],[[],[[2,5,3],[],[6,7,10,7],5,7]],[5,9,[[],[4,3],3,[1,4,3]],3],[10,5,3,5,[9,2,[9,9],10,[3,8,9,8,5]]]]"),
                &mut epparse("[[[6,2,[8,2,4,6]],[9,6,[],[]],[2,[9,7,8],[8,8,5,7],[6,7,0,10]],[[0,7]],8],[[[6],[9],1],[[],5]],[[7,0,[4,8],10],5,7,8]]")
            ),
            true
        );
        assert_eq!(
            is_epacket_ordered(
                &mut epparse("[[[[6,10,2,9,8],9],[[9,7,6],4,[2,10,5],[]],8,4,[[7,1,8],[0],[5]]]]"),
                &mut epparse(
                    "[[8,[2,[2],8,[]]],[[3,[10,0,8,3,9]]],[[4],[],[[7,6,8,10],3,[7,6],[0,8]]]]"
                )
            ),
            true
        );
        assert_eq!(
            is_epacket_ordered(&mut epparse("[[3,[7],[0],7]]"), &mut epparse("[[5,2]]")),
            true
        );
        assert_eq!(
            is_epacket_ordered(
                &mut epparse("[[[[6,1,4,1],8,6,10,[5,3,9,10]],[[],4,0],9],[4,1,7,6],[[5,[3,8,1,4,6],1,[]],7]]"),
                &mut epparse("[[],[7,[],[1,[6,10,5,3],[1],7,[9,0,5,0,5]],[7,5,4,3],[[4,7,6],[],4,[8,9,2,9,3]]],[[0,[],[8,4,8],1],4,0,[6,[8,10,1,8,2],3,6]],[]]")
            ),
            false
        );
        assert_eq!(
            is_epacket_ordered(
                &mut epparse("[[[10,[1,5],[1,4,3,0,8],2,[9]],[],3],[[[8,0,5],[],2],[2],5,[1,6,[8,8,7],5]],[10,7,9],[]]"),
                &mut epparse("[[[[1],[8,0,0,8,1],[1],[8,4]],[2,[],[2],4,[1,8,9]],9],[],[[4,8,[8,0],[10,0,10,0,0]],1,2,1],[[6],5,6,9,6],[[[5],[7,4,9],2,[6]],9,[[10],6]]]")
            ),
            false
        );
        assert_eq!(
            is_epacket_ordered(
                &mut epparse("[[[[1,7,4,3,2],4,8,[2,2,6],4],6,6],[3,3,3,1,5],[[1]],[10,[]],[2]]"),
                &mut epparse("[[[],3],[[],7],[8,0,8],[8,2]]")
            ),
            false
        );
        assert_eq!(
            is_epacket_ordered(
                &mut epparse("[[0],[0],[[],[10,2,6,[],1],0,[[1],[6,9,6,7],[],0,2],[6,[6,10,5,5],[8,5,7,4]]],[],[]]"),
                &mut epparse("[[8,[[2,6,8,4],1]],[[],[],[[8,3,0,8],6],[3,[1,3],8]],[1,10,[2,2,[0,5,2,9],[4,10,4,8]]],[[8,9,6],[[],[4]]]]")
            ),
            true
        );
        assert_eq!(
            is_epacket_ordered(
                &mut epparse("[[4,2,[[4,3],2,[4,5,5,1,1]]],[[8,[7],[9,7],4,[0,3,8,6]],0],[5],[2,2,[10,10,[3,6,10,2,5],[],0]]]"),
                &mut epparse("[[[[6,3,2,2,6],1,[0,2,5,9,4],[6],[2]],9,[7,3]],[[[7],7,4,2,2],5,6,[3],3]]")
            ),
            true
        );
        assert_eq!(
            is_epacket_ordered(
                &mut epparse("[[1,4,6,[]]]"),
                &mut epparse("[[1,3],[2],[6,6,0,[5,[],9]],[[2,[1,4,5],[6],[5,10,9,4]],[7,4,6,[2,6,8,9],[5,6]],[[8],[]],[[9,5,6],[]],8],[[0,[2,9,6,3],[5,3]]]]")
            ),
            false
        );
    }
}
