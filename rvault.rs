use std::io::{prelude::*, BufReader};
use std::fs::File;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Num(u16),
    Oper(char),
}

fn main() -> std::io::Result<()> {
    let mut grid = HashMap::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut end_weight = 0;
    let file = BufReader::new(File::open("vault_lock.txt")?);
    for (j, line) in file.lines().enumerate() {
        for (i, field) in line?.split_whitespace().enumerate() {
            match field {
                "|" => continue,
                assign if assign.contains('=') => {
                    let (name, weight) = field.split_at(field.find('=').unwrap());
                    // skip '=' that is returned part of the weight after split_at
                    let weight = (&weight[1..]).parse::<u16>().expect("weight isn't a valid number");
                    println!("weight {}", weight);
                    grid.insert((i, j), Cell::Num(weight));
                    match name {
                        "orb" => start = (i, j),
                        "vault" => { end = (i, j); end_weight = weight },
                        _ => panic!("name {:?} not recognized", name),
                    }
                },
                _ => {
                    let cell = match field.parse::<u16>() {
                        Ok(u) => Cell::Num(u),
                        Err(_) => Cell::Oper(field.chars().next().unwrap()),
                    };
                    grid.insert((i, j), cell);
                }
            }
        }
    }
    Ok(())
}
