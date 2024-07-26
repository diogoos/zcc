mod ast_tests {
    use crate::ast::*;
    use crate::lexer::{Tag, Token};

    #[test]
    fn test_basic_return() {
        let buffer = "int main(void) {\n  return 2;\n}\n".to_string();
        let tokens = vec![
            Token { tag: Tag::KInt, range: 0..3 },
            Token { tag: Tag::Identifier, range: 4..8 },
            Token { tag: Tag::LParen, range: 8..9 },
            Token { tag: Tag::KVoid, range: 9..13 },
            Token { tag: Tag::RParen, range: 13..14 },
            Token { tag: Tag::LBrace, range: 15..16 },
            Token { tag: Tag::KReturn, range: 19..25 },
            Token { tag: Tag::NumberLiteral, range: 26..27 },
            Token { tag: Tag::Semicolon, range: 27..28 },
            Token { tag: Tag::RBrace, range: 29..30 },
            Token { tag: Tag::Eof, range: 31..31 }
        ];

        let mut parser = ASTParser::new(buffer, tokens);
        let ast = parser.parse().expect("Unable to generate AST");

        assert_eq!(ast.len(), 1);
        
        let program = &ast[0];
        match program {
            Program::Function((name, statements)) => {
                assert_eq!(*name, "main");
                assert_eq!(statements.len(), 1);

                match &statements[0] {
                    Statement::Return(exp) => {
                        let exp = exp.as_ref().expect("Return should have expression");
                        match exp {
                            Expression::Int(int_val) => {
                                assert_eq!(*int_val, "2".to_string());
                            }
                            #[allow(unreachable_patterns)]
                            _ => { panic!("Expression should be of type Int"); }
                        }

                    }
                    
                    #[allow(unreachable_patterns)]
                    _ => { panic!("Statement should be of type Return"); }
                }
            },
            
            #[allow(unreachable_patterns)]
            _ => { panic!("AST root node should match Program::Function"); }
        }
    }

    #[test]
    fn test_complex() {
        let buffer = "int custom(void) {\nreturn 500;\nreturn 10;\n}\nvoid two() {\nreturn;\n}".to_string();
        let tokens = vec![
            Token { tag: Tag::KInt, range: 0..3 },
            Token { tag: Tag::Identifier, range: 4..10 },
            Token { tag: Tag::LParen, range: 10..11 },
            Token { tag: Tag::KVoid, range: 11..15 },
            Token { tag: Tag::RParen, range: 15..16 },
            Token { tag: Tag::LBrace, range: 17..18 },
            Token { tag: Tag::KReturn, range: 19..25 },
            Token { tag: Tag::NumberLiteral, range: 26..29 },
            Token { tag: Tag::Semicolon, range: 29..30 },
            Token { tag: Tag::KReturn, range: 31..37 },
            Token { tag: Tag::NumberLiteral, range: 38..40 },
            Token { tag: Tag::Semicolon, range: 40..41 },
            Token { tag: Tag::RBrace, range: 42..43 },
            Token { tag: Tag::KVoid, range: 44..48 },
            Token { tag: Tag::Identifier, range: 49..52 },
            Token { tag: Tag::LParen, range: 52..53 },
            Token { tag: Tag::RParen, range: 53..54 },
            Token { tag: Tag::LBrace, range: 55..56 },
            Token { tag: Tag::KReturn, range: 57..63 },
            Token { tag: Tag::Semicolon, range: 63..64 },
            Token { tag: Tag::RBrace, range: 65..66 },
            Token { tag: Tag::Eof, range: 66..66 },
        ];
        
        let mut parser = ASTParser::new(buffer, tokens);
        let ast = parser.parse().expect("Unable to generate AST");

        assert_eq!(ast.len(), 2);
        
        let program1 = &ast[0];
        match program1 {
            Program::Function((name, statements)) => {
                assert_eq!(*name, "custom");
                assert_eq!(statements.len(), 2);

                match &statements[0] {
                    Statement::Return(exp) => {
                        let exp = exp.as_ref().expect("Return should have expression");
                        match exp {
                            Expression::Int(int_val) => {
                                assert_eq!(*int_val, "500".to_string());
                            }
                            #[allow(unreachable_patterns)]
                            _ => { panic!("Expression should be of type Int"); }
                        }

                    }
                    
                    #[allow(unreachable_patterns)]
                    _ => { panic!("Statement should be of type Return"); }
                }

                match &statements[1] {
                    Statement::Return(Some(exp)) => {
                        match exp {
                            Expression::Int(int_val) => {
                                assert_eq!(*int_val, "10".to_string());
                            }
                            #[allow(unreachable_patterns)]
                            _ => { panic!("Expression should be of type Return(Int)"); }
                        }
                    }
                    
                    _ => { panic!("Statement should be of type Return(Some)"); }
                }
            },
            
            #[allow(unreachable_patterns)]
            _ => { panic!("AST root node 0 should match Program::Function"); }
        }

        let program2 = &ast[1];
        match program2 {
            Program::Function((name, statements)) => {
                assert_eq!(*name, "two");
                assert_eq!(statements.len(), 1);

                match &statements[0] {
                    Statement::Return(None) => {},
                    _ => { panic!("Statement should be of type Return(None)"); }
                }
            },
            
            #[allow(unreachable_patterns)]
            _ => { panic!("AST root node 1 should match Program::Function"); }
        }
        
    }

    #[test]
    fn test_end_before_expr() {
        let buffer = "int main(void) {\nreturn".to_string();
        let tokens = vec![
            Token { tag: Tag::KInt, range: 0..3 },
            Token { tag: Tag::Identifier, range: 4..8 },
            Token { tag: Tag::LParen, range: 8..9 },
            Token { tag: Tag::KVoid, range: 9..13 },
            Token { tag: Tag::RParen, range: 13..14 },
            Token { tag: Tag::LBrace, range: 15..16 },
            Token { tag: Tag::Identifier, range: 17..24 },
        ];

        let mut parser = ASTParser::new(buffer, tokens);
        assert!(parser.parse().is_err());
    }

    #[test]
    fn test_extra_junk() {
        let buffer = "int main(void)\n{\nreturn 2;\n}\nfoo".to_string();
        let tokens = vec![
            Token { tag: Tag::KInt, range: 0..3 },
            Token { tag: Tag::Identifier, range: 4..8 },
            Token { tag: Tag::LParen, range: 8..9 },
            Token { tag: Tag::KVoid, range: 9..13 },
            Token { tag: Tag::RParen, range: 13..14 },
            Token { tag: Tag::LBrace, range: 15..16 },
            Token { tag: Tag::KReturn, range: 17..23 },
            Token { tag: Tag::NumberLiteral, range: 24..25 },
            Token { tag: Tag::Semicolon, range: 25..26 },
            Token { tag: Tag::RBrace, range: 27..28 },
            Token { tag: Tag::Identifier, range: 29..33 },
        ];

        let mut parser = ASTParser::new(buffer, tokens);
        assert!(parser.parse().is_err());
    }

    #[test]
    fn test_no_semicolon() {
        let buffer = "int main(void)\n{\nreturn 2\n}".to_string();
        let tokens = vec![
            Token { tag: Tag::KInt, range: 0..3 },
            Token { tag: Tag::Identifier, range: 4..8 },
            Token { tag: Tag::LParen, range: 8..9 },
            Token { tag: Tag::KVoid, range: 9..13 },
            Token { tag: Tag::RParen, range: 13..14 },
            Token { tag: Tag::LBrace, range: 15..16 },
            Token { tag: Tag::KReturn, range: 17..23 },
            Token { tag: Tag::NumberLiteral, range: 24..25 },
            Token { tag: Tag::RBrace, range: 27..28 },
        ];

        let mut parser = ASTParser::new(buffer, tokens);
        assert!(parser.parse().is_err());
    }
}
