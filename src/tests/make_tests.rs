#[cfg(test)]
mod make_things {
    use itertools::{Itertools, MultiPeek};
    use std::{borrow::BorrowMut, slice::Iter};

    use crate::lexical::models::Tokens::{Token, TokenType};

    fn assert_token(tk: Token, typ: TokenType, value: String) {
        // println!("The value is: {}", value);
        assert!(matches!(tk.ttype, typ) && tk.tvalue.eq(&value));
    }

    fn assert_between_token(tk1: Token, tk2: Token) {
        assert_token(tk1, tk2.ttype, tk2.tvalue);
    }

    #[test]
    fn make_var_keyword_test() {
        let r1 = "fun saawefawef".chars().collect::<Vec<char>>();
        let r2 = "function awefawef".chars().collect::<Vec<char>>();
        let r3 = "let awefawef".chars().collect::<Vec<char>>();
        let r4 = "string 0899jlk".chars().collect::<Vec<char>>();
        let r5 = "else awefawef".chars().collect::<Vec<char>>();
        let r6 = "tRUE awefawef".chars().collect::<Vec<char>>();
        let r7 = "awefawef if ".chars().collect::<Vec<char>>();
        let r8 = "swef".chars().collect::<Vec<char>>();
        let r9 = "iawf".chars().collect::<Vec<char>>();
        let r10 = "varjowaef".chars().collect::<Vec<char>>();
        let mut srcs: [MultiPeek<Iter<char>>; 10] = [
            r1.iter().multipeek(),
            r2.iter().multipeek(),
            r3.iter().multipeek(),
            r4.iter().multipeek(),
            r5.iter().multipeek(),
            r6.iter().multipeek(),
            r7.iter().multipeek(),
            r8.iter().multipeek(),
            r9.iter().multipeek(),
            r10.iter().multipeek(),
        ];

        let res_tk = [
            Token {
                ttype: TokenType::KEYWORD,
                tvalue: "fun".to_string(),
            },
            Token {
                ttype: TokenType::KEYWORD,
                tvalue: "function".to_string(),
            },
            Token {
                ttype: TokenType::KEYWORD,
                tvalue: "let".to_string(),
            },
            Token {
                ttype: TokenType::KEYWORD,
                tvalue: "string".to_string(),
            },
            Token {
                ttype: TokenType::KEYWORD,
                tvalue: "else".to_string(),
            },
            Token {
                ttype: TokenType::KEYWORD,
                tvalue: "tRUE".to_string(),
            },
            Token {
                ttype: TokenType::VARIABLE,
                tvalue: "awefawef".to_string(),
            },
            Token {
                ttype: TokenType::VARIABLE,
                tvalue: "swef".to_string(),
            },
            Token {
                ttype: TokenType::VARIABLE,
                tvalue: "iawf".to_string(),
            },
            Token {
                ttype: TokenType::VARIABLE,
                tvalue: "varjowaef".to_string(),
            },
        ];

        for ts in srcs.iter_mut().enumerate() {
            let (i, t) = ts;
            let test_tk = Token::make_var_keyword(t);
            let ans_tk = res_tk.get(i).unwrap();
            assert_between_token(test_tk, ans_tk.clone());
        }
    }

    #[test]
    fn make_string_test() {
        let src1: Vec<char> = "\"This is a double quote string awfawefawef\""
            .chars()
            .collect();
        let src2: Vec<char> = "'This is a single quote string awfawefawef'"
            .chars()
            .collect();
        let src3: Vec<char> = "'This is a single quote \"string contains double quote'"
            .chars()
            .collect();
        let src4: Vec<char> = "'This is a single quote \"string contains\" double quote'"
            .chars()
            .collect();
        let src5: Vec<char> = "\"This is a double quote string contains' single quote\""
            .chars()
            .collect();
        let src6: Vec<char> = "\"This is a double quote 'string contains' single quote\""
            .chars()
            .collect();
        let src7: Vec<char> = "'This is;' a string".chars().collect();
        let src8: Vec<char> = "\"stop string\"awefwaef awaefawef".chars().collect();

        let mut src1 = src1.iter().multipeek();
        let mut src2 = src2.iter().multipeek();
        let mut src3 = src3.iter().multipeek();
        let mut src4 = src4.iter().multipeek();
        let mut src5 = src5.iter().multipeek();
        let mut src6 = src6.iter().multipeek();
        let mut src7 = src7.iter().multipeek();
        let mut src8 = src8.iter().multipeek();

        let tk1 = Token::make_string(&mut src1);
        let tk2 = Token::make_string(&mut src2);
        let tk3 = Token::make_string(&mut src3);
        let tk4 = Token::make_string(&mut src4);
        let tk5 = Token::make_string(&mut src5);
        let tk6 = Token::make_string(&mut src6);
        let tk7 = Token::make_string(&mut src7);
        let tk8 = Token::make_string(&mut src8);

        assert_token(
            tk1,
            TokenType::STRING,
            "\"This is a double quote string awfawefawef\"".to_string(),
        );
        assert_token(
            tk2,
            TokenType::STRING,
            "'This is a single quote string awfawefawef'".to_string(),
        );
        assert_token(
            tk3,
            TokenType::STRING,
            "'This is a single quote \"string contains double quote'".to_string(),
        );
        assert_token(
            tk4,
            TokenType::STRING,
            "'This is a single quote \"string contains\" double quote'".to_string(),
        );
        assert_token(
            tk5,
            TokenType::STRING,
            "\"This is a double quote string contains' single quote\"".to_string(),
        );
        assert_token(
            tk6,
            TokenType::STRING,
            "\"This is a double quote 'string contains' single quote\"".to_string(),
        );
        assert_token(tk7, TokenType::STRING, "'This is;'".to_string());
        assert_token(tk8, TokenType::STRING, "\"stop string\"".to_string());
    }

