use std::io::BufRead;
fn main() {
    for line in std::io::stdin().lock().lines() {
        for c in line.unwrap().chars().rev().map(|x| match x {
            'q' => 'p',
            'p' => 'q',
            _ => x
        }) {
            print!("{}", c);
        }
        println!("");
    }
}
