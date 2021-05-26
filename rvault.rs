use std::io::{prelude::*, BufReader};
use std::fs::File;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::Write;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Num(u16),
    Oper(char),
}

#[derive(Default)]
struct Vault {
    grid: HashMap<(u16, u16), Cell>,
    start: (u16, u16),
    end: (u16, u16),
    end_weight: u16,

    // these two used as stacks, to avoid too many memory allocations
    dir: String,
    txt: String,
}

fn main() -> std::io::Result<()> {
    let mut vault = Vault::default();
    let file = BufReader::new(File::open("vault_lock.txt")?);
    for (j, line) in file.lines().enumerate() {
        let j = j.try_into().unwrap();
        for (i, field) in line?.split_whitespace().filter(|x| !x.contains('|')).enumerate() {
            let i = i.try_into().unwrap();
            match field {
                "|" => continue,
                assign if assign.contains('=') => {
                    let (name, weight) = field.split_at(field.find('=').unwrap());
                    // skip '=' that is returned part of the weight after split_at
                    let weight = (&weight[1..]).parse::<u16>().expect("weight isn't a valid number");
                    println!("weight {}", weight);
                    vault.grid.insert((i, j), Cell::Num(weight));
                    match name {
                        "orb" => vault.start = (i, j),
                        "vault" => { vault.end = (i - 1, j); vault.end_weight = weight },
                        _ => panic!("name {:?} not recognized", name),
                    }
                },
                _ => {
                    let cell = match field.parse::<u16>() {
                        Ok(u) => Cell::Num(u),
                        Err(_) => Cell::Oper(field.chars().next().unwrap()),
                    };
                    vault.grid.insert((i, j), cell);
                }
            }
        }
    }

    // now solve the puzzle
    println!("start_i {}, start_j {}, end_i {}, end_j {}", vault.start.0, vault.start.1, vault.end.0, vault.end.1);
    if let Cell::Num(weight) = vault.grid.get(&vault.start).unwrap() {
        let weight = *weight;
        write!(vault.txt, "{}", weight).unwrap();
        vault.go(vault.start, weight, '$');
    } else {
        panic!("starting cell has got an unvalid weight: {:?}", vault.grid.get(&vault.start).unwrap());
    }
    Ok(())
}

impl Vault {
    fn go(&mut self, pos: (u16, u16), mut weight: u16, mut oper: char) {
        if self.dir.len() > 12 {
            return;
        }
        let old_txt_len = self.txt.len();
        let cell = self.grid.get(&pos).unwrap();
        match *cell {
            Cell::Num(c) => {
                match oper {
                    '-' => weight -= c,
                    '+' => weight += c,
                    '*' => weight *= c,
                    '$' => {} // skip the start cell
                    _ => panic!("oper not recognized: {:?}", oper)
                }
                if oper != '$' { write!(self.txt, " {}{}", oper, c).unwrap(); }
                oper = '@'; // an invalid value
            },
            Cell::Oper(o) => oper = o, // store here because it applies to the next step with a Cell::Num
        }
        if weight == self.end_weight {
            if self.end == pos {
            for z in self.dir.chars() {
                println!("{}", match z {
                    'N' => "north",
                    'S' => "south",
                    'E' => "east",
                    'W' => "west",
                    _ => "?",
                });
            }}
            println!("{} -> {} -> {} ok{}", self.dir, self.txt, weight, if self.end == pos {" OK"} else {""} );
        }
        if self.end != pos {
            let (i, j) = pos;
            let (start_i, start_j) = self.start;
            let (end_i, end_j) = self.end;
            if i > start_i && (i-1 != start_i || j != start_j) {
                self.dir.push('W');
                self.go((i - 1, j), weight, oper);
                self.dir.pop();
            }
            if i < end_i {
                self.dir.push('E');
                self.go((i + 1, j), weight, oper);
                self.dir.pop();
            }
            if j > end_j {
                self.dir.push('N');
                self.go((i, j - 1), weight, oper);
                self.dir.pop();
            }
            if j < start_j && (i != start_i || j+1 != start_j) {
                self.dir.push('S');
                self.go((i, j + 1), weight, oper);
                self.dir.pop();
            }
        } // else visiting the last room is only allowed once, so stop the search

        self.txt.truncate(old_txt_len);
    }
}
