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
    Data(Arg)
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