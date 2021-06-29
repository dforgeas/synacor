use std::fs::{self, File};
use std::io::{stdout, stdin, Read, Write, BufRead, BufReader, BufWriter};
use std::convert::TryInto;
use std::slice;

enum _Arg
{
    Val(u16),
    Reg(u16)
}

enum _Instr
{
    Halt,
    Set(_Arg, _Arg),
    Push(_Arg),
    Pop(_Arg),
    Eq(_Arg, _Arg, _Arg),
    Gt(_Arg, _Arg, _Arg),
    Jmp(_Arg),
    Jt(_Arg, _Arg),
    Jf(_Arg, _Arg),
    Add(_Arg, _Arg, _Arg),
    Mult(_Arg, _Arg, _Arg),
    Mod(_Arg, _Arg, _Arg),
    And(_Arg, _Arg, _Arg),
    Or(_Arg, _Arg, _Arg),
    Not(_Arg, _Arg, _Arg),
    Rmem(_Arg, _Arg),
    Wmem(_Arg, _Arg),
    Call(_Arg),
    Ret,
    Out(_Arg),
    In(_Arg),
    Noop,
    Data(u16)
}

// This is not intended for the highest performance,
// rather it's about adding more type safety with natural
// features of Rust. Every Cell is then 32 bits instead of 16.
// Small benefits are that the register indices are precomputed,
// and the values are in native endianness.
#[derive(Clone, Copy, Debug)]
enum Cell
{
    Halt,
    Set,
    Push,
    Pop,
    Eq,
    Gt,
    Jmp,
    Jt,
    Jf,
    Add,
    Mult,
    Mod,
    And,
    Or,
    Not,
    Rmem,
    Wmem,
    Call,
    Ret,
    Out,
    In,
    Noop,
    Data(u16),
    Reg(u16),
}

const MAX: u16 = 0x7fff;
const REG_START: u16 = MAX + 1;
const REG_END: u16 = MAX + 8;

impl Cell
{
    fn decode(x: u16) -> Cell
    {
        match x
        {
            0 => Cell::Halt,
            1 => Cell::Set,
            2 => Cell::Push,
            3 => Cell::Pop,
            4 => Cell::Eq,
            5 => Cell::Gt,
            6 => Cell::Jmp,
            7 => Cell::Jt,
            8 => Cell::Jf,
            9 => Cell::Add,
            10=> Cell::Mult,
            11=> Cell::Mod,
            12=> Cell::And,
            13=> Cell::Or,
            14=> Cell::Not,
            15=> Cell::Rmem,
            16=> Cell::Wmem,
            17=> Cell::Call,
            18=> Cell::Ret,
            19=> Cell::Out,
            20=> Cell::In,
            21=> Cell::Noop,
            22..=MAX => Cell::Data(x),
            REG_START..=REG_END => Cell::Reg(x - REG_START),
            x => panic!("Cannot decode value: {}", x),
        }
    }

    fn decode_from_bytes(b: [u8; 2]) -> Cell {
        Cell::decode(u16::from_le_bytes(b))
    }

    fn encode(&self) -> u16
    {
        match self
        {
            Cell::Halt=>0,
            Cell::Set=>1,
            Cell::Push=>2,
            Cell::Pop=>3,
            Cell::Eq=>4,
            Cell::Gt=>5,
            Cell::Jmp=>6,
            Cell::Jt=>7,
            Cell::Jf=>8,
            Cell::Add=>9,
            Cell::Mult=>10,
            Cell::Mod=>11,
            Cell::And=>12,
            Cell::Or=>13,
            Cell::Not=>14,
            Cell::Rmem=>15,
            Cell::Wmem=>16,
            Cell::Call=>17,
            Cell::Ret=>18,
            Cell::Out=>19,
            Cell::In=>20,
            Cell::Noop=>21,
            Cell::Data(x) => *x,
            Cell::Reg(x) => *x + REG_START,
        }
    }

    fn encode_to_bytes(&self) -> [u8; 2] {
        self.encode().to_le_bytes()
    }
}

// This is similar to .chunk(2), but returning a tuple
// and getting used to making our own Iterator and generic structs.
pub struct PairIter<I: Iterator> {
    i: I
}