    #[test]
    fn make_number() {
        let src1 = String::from("+0 aa").chars().collect::<Vec<char>>();
        let src2 = String::from("-0 awefo").chars().collect::<Vec<char>>();
        let src3 = String::from(".3 123").chars().collect::<Vec<char>>();
        let src4 = String::from(".55555 989").chars().collect::<Vec<char>>();
        let src5 = String::from("787887.1234 aweawef")
            .chars()
            .collect::<Vec<char>>();
        let src6 = String::from("-02345*123123 ailkk")
            .chars()
            .collect::<Vec<char>>();
        let src7 = String::from("+0xFFF87*afwaef")
            .chars()
            .collect::<Vec<char>>();
        let src8 = String::from("-09awaef wefwaeef")
            .chars()
            .collect::<Vec<char>>();
        let src9 = String::from("+0xFGF87*afwaef ")
            .chars()
            .collect::<Vec<char>>();
        let src10 = String::from("0x09FFF ").chars().collect::<Vec<char>>();
        let src11 = String::from("0000000.3 ").chars().collect::<Vec<char>>();
        let src12 = String::from("-00000005 JHULIkjk")
            .chars()
            .collect::<Vec<char>>();
        let src13 = String::from("0000000878987").chars().collect::<Vec<char>>();
        let src14 = String::from("0x09F.FF3wr").chars().collect::<Vec<char>>();
        let src15 = String::from("010.00008&*&%&")
            .chars()
            .collect::<Vec<char>>();
        let src16 = String::from("323452345awef").chars().collect::<Vec<char>>();
        let src17 = String::from("+8993 awef").chars().collect::<Vec<char>>();
        let src18 = String::from("-awefweaf").chars().collect::<Vec<char>>();
        let src19 = String::from("-325344").chars().collect::<Vec<char>>();
        let src20 = String::from("-.325344").chars().collect::<Vec<char>>();
        let srcs = vec![
            src1.iter().multipeek(),
            src2.iter().multipeek(),
            src3.iter().multipeek(),
            src4.iter().multipeek(),
            src5.iter().multipeek(),
            src6.iter().multipeek(),
            src7.iter().multipeek(),
            src8.iter().multipeek(),
            src9.iter().multipeek(),
            src10.iter().multipeek(),
            src11.iter().multipeek(),
            src12.iter().multipeek(),
            src13.iter().multipeek(),
            src14.iter().multipeek(),
            src15.iter().multipeek(),
            src16.iter().multipeek(),
            src17.iter().multipeek(),
            src18.iter().multipeek(),
            src19.iter().multipeek(),
            src20.iter().multipeek(),
        ];
        let res = vec![
            Token {
                ttype: TokenType::OCT,
                tvalue: String::from("+0"),
            },
            Token {
                ttype: TokenType::OCT,
                tvalue: String::from("-0"),
            },
            Token {
                ttype: TokenType::FLOAT,
                tvalue: String::from(".3"),
            },
            Token {
                ttype: TokenType::FLOAT,
                tvalue: String::from(".55555"),
            },
            Token {
                ttype: TokenType::FLOAT,
                tvalue: String::from("787887.1234"),
            },
            Token {
                ttype: TokenType::OCT,
                tvalue: String::from("-02345"),
            },
            Token {
                ttype: TokenType::HEX,
                tvalue: String::from("+0xFFF87"),
            },
            Token {
                ttype: TokenType::ERROR,
                tvalue: String::from("1"),
            },
            Token {
                ttype: TokenType::ERROR,
                tvalue: String::from("9"),
            },
            Token {
                ttype: TokenType::HEX,
                tvalue: String::from("0x09FFF"),
            },
            Token {
                ttype: TokenType::FLOAT,
                tvalue: String::from("0000000.3"),
            },
            Token {
                ttype: TokenType::OCT,
                tvalue: String::from("-00000005"),
            },
            Token {
                ttype: TokenType::ERROR,
                tvalue: String::from("1"),
            },
            Token {
                ttype: TokenType::ERROR,
                tvalue: String::from("9"),
            },
            Token {
                ttype: TokenType::ERROR,
                tvalue: String::from("7"),
            },
            Token {
                ttype: TokenType::INTEGER,
                tvalue: String::from("323452345"),
            },
            Token {
                ttype: TokenType::INTEGER,
                tvalue: String::from("+8993"),
            },
            Token {
                ttype: TokenType::ERROR,
                tvalue: String::from("2"),
            },
            Token {
                ttype: TokenType::INTEGER,
                tvalue: String::from("-325344"),
            },
            Token {
                ttype: TokenType::FLOAT,
                tvalue: String::from("-.325344"),
            },
        ];
        // println!("srcs len: {}", srcs.len());
        // println!("res len: {}", res.len());
        for is in srcs.into_iter().enumerate() {
            let (pos, mut s) = is;
            let test_tk = Token::make_number(&mut s);
            let res_tk = res.get(pos).unwrap();
            // print!("test_tk: {:?} ", test_tk);
            // println!("res_tk: {:?}", res_tk);
            assert_token(test_tk, res_tk.ttype.clone(), res_tk.tvalue.clone());
        }
    }

