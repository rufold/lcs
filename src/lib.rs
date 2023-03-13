#![deny(clippy::pedantic)]
#![allow(dead_code)]
use colour::green;
use rand::{thread_rng, Rng};
use std::cmp;

#[derive(Clone, Copy, PartialEq)]
enum Arrow {
    Vertical,
    Horizontal,
    Diagonal,
    Empty,
}

#[derive(Clone, Copy)]
struct Cell {
    length: u32,
    direction: Arrow,
    path: bool,
}

impl Cell {
    fn new() -> Self {
        Self {
            length: 0,
            direction: Arrow::Empty,
            path: false,
        }
    }
}

#[must_use]
pub fn lcs(a: &str, b: &str, vis: bool) -> String {
    let row_chars: Vec<char> = a.chars().collect();
    let column_chars: Vec<char> = b.chars().collect();

    let mut table: Vec<Vec<Cell>> =
        vec![vec![Cell::new(); column_chars.len() + 1]; row_chars.len() + 1];

    for (i1, c1) in row_chars.iter().enumerate() {
        for (i2, c2) in column_chars.iter().enumerate() {
            if c1 == c2 {
                table[i1 + 1][i2 + 1].length = table[i1][i2].length + 1;
                table[i1 + 1][i2 + 1].direction = Arrow::Diagonal;
            } else {
                table[i1 + 1][i2 + 1].length =
                    cmp::max(table[i1 + 1][i2].length, table[i1][i2 + 1].length);
                table[i1 + 1][i2 + 1].direction =
                    if table[i1 + 1][i2].length > table[i1][i2 + 1].length {
                        Arrow::Horizontal
                    } else {
                        Arrow::Vertical
                    }
            }
        }
    }

    let mut lcs: Vec<char> = Vec::new();
    let (mut x, mut y) = (a.len(), b.len());

    while x > 0 && y > 0 {
        table[x][y].path = true;
        if row_chars[x - 1] == column_chars[y - 1] {
            lcs.push(row_chars[x - 1]);
        }
        match table[x][y].direction {
            Arrow::Diagonal => {
                x -= 1;
                y -= 1;
            }
            Arrow::Vertical => x -= 1,
            Arrow::Horizontal => y -= 1,
            Arrow::Empty => (),
        }
    }
    if vis {
        visualize(&column_chars, row_chars, &table);
    }

    lcs.iter().rev().collect::<String>()
}

fn lcs_length(a: &str, b: &str) -> u32 {
    let mut lens = vec![0; b.len() + 1];
    for ac in a.chars() {
        let mut prev = 0;
        for (i, bc) in b.chars().enumerate() {
            let temp = lens[i + 1];
            lens[i + 1] = if ac == bc {
                prev + 1
            } else {
                lens[i].max(lens[i + 1])
            };
            prev = temp;
        }
    }
    lens[b.len()]
}

fn chavatal_sankoff() {
    let ks = vec![2, 4, 8, 16];
    let ns = vec![100, 500, 1000];

    for k in ks {
        for n in &ns {
            let s1 = random_string(*n, k);
            let s2 = random_string(*n, k);
            let length = lcs_length(&s1, &s2);
            println!("n: {n}, k: {k}");
            println!("dlugosc nwp: {length}");
            println!("dlugosc / n: {}", f64::from(length) / f64::from(*n));
            println!("--------------");
        }
    }
}

fn visualize(column_chars: &[char], mut row_chars: Vec<char>, table: &[Vec<Cell>]) {
    print!("        ");
    for l in column_chars {
        print!("{l}   ");
    }
    println!();

    row_chars.insert(0, ' ');
    for (i, v) in table.iter().enumerate() {
        print!("{} ", row_chars[i]);
        for n in v {
            match n.direction {
                Arrow::Diagonal => print!("\\"),
                Arrow::Horizontal => print!("-"),
                Arrow::Vertical => print!("|"),
                Arrow::Empty => print!(" "),
            }
            if n.path {
                green!("{:^3}", n.length);
            } else {
                print!("{:^3}", n.length);
            }
        }
        println!();
    }
}

fn random_string(n: u32, k: u8) -> String {
    let mut rng = thread_rng();
    let string: String = (0..n)
        .map(|_| rng.gen_range(65..=(65 + k)) as char)
        .collect();
    string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chavatal_sankoff_test() {
        chavatal_sankoff();
    }

    #[test]
    fn it_works() {
        assert_eq!(lcs("nawigator", "nowator", true), "nwator");
    }

    #[test]
    fn test_five() {
        assert_eq!(lcs("algorytm", "program", false), "grm");
        assert_eq!(lcs("longest", "common", false), "on");
        assert_eq!(lcs("bacbacba", "abbaac", false), "baac");
        assert_eq!(lcs("rust", "crust", false), "rust");
        assert_eq!(lcs("calkowita", "zmiennoprzecinkowa", false), "ckowa");
    }

    #[test]
    fn test_length() {
        assert_eq!(lcs_length("algorytm", "program"), 3);
        assert_eq!(lcs_length("longest", "common"), 2);
        assert_eq!(lcs_length("bacbacba", "abbaac"), 4);
        assert_eq!(lcs_length("rust", "crust"), 4);
        assert_eq!(lcs_length("calkowita", "zmiennoprzecinkowa"), 5);
    }
}