impl<I: Iterator> PairIter<I> {
    pub fn new(i: I) -> PairIter<I> {
        PairIter{i}
    }
}

impl<I: Iterator> Iterator for PairIter<I> {
    type Item = (I::Item, I::Item);
    fn next(&mut self) -> Option<Self::Item> {
        let a = self.i.next();
        let b = self.i.next();
        match (a, b) {
            (Some(x), Some(y)) => Some((x, y)),
            _ => None
        }
    }
}

const MEM_SIZE: usize = MAX as usize + 1;
const SAVESTATE_BIN: &str = "savestate.bin";

struct Program {
    memory: [Cell; MEM_SIZE],
    regs: [Cell; 8],
    stack: Vec<Cell>,
    seen_new_line: bool,
}

impl Program {
    fn load_challenge(&mut self, filename: &str) -> std::io::Result<u16> {
        let binary = fs::read(filename)?;
        for (i, (a, b)) in PairIter::new(binary.iter()).enumerate() {
            self.memory[i] = Cell::decode_from_bytes([*a, *b]);
        }

        Ok(0u16) // pc
    }
    fn load_state(&mut self) -> std::io::Result<u16> {
        let mut savestate = BufReader::new(File::open(SAVESTATE_BIN)?);
        let mut b = [0u8; 2];
        for r in self.memory.iter_mut().chain(self.regs.iter_mut()) {
            savestate.read_exact(&mut b)?;
            *r = Cell::decode_from_bytes(b);
        }
        savestate.read_exact(&mut b)?;
        let pc = u16::from_le_bytes(b);
        let mut binary = Vec::new();
        savestate.read_to_end(&mut binary)?;
        for (a, b) in PairIter::new(binary.iter()) {
            self.stack.push(Cell::decode_from_bytes([*a, *b]));
        }
        Ok(pc)
    }
    fn save_state(&mut self, pc: u16) -> std::io::Result<()> {
        let mut savestate = BufWriter::new(File::create(SAVESTATE_BIN)?);
        for b in self.memory.iter().chain(self.regs.iter()).map(Cell::encode_to_bytes) {
            savestate.write_all(&b)?;
        }
        savestate.write_all(&pc.to_le_bytes())?;
        for b in self.stack.iter().map(Cell::encode_to_bytes) {
            savestate.write_all(&b)?;
        }
        Ok(())
    }

    fn next(&self, pc: &mut u16) -> Cell {
        let x = self.memory[*pc as usize];
        *pc += 1;
        x
    }
    fn rreg(&self, a: Cell) -> Cell {
        match a {
            Cell::Reg(r) => self.regs[r as usize],
            _ => a,
        }
    }
    fn wreg(&mut self, a: Cell, b: Cell) {
        if let Cell::Reg(r) = a {
            self.regs[r as usize] = b
        }
    }
    fn ternary<O>(&mut self, pc: &mut u16, oper: O)
        where O: Fn(u16, u16) -> u16
    {
        let a = self.next(pc);
        let b = self.rreg(self.next(pc)).encode();
        let c = self.rreg(self.next(pc)).encode();
        self.wreg(a, Cell::decode(oper(b, c)));
    }

