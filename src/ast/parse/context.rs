#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Context {
    Identifier,
    BoolLit,
    StringLit,
    NumLit,
    Expression,
    HexNum,
    OctNum,
    BinNum,
    DecNum,
    Call,
    BinaryOperation,
    Eject,
    Statement,
    Block,
}
