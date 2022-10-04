#[cfg(test)]
mod lexical_tests {
    use itertools::Itertools;

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

    #[test]
    fn lexical_test2() {
        let source_code = "func add(a, b){\n print(a+b); a *= b;\n}\n add(0124, -20.5, 122, 0x9FFF)";
        let tokens = Lexical::analyse(&source_code.chars().collect_vec());
        println!("================== geted tokens ==================");
        for tks in &tokens {
            println!("geted token: {:?}", tks);
        }
        println!("================== geted tokens ==================");
        let tk1 = Token{ttype: TokenType::KEYWORD, tvalue: "func".to_string()};
        let tk2 = Token{ttype: TokenType::VARIABLE, tvalue: "add".to_string()};
        let tk3 = Token{ttype: TokenType::BRACKET, tvalue: "(".to_string()};
        let tk4 = Token{ttype: TokenType::VARIABLE, tvalue: "a".to_string()};
        let tk5 = Token{ttype: TokenType::OPERATOR, tvalue: ",".to_string()};
        let tk6 = Token{ttype: TokenType::VARIABLE, tvalue: "b".to_string()};
        let tk7 = Token{ttype: TokenType::BRACKET, tvalue: ")".to_string()};
        let tk8 = Token{ttype: TokenType::BRACKET, tvalue: "{".to_string()};
        let tk9 = Token{ttype: TokenType::VARIABLE, tvalue: "print".to_string()};
        let tk10 = Token{ttype: TokenType::BRACKET, tvalue: "(".to_string()};
        let tk11 = Token{ttype: TokenType::VARIABLE, tvalue: "a".to_string()};
        let tk12 = Token{ttype: TokenType::OPERATOR, tvalue: "+".to_string()};
        let tk13 = Token{ttype: TokenType::VARIABLE, tvalue: "b".to_string()};
        let tk14 = Token{ttype: TokenType::BRACKET, tvalue: ")".to_string()};
        let tk15 = Token{ttype: TokenType::OPERATOR, tvalue: ";".to_string()};

        let tk27 = Token{ttype: TokenType::VARIABLE, tvalue: "a".to_string()};
        let tk28 = Token{ttype: TokenType::OPERATOR, tvalue: "*=".to_string()};
        let tk29 = Token{ttype: TokenType::VARIABLE, tvalue: "b".to_string()};
        let tk30 = Token{ttype: TokenType::OPERATOR, tvalue: ";".to_string()};

        let tk16 = Token{ttype: TokenType::BRACKET, tvalue: "}".to_string()};
        let tk17 = Token{ttype: TokenType::VARIABLE, tvalue: "add".to_string()};
        let tk18 = Token{ttype: TokenType::BRACKET, tvalue: "(".to_string()};
        let tk19 = Token{ttype: TokenType::OCT, tvalue: "0124".to_string()};
        let tk20 = Token{ttype: TokenType::OPERATOR, tvalue: ",".to_string()};
        let tk21 = Token{ttype: TokenType::FLOAT, tvalue: "-20.5".to_string()};
        let tk22 = Token{ttype: TokenType::OPERATOR, tvalue: ",".to_string()};
        let tk23 = Token{ttype: TokenType::INTEGER, tvalue: "122".to_string()};
        let tk24 = Token{ttype: TokenType::OPERATOR, tvalue: ",".to_string()};
        let tk25 = Token{ttype: TokenType::HEX, tvalue: "0x9FFF".to_string()};
        let tk26 = Token{ttype: TokenType::BRACKET, tvalue: ")".to_string()};

        let results: [Token; 30] = [
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
          tk12,
          tk13,
          tk14,
          tk15,
          tk27,
          tk28,
          tk29,
          tk30,
          tk16,
          tk17,
          tk18,
          tk19,
          tk20,
          tk21,
          tk22,
          tk23,
          tk24,
          tk25,
          tk26,
        ];

        assert!(tokens.len() == results.len());
        for (test_tk, res_tk) in tokens.into_iter().zip(results.into_iter()) {
            println!("test token: {:?}, result token: {:?}", test_tk, res_tk);
            assert_between_token(test_tk, res_tk);
        }

    }

    #[test]
    fn lexical_test3() {
        let source = "//waefwaefwaef这是一个单行注释\na=12;/*\n这是一个多行注释\n*/b ^= a;";
        let tokens = Lexical::analyse(&source.chars().collect_vec());
        println!("================== geted tokens ==================");
        for tks in &tokens {
            println!("geted token: {:?}", tks);
        }
        println!("================== geted tokens ==================");
        let tk1 = Token{ttype: TokenType::VARIABLE, tvalue: "a".to_string()};
        let tk2 = Token{ttype: TokenType::OPERATOR, tvalue: "=".to_string()};
        let tk3 = Token{ttype: TokenType::INTEGER, tvalue: "12".to_string()};
        let tk4 = Token{ttype: TokenType::OPERATOR, tvalue: ";".to_string()};
        let tk5 = Token{ttype: TokenType::VARIABLE, tvalue: "b".to_string()};
        let tk6 = Token{ttype: TokenType::OPERATOR, tvalue: "^=".to_string()};
        let tk7 = Token{ttype: TokenType::INTEGER, tvalue: "a".to_string()};
        let tk8 = Token{ttype: TokenType::OPERATOR, tvalue: ";".to_string()};
        let results: [Token; 8] = [
            tk1,
            tk2,
            tk3,
            tk4,
            tk5,
            tk6,
            tk7,
            tk8,
        ];
        assert!(tokens.len() == results.len());
        for (test_tk, res_tk) in tokens.into_iter().zip(results.into_iter()) {
            println!("test token: {:?}, result token: {:?}", test_tk, res_tk);
            assert_between_token(test_tk, res_tk);
        }
    }
}
