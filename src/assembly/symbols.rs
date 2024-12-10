
use crate::zil::symbols as Z;

pub type Program = Vec<Function>;

#[derive(Debug)]
pub struct Function {
    pub identifier: String,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub enum Instruction {
    Mov(Opd, Opd),
    Unary(UnaryOp, Opd),
    AllocateStack(i8),
    Ret
}

#[derive(Debug)]
pub enum UnaryOp {
    Not,
    Neg
}

impl From<Z::UnaryInstructionOperator> for UnaryOp {
    fn from(op: Z::UnaryInstructionOperator) -> Self {
        match op {
            Z::UnaryInstructionOperator::Negate => Self::Neg,
            Z::UnaryInstructionOperator::Complement => Self::Not
        }
    }
}

#[derive(Debug, Clone)]
pub enum Opd { // Operand
    Imm(String),
    Reg(Reg),
    Stack(i8),
}

#[derive(Debug, Clone)]
pub enum Reg {
    AX,
    R10D
}

impl Reg {
    pub fn operand(&self) -> String {
        match self {
            Reg::AX => "%eax".into(),
            Reg::R10D => "%r10d".into(),
        }
    }
}

