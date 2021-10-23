

enum Inst {
    Match(char),
    MatchRange(char, char),
    If(char, usize, usize),
    IfRange(char, char, usize, usize),
    Split(usize),
    SplitIf(char, usize),
    SplitIfRange(char, char, usize),
    Jmp(usize),
    Begin,
    End,
    Nop,
    Accept,
    Halt,
}