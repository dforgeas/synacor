use std::fs::File;
use std::io::Read;

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
    Data(u16)
}

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
            y => Cell::Data(y)
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
            Cell::Data(x)=>*x
        }
    }
}

const MAX: u16 = 0x7fff;
const MEM_SIZE: usize = MAX as usize + 1;

struct Program {
    memory: [u16; MEM_SIZE],
    regs: [u16; 8],
    stack: Vec<u16>,
}

fn load_state(program: &mut Program) -> Result<u16, std::io::Error> {
    Err(std::io::Error::last_os_error()) // TODO, replace dummy error with real code
}

fn run(program: &mut Program, pc: u16) {
}

fn main() -> Result<(), std::io::Error> {
    let mut program = Program {
        memory: [0u16; MEM_SIZE],
        regs: [0u16; 8],
        stack: Vec::new(),
    };
    let loaded = load_state(&mut program);
    let pc = match loaded {
        Ok(p) => p,
        Err(_) => {
            let mut challenge = File::open("challenge.bin")?;
            // TODO fill memory with the challenge file
            let mut binary = Vec::new();
            challenge.read_to_end(&mut binary)?;
            
            0u16 // pc
        }
    };
    run(&mut program, pc);
    Ok(())
}
