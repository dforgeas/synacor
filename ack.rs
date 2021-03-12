use std::io::prelude::*;

struct MemAck {
    mem: std::collections::HashMap<(u16, u16), u16>,
    x: u16,
}
impl MemAck {
    fn new() -> MemAck {
        MemAck{ mem: std::collections::HashMap::new(), x: 0 }
    }
    fn reset(&mut self, x: u16) {
        self.mem.clear();
        self.x = x;
    }

    fn ack(&mut self, m: u16, n: u16) -> u16 {
        match self.mem.get(&(m, n)) {
            None => {
                let r;
                if m == 0 { r = (n+1) & 0x7fff; }
                else if n == 0 { r = self.ack(m-1, self.x); }
                else {
                    let t = self.ack(m, n-1);
                    r = self.ack(m-1, t);
                }
                self.mem.insert((m, n), r);
                r
            },
            Some(r) => *r,
        }
    }
}

fn main() {
    let mut mem_ack = MemAck::new();
    for x in 2..0x8000 {
        print!("{}", x); std::io::stdout().flush().unwrap();
        mem_ack.reset(x);
        let a = mem_ack.ack(4, 1);
        println!(" -> {}{}", a, if a == 6 {" OK"} else { "" });
    }
}
