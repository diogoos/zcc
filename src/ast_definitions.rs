#[allow(dead_code)]
#[derive(Debug)]
pub enum Program<'a> {
    Function(FunctionDefinition<'a>),
    // Variable(VariableDefinition)
}
#[allow(dead_code)]
impl<'a> Program<'a> {
    fn node_name(&self) -> String {
        match self {
            Program::Function((name, _)) => format!("Function({})", name)
        }
    }
}

pub type FunctionDefinition<'a> = (&'a str, Vec<Statement>);

#[allow(dead_code)]
#[derive(Debug)]
pub enum Statement {
    Return(Option<Expression>),
    // If(Expression, Box<Statement>, Option<Box<Statement>>)
}
#[allow(dead_code)]
impl Statement {
    fn node_name(&self) -> String {
        match self {
            Statement::Return(_) => format!("Return")
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Expression {
    Int(String)
}
#[allow(dead_code)]
impl Expression {
    fn node_name(&self) -> String {
        match self {
            Expression::Int(val) => format!("Int({})", val)
        }
    }
}

#[allow(dead_code)]
pub fn print_program_tree<'a>(tree: &Vec<Program<'a>>) {
    println!("Program root");
    for program in tree {
        println!("   |_ {}", program.node_name());
        match program {
            Program::Function((_, statements)) => {
                for statement in statements {
                    println!("      |_ {}", statement.node_name());
                    match statement {
                        Statement::Return(Some(expression)) => {
                            println!("         |_ {}", expression.node_name());
                        },
                        Statement::Return(None) => {}
                    }
                }
            }
        }
    }
}