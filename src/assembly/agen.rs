use super::symbols as S;

pub fn codegen(program: &S::Program) -> String {
    let mut gen = String::new();

    // FIXME: Allow more than one function
    assert_eq!(program.len(), 1);
    let program = &program[0];

    // FIXME: fix this too


    // Function definition header
    gen += "\t.globl _";
    gen += program.identifier.as_str();
    gen += "\n_";
    gen += program.identifier.as_str();
    gen += ":\n\tpushq\t%rbp\n\tmovq\t%rsp, %rbp\n";


    for i in &program.instructions {
        gen += "\t";
        gen += gen_instruction(i).as_str();
        gen += "\n";
    }

    gen
}

fn gen_instruction(i: &S::Instruction) -> String {
    match i {
        S::Instruction::Mov(src, dst) => {
            let mut instruction = String::new();
            instruction += "movl\t";
            instruction += gen_op(src).as_str();
            instruction += ", ";
            instruction += gen_op(dst).as_str();

            instruction
        },

        S::Instruction::Ret => {
            "movq\t%rbp, %rsp\n\tpopq\t%rbp\n\tret".to_string()
        },

        S::Instruction::Unary(op, dst) => {
            let mut instruction = String::new();

            instruction += match op {
                S::UnaryOp::Neg => "negl\t",
                S::UnaryOp::Not => "notl\t"
            };

            instruction += gen_op(dst).as_str();
            instruction
        },

        S::Instruction::AllocateStack(size) => {
            "subq\t$".to_string() + &size.to_string() + ", %rsp"
        }
    }
}

fn gen_op(v: &S::Opd) -> String {
    match v {
        S::Opd::Reg(r) => { r.operand() },
        S::Opd::Imm(imm) => { "$".to_string() + imm.as_str() },
        S::Opd::Stack(offset) => { offset.to_string() + "(%rbp)" }
    }
}

