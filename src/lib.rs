mod Lexer;


#[cfg(test)]
mod tests {
    use super::Lexer;

    #[test]
    fn pop_char_removes_and_returns_next_char() {
        let source = String::from("let num = 5;");
        let mut lexer = Lexer::Lexer::new(&source);
        let answers = vec!['l', 'e', 't', ' ', 'n', 'u', 'm', ' ', '=', ' ', '5', ';'];
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
                    let word = lexer.collect_str(character);
                    assert_eq!(word, answers[i]);
                    i += 1;
                },
                false => {
                    match character {
                        ' ' => (),
                        _ => {
                            assert_eq!(character, '!');
                            assert_eq!(lexer.source.as_str(), "");
                        }
                    }
                }
            }
        }
    }
}
