enum _Arg
{
    Val(u16),
    Reg(u16)
}

enum _Instr
{
    Halt,
    Set(Arg, Arg),
    Push(Arg),
    Pop(Arg),
    Eq(Arg, Arg, Arg),
    Gt(Arg, Arg, Arg),
    Jmp(Arg),
    Jt(Arg, Arg),
    Jf(Arg, Arg),
    Add(Arg, Arg, Arg),
    Mult(Arg, Arg, Arg),
    Mod(Arg, Arg, Arg),
    And(Arg, Arg, Arg),
    Or(Arg, Arg, Arg),
    Not(Arg, Arg, Arg),
    Rmem(Arg, Arg),
    Wmem(Arg, Arg),
    Call(Arg),
    Ret,
    Out(Arg),
    In(Arg),
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
    fn decode(x: u16)
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

    fn encode(&self)
    {
        match self
        {
        }
    }
}
