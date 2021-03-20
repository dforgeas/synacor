use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hasher, BuildHasher};
struct SimpleHasher {
    h: usize
}
impl Hasher for SimpleHasher {
    fn write(&mut self, bytes: &[u8]) {
        for x in bytes.iter() {
            self.h = (self.h << 7) ^ (*x as usize).wrapping_mul(0x10001);
        }
    }
    fn finish(&self) -> u64 {
        self.h as u64
    }
}
#[derive(Default)]
struct SimpleHashBuilder{}
impl BuildHasher for SimpleHashBuilder {
    type Hasher = SimpleHasher;
    fn build_hasher(&self) -> Self::Hasher {
        SimpleHasher{h: 0}
    }
}
fn main() {
    let coins = [("red", 2), ("corroded", 3), ("shiny", 5), ("concave", 7), ("blue", 9)]
        .iter().cloned().collect::<HashMap<_,isize,SimpleHashBuilder>>();
    let mut seen = HashSet::with_capacity_and_hasher(coins.len(), SimpleHashBuilder{});
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
