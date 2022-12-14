use std::io::BufRead;
fn main() {
    for line in std::io::stdin().lock().lines() {
        let code = line.unwrap().chars().rev().map(|x| match x {
            'q' => 'p',
            'p' => 'q',
            _ => x
        }).collect::<String>();
        println!("{}", code);
    }
}
