use std::fs::File;
use std::io::{stdout, stdin, Read, Write};
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
            REG_START..=REG_END => Cell::Reg(x - REG_START),
            _ => Cell::Data(x),
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

struct Program {
    memory: [Cell; MEM_SIZE],
    regs: [Cell; 8],
    stack: Vec<u16>,
}

impl Program {
    fn load_challenge(&mut self) -> Result<u16, std::io::Error> {
        let mut challenge = File::open("challenge.bin")?;
        let mut binary = Vec::new();
        challenge.read_to_end(&mut binary)?;
        for (i, (a, b)) in PairIter::new(binary.iter()).enumerate() {
            self.memory[i] = Cell::decode(*a as u16 | (*b as u16) << 8);
        }

        Ok(0u16) // pc
    }
    fn load_state(&mut self) -> Result<u16, std::io::Error> {
        Err(std::io::Error::last_os_error()) // TODO, replace dummy error with real code
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
                Cell::Set => continue,
                Cell::Push => continue,
                Cell::Pop => continue,
                Cell::Eq => continue,
                Cell::Gt => continue,
                Cell::Jmp => continue,
                Cell::Jt => continue,
                Cell::Jf => continue,
                Cell::Add => continue,
                Cell::Mult => continue,
                Cell::Mod => continue,
                Cell::And => continue,
                Cell::Or => continue,
                Cell::Not => continue,
                Cell::Rmem => continue,
                Cell::Wmem => continue,
                Cell::Call => continue,
                Cell::Ret => continue,
                Cell::Out => {
                    let a = self.rreg(self.next(&mut pc));
                    stdout().write(&[a.encode().try_into().unwrap()]).unwrap();
                }
                Cell::In => continue,
                Cell::Noop => continue,
                _ => panic!("Invalid instruction: {:?} at {:x}!", c, pc-1),
            }
        }
    }
}

fn main() -> Result<(), std::io::Error> {
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
