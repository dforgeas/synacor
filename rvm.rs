use std::fs::File;
use std::io::{stdout, stdin, Read, Write, ErrorKind};
use std::convert::TryInto;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

    fn encode(&self) -> u16
    {
        match self
        {
            Cell::Halt=>0 ,
            Cell::Set=>1 ,
            Cell::Push=>2 ,
            Cell::Pop=>3 ,
            Cell::Eq=>4 ,
            Cell::Gt=>5 ,
            Cell::Jmp=>6 ,
            Cell::Jt=>7 ,
            Cell::Jf=>8 ,
            Cell::Add=>9 ,
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
}

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
}

impl Program {
    fn load_challenge(&mut self) -> std::io::Result<u16> {
        let mut challenge = File::open("challenge.bin")?;
        let mut binary = Vec::new();
        challenge.read_to_end(&mut binary)?;
        for (i, (a, b)) in PairIter::new(binary.iter()).enumerate() {
            self.memory[i] = Cell::decode(*a as u16 | (*b as u16) << 8);
        }

        Ok(0u16) // pc
    }
    fn load_state(&mut self) -> std::io::Result<u16> {
        let mut savestate = File::open(SAVESTATE_BIN)?;
        let mut b = [0u8; 2];
        for r in self.memory.iter_mut().chain(self.regs.iter_mut()) {
            savestate.read_exact(&mut b)?;
            *r = Cell::decode(b[0] as u16 | (b[1] as u16) << 8);
        }
        savestate.read_exact(&mut b)?;
        let pc = b[0] as u16 | (b[1] as u16) << 8;
        loop {
            match savestate.read_exact(&mut b) {
                Ok(_) => self.stack.push(Cell::decode(b[0] as u16 | (b[1] as u16) << 8)),
                Err(e) => {
                    if e.kind() == ErrorKind::UnexpectedEof {
                        // we found the end of the stack
                        return Ok(pc)
                    } else {
                        // something else which we just pass on
                        return Err(e)
                    }
                },
            }
        }
    }
    fn save_state(&mut self, pc: u16) -> std::io::Result<()> {
        let mut savestate = File::create(SAVESTATE_BIN)?;
        for x in self.memory.iter().chain(self.regs.iter()).map(Cell::encode) {
            savestate.write_all(&[x as u8, (x >> 8) as u8])?;
        }
        savestate.write_all(&[pc as u8, (pc >> 8) as u8])?;
        for x in self.stack.iter().map(Cell::encode) {
            savestate.write_all(&[x as u8, (x >> 8) as u8])?;
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

    fn run(&mut self, pc: u16) {
        let mut pc = pc;
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
                    let a = self.next(&mut pc);
                    let b = self.rreg(self.next(&mut pc));
                    let c = self.rreg(self.next(&mut pc));
                    self.wreg(a, Cell::decode((b == c).into()));
                },
                Cell::Gt => {
                    let a = self.next(&mut pc);
                    let b = self.rreg(self.next(&mut pc)).encode();
                    let c = self.rreg(self.next(&mut pc)).encode();
                    self.wreg(a, Cell::decode((b > c).into()));
                },
                Cell::Jmp => {
                    let a = self.rreg(self.next(&mut pc)).encode();
                    pc = a;
                },
                Cell::Jt => {
                    let a = self.rreg(self.next(&mut pc));
                    let b = self.next(&mut pc);
                    if a != Cell::Halt {
                        pc = self.rreg(b).encode();
                    }
                },
                Cell::Jf => {
                    let a = self.rreg(self.next(&mut pc));
                    let b = self.next(&mut pc);
                    if a == Cell::Halt {
                        pc = self.rreg(b).encode();
                    }
                },
                Cell::Add => {
                    let a = self.next(&mut pc);
                    let b = self.rreg(self.next(&mut pc)).encode();
                    let c = self.rreg(self.next(&mut pc)).encode();
                    self.wreg(a, Cell::decode(b + c & MAX));
                },
                Cell::Mult => {
                    let a = self.next(&mut pc);
                    let b = self.rreg(self.next(&mut pc)).encode();
                    let c = self.rreg(self.next(&mut pc)).encode();
                    self.wreg(a, Cell::decode(b * c & MAX));
                },
                Cell::Mod => {
                    let a = self.next(&mut pc);
                    let b = self.rreg(self.next(&mut pc)).encode();
                    let c = self.rreg(self.next(&mut pc)).encode();
                    self.wreg(a, Cell::decode(b % c));
                },
                Cell::And => {
                    let a = self.next(&mut pc);
                    let b = self.rreg(self.next(&mut pc)).encode();
                    let c = self.rreg(self.next(&mut pc)).encode();
                    self.wreg(a, Cell::decode(b & c));
                },
                Cell::Or => {
                    let a = self.next(&mut pc);
                    let b = self.rreg(self.next(&mut pc)).encode();
                    let c = self.rreg(self.next(&mut pc)).encode();
                    self.wreg(a, Cell::decode(b | c));
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
                    stdout().write(&[a.encode().try_into().unwrap()]).unwrap();
                }
                Cell::In => {
                    let mut b = [0u8];
                    if 0 == stdin().read(&mut b).unwrap() {
                        self.save_state(pc - 1).unwrap(); // restart the In
                        break;
                    }
                    if b[0] == b'\r' {
                        stdin().read(&mut b).unwrap(); // unlikely this could be EOF here
                    }
                    self.wreg(self.next(&mut pc), Cell::decode(b[0].into()));
                }
                Cell::Noop => continue,
                _ => panic!("Invalid instruction: {:?} at {:x}!", c, pc-1),
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut program = Program {
        memory: [Cell::Halt; MEM_SIZE],
        regs: [Cell::Halt; 8],
        stack: Vec::new(),
    };
    let loaded = program.load_state();
    let pc = match loaded {
        Ok(p) => p,
        Err(_) => program.load_challenge()?
    };
    program.run(pc);
    Ok(())
}
