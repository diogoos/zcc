#[allow(dead_code)]
enum ASTNodeType { Program, Statement, Expression }

#[allow(dead_code)]
trait ASTNode {
    fn node_type() -> ASTNodeType;
    fn debug_node_name(&self) -> String;
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Program<'a> {
    Function(FunctionDefinition<'a>),
    // GlobalVariable(VariableDefinition)
}
impl<'a> ASTNode for Program<'a> {
    fn node_type() -> ASTNodeType { ASTNodeType::Program }
    fn debug_node_name(&self) -> String {
        match self {
            Program::Function((name, _)) => format!("Function({})", name)
        }
    }
}

pub type FunctionDefinition<'a> = (&'a str, Vec<Statement>);

#[allow(dead_code)]
#[derive(Debug)]
pub enum Statement {
    Return(Expression),
    // If(Expression, Box<Statement>, Option<Box<Statement>>)
}
impl ASTNode for Statement {
    fn node_type() -> ASTNodeType { ASTNodeType::Statement }
    fn debug_node_name(&self) -> String {
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
impl ASTNode for Expression {
    fn node_type() -> ASTNodeType { ASTNodeType::Expression }
    fn debug_node_name(&self) -> String {
        match self {
            Expression::Int(val) => format!("Int({})", val)
        }
    }
}

#[allow(dead_code)]
pub fn print_program_tree<'a>(tree: &Vec<Program<'a>>) {
    println!("Program root");
    for program in tree {
        println!("   |_ {}", program.debug_node_name());
        match program {
            Program::Function((_, statements)) => {
                for statement in statements {
                    println!("      |_ {}", statement.debug_node_name());
                    match statement {
                        Statement::Return(expression) => {
                            println!("         |_ {}", expression.debug_node_name());
                        },
                    }
                }
            }
        }
    }
}