mod Lexer;


#[cfg(test)]
mod tests {
    use super::Lexer;
    use Lexer::Token;

    #[test]
    fn pop_char_removes_and_returns_next_char() {
        let source = String::from("let num = 5;");
        let mut lexer = Lexer::Lexer::new(&source);
        let answers = vec!['l', 'e', 't', 'n', 'u', 'm', '=', '5', ';'];
        let mut i = 0;
        while let Some(value) = lexer.pop_char() {
           assert_eq!(value, answers[i]);
           i += 1;
        }
        assert_eq!(lexer.source.as_str(), "");
        assert_eq!(lexer.pop_char(), None)
    }

    #[test]
    fn is_next_alphanumeric_tests_correctly() {
        let source = String::from("5; r");
        let mut lexer = Lexer::Lexer::new(&source);
        let answers = vec![true, false, false, true];
        let mut i = 0;
        while let Some(is_an) = lexer.is_next_alphanumeric() {
            assert_eq!(is_an, answers[i]);
            lexer.pop_char();
            i += 1;
        }
    }

    #[test]
    fn collect_str_tests_correctly() {
        let source = String::from("robot 80 chickens 4ever!");
        let answers = vec!["robot", "80", "chickens", "4ever"];
        let mut lexer = Lexer::Lexer::new(&source);
        let mut i = 0;
        while let Some(character) = lexer.pop_char() {
            match character.is_alphanumeric() {
                true => {
                    let word = lexer.collect_str(character).unwrap();
                    assert_eq!(word, answers[i]);
                    i += 1;
                },
                false if character == ' ' => (),
                _ => assert_eq!(character, '!')
            }
        }
        assert_eq!(lexer.source.as_str(), "");
    }

    #[test]
    fn next_token_returns_token() {
        let source = String::from("fn dothis(x, y) { return x + y == 6; }; let num = 5; 5 != 6;");
        let mut lexer = Lexer::Lexer::new(&source);
        let answers = vec![
            Token::FUNCTION,
            Token::IDENT("dothis".to_string()),
            Token::LPAREN,
            Token::IDENT("x".to_string()),
            Token::COMMA,
            Token::IDENT("y".to_string()),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT("return".to_string()),
            Token::IDENT("x".to_string()),
            Token::PLUS,
            Token::IDENT("y".to_string()),
            Token::EQ,
            Token::INT(6),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("num".to_string()), 
            Token::ASSIGN,
            Token::INT(5),
            Token::SEMICOLON,
            Token::INT(5),
            Token::NE,
            Token::INT(6),
            Token::SEMICOLON
        ];
        let mut i = 0;
        while let Some(cur_token) = lexer.next_token() {
            assert_eq!(cur_token, answers[i]);
            i += 1;
        }
    }

    // #[test]
    // fn is_next_alphanumeric_tests_correctly() {
    //     let source = String::from("5; r");
    //     let mut lexer = Lexer::Lexer::new(&source);
    //     let answers = vec![true, false, false, true];
    //     let mut i = 0;
    //     while let Some(is_an) = lexer.is_next_alphanumeric() {
    //         assert_eq!(is_an, answers[i]);
    //         lexer.pop_char();
    //         i += 1;
    //     }
    // }
}
