use crate::ast::symbols as A;
use super::symbols as Z;

pub fn parse(program: A::Program) -> Z::Program {
    let mut result = Z::Program::new();

    for dec in program {
        match dec {
            A::Declaration::Function(def) => {
                let mut t = FunctionTranspiler::new(def.name);
                for s in def.statements {
                    t.parse_statement(s);
                }

                result.push(Z::Construct::from_transpiler(t));
            }
        }
    }

    return result;
}


struct FunctionTranspiler {
    instructions: Vec<Z::Instruction>,
    f_name: String,
    tmp_count: usize
}

impl FunctionTranspiler {
    fn new(name: String) -> Self {
        Self {
            instructions: vec![],
            f_name: name,
            tmp_count: 0
        }
    }

    fn make_temporary(&mut self) -> String {
        let name = format!("fn.{}.{}", self.f_name, self.tmp_count);
        self.tmp_count += 1;
        return name;
    }

    fn parse_statement(&mut self, s: A::Statement){
        match s {
            A::Statement::Return(exp) => {
                let value = self.parse_value(exp);
                self.instructions.push(Z::Instruction::Return(value));
            }
        }
    }

    fn parse_value(&mut self, e: A::Expression) -> Z::Value {
        match e {
            A::Expression::Constant(c) => Z::Value::Constant(c.inner().clone()),
            A::Expression::Unary(op, inner) => {
                let src = self.parse_value(*inner);
                let dst = Z::Value::Variable(self.make_temporary());
                let op = Self::convert_unop(op);
                
                let im = Z::Instruction::Unary(Z::UnaryInstructionDefinition {
                    operator: op,
                    source: src,
                    destination: dst.clone(),
                });
                self.instructions.push(im);

                return dst;
            }
        }
    }

    fn convert_unop(op: A::UnaryExpressionType) -> Z::UnaryInstructionOperator {
        match op {
            A::UnaryExpressionType::Complement => Z::UnaryInstructionOperator::Complement,
            A::UnaryExpressionType::Negation => Z::UnaryInstructionOperator::Negate,
        }
    }
}

impl Z::Construct {
    fn from_transpiler(t: FunctionTranspiler) -> Self {
        Self::Function(
            Z::FunctionDefinition {
                identifier: t.f_name,
                instructions: t.instructions
            }
        )
    }
}
