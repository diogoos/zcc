mod lexer_tests {
    use crate::lexer::*;

    #[test]
    fn test_return_2() {
        let mut lexer = Lexer::load_test_str("int main(void) {\n  return 2;\n}\n");

        assert_eq!(lexer.next(), Token { tag: Tag::KInt, range: 0..3 });
        assert_eq!(lexer.next(), Token { tag: Tag::Identifier, range: 4..8 });
        assert_eq!(lexer.next(), Token { tag: Tag::LParen, range: 8..9 });
        assert_eq!(lexer.next(), Token { tag: Tag::KVoid, range: 9..13 });
        assert_eq!(lexer.next(), Token { tag: Tag::RParen, range: 13..14 });
        assert_eq!(lexer.next(), Token { tag: Tag::LBrace, range: 15..16 });
        assert_eq!(lexer.next(), Token { tag: Tag::KReturn, range: 19..25 });
        assert_eq!(lexer.next(), Token { tag: Tag::NumberLiteral, range: 26..27 });
        assert_eq!(lexer.next(), Token { tag: Tag::Semicolon, range: 27..28 });
        assert_eq!(lexer.next(), Token { tag: Tag::RBrace, range: 29..30 });
        assert_eq!(lexer.next(), Token { tag: Tag::Eof, range: 31..31 });
    }

    #[test]
    #[should_panic]
    fn test_lex_out_of_bounds() {
        let mut lexer = Lexer::load_test_str("int");
        
        assert_eq!(lexer.next(), Token { tag: Tag::KInt, range: 0..3 });
        assert_eq!(lexer.next(), Token { tag: Tag::Eof, range: 4..4 });
        lexer.next(); // should panic
    }

    #[test]
    #[should_panic]
    fn test_empty_string() {
        let mut lexer = Lexer::load_test_str("");
        assert_eq!(lexer.next(), Token { tag: Tag::Eof, range: 0..0 });
        lexer.next(); // should panic, for consistency
    }

    #[test]
    fn test_invalid_char() {
        let mut lexer = Lexer::load_test_str("int @ void\0");
        assert_eq!(lexer.next(), Token { tag: Tag::KInt, range: 0..3 });
        assert_eq!(lexer.next(), Token { tag: Tag::Invalid, range: 4..5 });
        assert_eq!(lexer.next(), Token { tag: Tag::KVoid, range: 6..10 });
    }
}
