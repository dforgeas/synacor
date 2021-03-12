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

const NUM_THREADS: u16 = 4;

fn main() {
    // TODO: implement the search in multiple threads
    // and set a suitable stack size for them: https://doc.rust-lang.org/std/thread/index.html#stack-size
    // perhaps the default 2MB of stack size Rust allocates will be sufficent?
    // it seems not, I tried with 16MB and it worked
    let mut threads = Vec::with_capacity(NUM_THREADS as usize);
    for i in 0..NUM_THREADS {
        threads.push(std::thread::Builder::new()
                     .stack_size(1 << 24).spawn(move || {
            let mut mem_ack = MemAck::new();
            let mut x = i;
            while x < 0x8000 {
                mem_ack.reset(x);
                let a = mem_ack.ack(4, 1);
                println!("{} -> {}{}", x, a, if a == 6 {" OK"} else { "" });
                x += NUM_THREADS;
            }
        }).unwrap());
    }
    for t in threads {
        t.join().unwrap();
    }
}
