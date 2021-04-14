use std::hash::{Hasher, BuildHasher};
use std::sync::mpsc;
use std::io::prelude::*;
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

struct MemAck {
    mem: std::collections::HashMap<(u16, u16), u16, SimpleHashBuilder>,
    x: u16,
}
impl MemAck {
    fn new() -> MemAck {
        MemAck{ mem: Default::default(), x: 0 }
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
    let mut threads = Vec::with_capacity(NUM_THREADS as usize);
    let (tx, rx) = mpsc::channel();
    // create the thread that reads rx and sorts integers in a priority queue
    // don't add it to threads, instead join it last, because by itself it would not know when to stop yet
    let output_thread = std::thread::spawn(move || {
        // lock it once because nothing else uses them here
        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();
        let mut heap = std::collections::BinaryHeap::new();
        let mut expected = 0;
        for (y, b) in rx.iter() {
            heap.push(std::cmp::Reverse((y, b)));
            while let Some(std::cmp::Reverse((x, a))) = heap.peek() {
                if *x != expected {
                    break;
                }
                writeln!(stdout, "{} -> {}{}", *x, *a, if *a == 6 {" OK"} else { "" }).unwrap();
                heap.pop();
                expected += 1;
            }
        }
    });
    for i in 0..NUM_THREADS {
        let tx = tx.clone();
        threads.push(std::thread::Builder::new()
                     .stack_size(1 << 24).spawn(move || {
            let mut mem_ack = MemAck::new();
            let mut x = i;
            while x < 0x8000 {
                mem_ack.reset(x);
                let a = mem_ack.ack(4, 1);
                // println!("{} -> {}{}", x, a, if a == 6 {" OK"} else { "" });
                tx.send((x, a)).unwrap();
                x += NUM_THREADS;
            }
        }).unwrap());
    }
    for t in threads {
        t.join().unwrap();
    }
    // send the <end> signal to tx, the clones are destroyed when the threads terminate
    drop(tx);
    // and join the receiver thread
    output_thread.join().unwrap();
}