    fn run(&mut self, mut pc: u16) {
        // lock them once because nothing else uses them here
        let stdin = stdin();
        let stdout = stdout();
        let mut stdin = stdin.lock();
        let mut stdout = stdout.lock();
        loop {
            let c = self.next(&mut pc);
            match c {
                Cell::Halt => break,
                Cell::Set => {
                    let a = self.next(&mut pc);
                    let b = self.next(&mut pc);
                    self.wreg(a, self.rreg(b));
                },
                Cell::Push => {
                    let a = self.next(&mut pc);
                    self.stack.push(self.rreg(a));
                },
                Cell::Pop => {
                    let a = self.next(&mut pc);
                    let top = self.stack.pop().unwrap();
                    self.wreg(a, top);
                },
                Cell::Eq => {
                    self.ternary(&mut pc, |b, c| (b == c).into());
                },
                Cell::Gt => {
                    self.ternary(&mut pc, |b, c| (b > c).into());
                },
                Cell::Jmp => {
                    let a = self.rreg(self.next(&mut pc)).encode();
                    pc = a;
                },
                Cell::Jt => {
                    let a = self.rreg(self.next(&mut pc));
                    let b = self.next(&mut pc);
                    if let Cell::Halt = a {
                    } else {
                        pc = self.rreg(b).encode();
                    }
                },
                Cell::Jf => {
                    let a = self.rreg(self.next(&mut pc));
                    let b = self.next(&mut pc);
                    if let Cell::Halt = a {
                        pc = self.rreg(b).encode();
                    }
                },
                Cell::Add => {
                    self.ternary(&mut pc, |b, c| b + c & MAX);
                },
                Cell::Mult => {
                    self.ternary(&mut pc, |b, c| b * c & MAX);
                },
                Cell::Mod => {
                    self.ternary(&mut pc, |b, c| b % c);
                },
                Cell::And => {
                    self.ternary(&mut pc, |b, c| b & c);
                },
                Cell::Or => {
                    self.ternary(&mut pc, |b, c| b | c);
                },
                Cell::Not => {
                    let a = self.next(&mut pc);
                    let b = self.rreg(self.next(&mut pc)).encode();
                    self.wreg(a, Cell::decode(! b & MAX));
                },
                Cell::Rmem => {
                    let a = self.next(&mut pc);
                    let b = self.rreg(self.next(&mut pc)).encode();
                    self.wreg(a, self.memory[b as usize]);
                },
                Cell::Wmem => {
                    let a = self.rreg(self.next(&mut pc)).encode();
                    let b = self.rreg(self.next(&mut pc));
                    self.memory[a as usize] = b;
                },
                Cell::Call => {
                    let a = self.rreg(self.next(&mut pc)).encode();
                    self.stack.push(Cell::decode(pc));
                    pc = a;
                },
                Cell::Ret => {
                    if let Some(c) = self.stack.pop() {
                        pc = c.encode();
                    } else {
                        break;
                    }
                },
                Cell::Out => {
                    let a = self.rreg(self.next(&mut pc));
                    stdout.write(slice::from_ref(&a.encode().try_into().unwrap())).unwrap();
                },
                Cell::In => {
                    let mut b = 0;
                    if 0 == stdin.read(slice::from_mut(&mut b)).unwrap() {
                        self.save_state(pc - 1).unwrap(); // restart the In
                        break;
                    }
                    if b == b'\r' {
                        stdin.read(slice::from_mut(&mut b)).unwrap(); // unlikely this could be EOF here
                    }
                    if self.seen_new_line && b == b'!' {
                        let mut line = String::new();
                        let _bytes_read = stdin.read_line(&mut line).unwrap();
                        if line.trim() == "prepare teleporter" {
	// set 6 into the first register instead, the expected result from ack
                            self.memory[5483 + 2] = Cell::decode(6);
	// replace the call to ack with noops
                            self.memory[5489] = Cell::Noop;
                            self.memory[5489 + 1] = Cell::Noop;
	// set the eigth register to the correct teleporter value
                            self.regs[7] = Cell::decode(25734);
                            write!(stdout, "\nPrepared.\n").unwrap();
                        }
                        b = b'\n'; // hide this line to the program
                    }
                    self.seen_new_line = b == b'\n';
                    self.wreg(self.next(&mut pc), Cell::decode(b.into()));
                },
                Cell::Noop => continue,
                _ => panic!("Invalid instruction: {:?} at {:x}!", c, pc-1),
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mut program = Program {
        memory: [Cell::Halt; MEM_SIZE],
        regs: [Cell::Halt; 8],
        stack: Vec::with_capacity(16),
        seen_new_line: false,
    };
    match args.len() {
        // no arguments passed
        1 => {
            let loaded = program.load_state();
            let pc = match loaded {
                Ok(p) => p,
                Err(_) => program.load_challenge("challenge.bin")?
            };
            program.run(pc);
        },
        // one argument passed
        2 => {
            let pc = program.load_challenge(&args[1])?;
            program.run(pc);
        },
        // all the other cases
        _ => panic!("Unexpected arguments, only pass a program name, or nothing to resume from the saved state."),
    }
    Ok(())
}
