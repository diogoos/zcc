#[allow(dead_code)]
#[derive(Debug)]
pub enum Construct<'a> {
    Program(Box<Construct<'a>>),
    Function(&'a str, Vec<Instruction<'a>>)
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Instruction<'a> {
    Mov(Operand<'a>, Operand<'a>),
    Ret
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Operand<'a> {
    Imm(&'a str),
    Register
}
impl<'a> Operand<'a> {
    fn assembled_value(&self) -> String {
        match self {
            Self::Imm(str) => format!("${}", str),
            Self::Register => format!("%eax")
        }
    }
}

use crate::ast_definitions as c;
pub fn ast_to_assembly<'a>(program: &'a c::Program) -> Construct<'a> {
    match program {
        c::Program::Function((name, statements)) => {
            let instructions = statements_to_instructions(&statements);
            let function = Construct::Function(name, instructions);

            return Construct::Program(Box::new(function));
        }
    }
}

fn statements_to_instructions<'a>(statements: &'a Vec<c::Statement>) -> Vec<Instruction<'a>> {
    let mut instructions = vec![];

    for (idx, statement) in statements.iter().enumerate() {
        match statement {
            c::Statement::Return(return_value) => {
                assert_eq!(idx + 1, statements.len()); // `Return` can only be the last value of a statement list (for now)
                
                let left_operand: Operand = match return_value {
                    c::Expression::Int(str_value) => Operand::Imm(str_value)
                };

                instructions.push(Instruction::Mov(left_operand, Operand::Register));
                instructions.push(Instruction::Ret);
            }
        }
    }    

    return instructions;
}

pub fn generate_assembly_code(construct: Construct) -> String {
    let mut code: String = "".to_string();
    match construct {
        Construct::Program(inner_construct) => {
            code += "	.section	__TEXT,__text,regular,pure_instructions\n";
            code += generate_assembly_code(*inner_construct).as_str();
        },

        Construct::Function(name, instructions) => {
            code += "	.globl _";
            code += name;
            code += "\n";

            code += "_";
            code += name;
            code += ":\n";


            // add instructions
            for i in instructions {
                match i {
                    Instruction::Mov(src, dst) => {
                        code += "	movl	";
                        code += src.assembled_value().as_str();
                        code += ", ";
                        code += dst.assembled_value().as_str();
                        code += "\n";
                    },
                    Instruction::Ret => {
                        code += "	ret";
                    }
                }
            }
        }
    }

    code += "\n";
    return code;
}