    #[test]
    fn make_operator() {
        let src1 = "+4".chars().collect::<Vec<char>>();
        let src2 = "--i".chars().collect::<Vec<char>>();
        let src3 = "*5".chars().collect::<Vec<char>>();
        let src4 = "++i".chars().collect::<Vec<char>>();
        let src5 = "+=waef".chars().collect::<Vec<char>>();
        let src6 = "-= ".chars().collect::<Vec<char>>();
        let src7 = "!=wa".chars().collect::<Vec<char>>();
        let src8 = "!= awefawef".chars().collect::<Vec<char>>();
        let src9 = "^^2a3ra ".chars().collect::<Vec<char>>();
        let src10 = "!^aewawf".chars().collect::<Vec<char>>();
        let src11 = "&".chars().collect::<Vec<char>>();
        let src12 = "%=aefaw".chars().collect::<Vec<char>>();
        let src13 = "||awfawe".chars().collect::<Vec<char>>();

        let srcs = vec![
           src1.iter().multipeek(),
           src2.iter().multipeek(),
           src3.iter().multipeek(),
           src4.iter().multipeek(),
           src5.iter().multipeek(),
           src6.iter().multipeek(),
           src7.iter().multipeek(),
           src8.iter().multipeek(),
           src9.iter().multipeek(),
           src10.iter().multipeek(),
           src11.iter().multipeek(),
           src12.iter().multipeek(),
           src13.iter().multipeek(),
        ];

        let ress = vec![
            Token {
                ttype: TokenType::OPERATOR,
                tvalue: "+".to_string(),
            },
            Token {
                ttype: TokenType::OPERATOR,
                tvalue: "--".to_string(),
            },
            Token {
                ttype: TokenType::OPERATOR,
                tvalue: "*".to_string(),
            },
            Token {
                ttype: TokenType::OPERATOR,
                tvalue: "++".to_string(),
            },
            Token {
                ttype: TokenType::OPERATOR,
                tvalue: "+=".to_string(),
            },
            Token {
                ttype: TokenType::OPERATOR,
                tvalue: "-=".to_string(),
            },
            Token {
                ttype: TokenType::OPERATOR,
                tvalue: "!=".to_string(),
            },
            Token {
                ttype: TokenType::OPERATOR,
                tvalue: "!=".to_string(),
            },
            Token {
                ttype: TokenType::OPERATOR,
                tvalue: "^^".to_string(),
            },
            Token {
                ttype: TokenType::OPERATOR,
                tvalue: "!^".to_string(),
            },
            Token {
                ttype: TokenType::OPERATOR,
                tvalue: "&".to_string(),
            },
            Token {
                ttype: TokenType::OPERATOR,
                tvalue: "%=".to_string(),
            },
            Token {
                ttype: TokenType::OPERATOR,
                tvalue: "||".to_string(),
            },
        ];
        for is in srcs.into_iter().enumerate() {
            let (pos, mut s) = is;
            let test_tk = Token::make_operator(&mut s);
            let res_tk = ress.get(pos).unwrap();
            print!("test_tk: {:?} ", test_tk);
            println!("res_tk: {:?}", res_tk);
            assert_token(test_tk, res_tk.ttype.clone(), res_tk.tvalue.clone());
        }
    }
}
