use std::collections::HashMap;
use crate::zil;
use zil::symbols as Z;
use crate::assembly::symbols::Opd;
use crate::assembly::symbols::Opd::Reg;
use super::symbols as S;

pub struct STranspiler {
    stack_map: HashMap<String, i8>,
    curr_offset: i8
}

impl STranspiler {
    pub fn new() -> STranspiler {
        STranspiler { stack_map: HashMap::new(), curr_offset: 0 }
    }

    pub fn parse(&mut self, program: zil::symbols::Program) -> S::Program {
        let mut new = S::Program::new();


        // First pass - transpile into assembly instructions
        for c in program {
            match c {
                Z::Construct::Function(def) => {
                    let mut parsed = self.parse_instructions(def.instructions);

                    // FIXME: insert stack allocation in the correct place
                    parsed.insert(0, S::Instruction::AllocateStack(self.curr_offset * -1));

                    new.push(S::Function {
                        identifier: def.identifier,
                        instructions: parsed
                    });
                }
            }
        }


        return new;
    }

    fn parse_instructions(&mut self, instructions: Vec<Z::Instruction>) -> Vec<S::Instruction> {
        let mut tp: Vec<S::Instruction> = vec![];

        for i in instructions {
            match i {
                Z::Instruction::Return(val) => {
                    let val = self.parse_value(val);

                    tp.push(S::Instruction::Mov(val, Opd::Reg(S::Reg::AX)));
                    tp.push(S::Instruction::Ret);
                },

                Z::Instruction::Unary(def) => {
                    let src= self.parse_value(def.source);
                    let dst = self.parse_value(def.destination);

                    // FIXME: move this to another compiler pass
                    if let Opd::Stack(_) = &src {
                        if let Opd::Stack(_) = &dst {
                            // When you encounter an invalid mov instruction, rewrite it to first
                            // copy from the source address into R10D and then copy from R10D to
                            // the destination

                            tp.push(S::Instruction::Mov(src, Reg(S::Reg::R10D)));
                            tp.push(S::Instruction::Mov(Reg(S::Reg::R10D), dst.clone()));
                        }
                    } else {
                        tp.push(S::Instruction::Mov(src, dst.clone()));
                    }

                    tp.push(S::Instruction::Unary(def.operator.into(), dst));
                }
            }
        }

        tp
    }

    fn parse_value(&mut self, value: Z::Value) -> S::Opd {
        match value {
            Z::Value::Constant(s) => S::Opd::Imm(s),
            Z::Value::Variable(id) => {
                let offset = self.stack_map.get(&id);
                match offset {
                    None => {
                        self.curr_offset -= 4;
                        self.stack_map.insert(id.clone(), self.curr_offset);

                        S::Opd::Stack(self.curr_offset)
                    }
                    Some(offset) => {
                        S::Opd::Stack(*offset)
                    }
                }
            }
        }
    }
}
