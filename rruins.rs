use std::collections::HashMap;
use std::collections::HashSet;
fn main() {
    let coins = [("red", 2), ("corroded", 3), ("shiny", 5), ("concave", 7), ("blue", 9)]
        .iter().cloned().collect::<HashMap<_,isize>>();
    let mut seen = HashSet::with_capacity(coins.len());
    for (name_a, value_a) in coins.iter() {
        seen.insert(value_a);
        for (name_b, value_b) in coins.iter() {
            if seen.contains(value_b) { continue }
            seen.insert(value_b);
            for (name_c, value_c) in coins.iter() {
                if seen.contains(value_c) { continue }
                seen.insert(value_c);
                for (name_d, value_d) in coins.iter() {
                    if seen.contains(value_d) { continue }
                    seen.insert(value_d);
                    for (name_e, value_e) in coins.iter() {
                        if seen.contains(value_e) { continue }
                        if value_a + value_b * (*value_c).pow(2) + (*value_d).pow(3) - value_e == 399 {
                            println!("{} {} {} {} {}", name_a, name_b, name_c, name_d, name_e);
                        }
                    }
                    seen.remove(value_d);
                }
                seen.remove(value_c);
            }
            seen.remove(value_b);
        }
        seen.remove(value_a);
    }
}
