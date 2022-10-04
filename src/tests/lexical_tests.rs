#[cfg(test)]
mod lexical_tests {
    use crate::{
        lexical::{
            lexical::Lexical,
            models::Tokens::{Token, TokenType},
        },
        tests::make_tests::make_things::assert_between_token,
    };

    #[test]
    fn lexical_test1() {
        let src1 = "(a+b)^100.12==+100-20";

        let tokens = Lexical::analyse(&src1.chars().collect::<Vec<char>>());
        println!("================== geted tokens ==================");
        for tks in &tokens {
            println!("geted token: {:?}", tks);
        }
        println!("================== geted tokens ==================");
        let tk1 = Token{ttype: TokenType::BRACKET, tvalue: "(".to_string()};
        let tk2 = Token{ttype: TokenType::VARIABLE, tvalue: "a".to_string()};
        let tk3 = Token{ttype: TokenType::OPERATOR, tvalue: "+".to_string()};
        let tk4 = Token{ttype: TokenType::VARIABLE, tvalue: "b".to_string()};
        let tk5 = Token{ttype: TokenType::BRACKET, tvalue: ")".to_string()};
        let tk6 = Token{ttype: TokenType::OPERATOR, tvalue: "^".to_string()};
        let tk7 = Token{ttype: TokenType::FLOAT, tvalue: "100.12".to_string()};
        let tk8 = Token{ttype: TokenType::OPERATOR, tvalue: "==".to_string()};
        let tk9 = Token{ttype: TokenType::INTEGER, tvalue: "+100".to_string()};
        let tk10 = Token{ttype: TokenType::OPERATOR, tvalue: "-".to_string()};
        let tk11 = Token{ttype: TokenType::INTEGER, tvalue: "20".to_string()};
        let results: [Token; 11] = [
            tk1,
            tk2,
            tk3,
            tk4,
            tk5,
            tk6,
            tk7,
            tk8,
            tk9,
            tk10,
            tk11,
        ];
        assert!(tokens.len() == results.len());
        for (test_tk, res_tk) in tokens.into_iter().zip(results.into_iter()) {
            println!("test token: {:?}, result token: {:?}", test_tk, res_tk);
            assert_between_token(test_tk, res_tk);
        }
    }
}
