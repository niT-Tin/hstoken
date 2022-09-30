#[cfg(test)]
mod make_things {
    use crate::lexical::models::Tokens::{Token, TokenType};

    fn assert_token(tk: Token, typ: TokenType, value: String) {
        // println!("The value is: {}", value);
        assert!(matches!(tk.ttype, typ) && tk.tvalue.eq(&value));
    }

    #[test]
    fn make_var_keyword_test() {
        let src1: Vec<char> = "fun saawefawef".chars().collect();
        let src2: Vec<char> = "function awefawef".chars().collect();
        let src3: Vec<char> = "let awefawef".chars().collect();
        let src4: Vec<char> = "string awefawef".chars().collect();
        let src5: Vec<char> = "else awefawef".chars().collect();
        let src6: Vec<char> = "tRUE awefawef".chars().collect();
        let src7: Vec<char> = "awefawef if ".chars().collect();
        let src8: Vec<char> = "swef".chars().collect();
        let src9: Vec<char> = "iawf".chars().collect();
        let src10: Vec<char> = "varjowaef".chars().collect();

        let tk1 = Token::make_var_keyword(&src1);
        let tk2 = Token::make_var_keyword(&src2);
        let tk3 = Token::make_var_keyword(&src3);
        let tk4 = Token::make_var_keyword(&src4);
        let tk5 = Token::make_var_keyword(&src5);
        let tk6 = Token::make_var_keyword(&src6);
        let tk7 = Token::make_var_keyword(&src7);
        let tk8 = Token::make_var_keyword(&src8);
        let tk9 = Token::make_var_keyword(&src9);
        let tk10 = Token::make_var_keyword(&src10);

        assert_token(tk1, TokenType::KEYWORD, "fun".to_string());
        assert_token(tk2, TokenType::KEYWORD, "function".to_string());
        assert_token(tk3, TokenType::KEYWORD, "let".to_string());
        assert_token(tk4, TokenType::KEYWORD, "string".to_string());
        assert_token(tk5, TokenType::KEYWORD, "else".to_string());
        assert_token(tk6, TokenType::BOOLEAN, "tRUE".to_string());
        assert_token(tk7, TokenType::VARIABLE, "awefawef".to_string());
        assert_token(tk8, TokenType::VARIABLE, "swef".to_string());
        assert_token(tk9, TokenType::VARIABLE, "iawf".to_string());
        assert_token(tk10, TokenType::VARIABLE, "varjowaef".to_string());
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

        let tk1 = Token::make_string(&src1);
        let tk2 = Token::make_string(&src2);
        let tk3 = Token::make_string(&src3);
        let tk4 = Token::make_string(&src4);
        let tk5 = Token::make_string(&src5);
        let tk6 = Token::make_string(&src6);
        let tk7 = Token::make_string(&src7);
        let tk8 = Token::make_string(&src8);

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
        let srcs = vec![
            String::from("+0 aa").chars().collect::<Vec<char>>(),
            String::from("-0 awefo").chars().collect::<Vec<char>>(),
            String::from(".3 123").chars().collect::<Vec<char>>(),
            String::from(".55555 989").chars().collect::<Vec<char>>(),
            String::from("787887.1234 aweawef")
                .chars()
                .collect::<Vec<char>>(),
            String::from("-02345*123123 ailkk")
                .chars()
                .collect::<Vec<char>>(),
            String::from("+0xFFF87*afwaef")
                .chars()
                .collect::<Vec<char>>(),
            String::from("-09awaef wefwaeef")
                .chars()
                .collect::<Vec<char>>(),
            String::from("+0xFGF87*afwaef ")
                .chars()
                .collect::<Vec<char>>(),
            String::from("0x09FFF ").chars().collect::<Vec<char>>(),
            String::from("0000000.3 ").chars().collect::<Vec<char>>(),
            String::from("-00000005 JHULIkjk")
                .chars()
                .collect::<Vec<char>>(),
            String::from("0000000878987").chars().collect::<Vec<char>>(),
            String::from("0x09F.FF3wr").chars().collect::<Vec<char>>(),
            String::from("010.00008&*&%&")
                .chars()
                .collect::<Vec<char>>(),
            String::from("323452345awef").chars().collect::<Vec<char>>(),
            String::from("+8993 awef").chars().collect::<Vec<char>>(),
            String::from("-awefweaf").chars().collect::<Vec<char>>(),
            String::from("-325344").chars().collect::<Vec<char>>(),
            String::from("-.325344").chars().collect::<Vec<char>>(),
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
        for is in srcs.iter().enumerate() {
            let (pos, s) = is;
            let test_tk = Token::make_number(s);
            let res_tk = res.get(pos).unwrap();
            // print!("test_tk: {:?} ", test_tk);
            // println!("res_tk: {:?}", res_tk);
            assert_token(test_tk, res_tk.ttype.clone(), res_tk.tvalue.clone());
        }
    }

    #[test]
    fn make_operator() {
        let srcs = vec![
            "+4".chars().collect::<Vec<char>>(),
            "--i".chars().collect::<Vec<char>>(),
            "*5".chars().collect::<Vec<char>>(),
            "++i".chars().collect::<Vec<char>>(),
            "+=waef".chars().collect::<Vec<char>>(),
            "-= ".chars().collect::<Vec<char>>(),
            "!=wa".chars().collect::<Vec<char>>(),
            "!= awefawef".chars().collect::<Vec<char>>(),
            "^^2a3ra ".chars().collect::<Vec<char>>(),
            "!^aewawf".chars().collect::<Vec<char>>(),
            "&".chars().collect::<Vec<char>>(),
            "%=aefaw".chars().collect::<Vec<char>>(),
            "||awfawe".chars().collect::<Vec<char>>(),
        ];

        let ress = vec![
            Token {ttype: TokenType::OPERATOR, tvalue: "+".to_string()},
            Token {ttype: TokenType::OPERATOR, tvalue: "--".to_string()},
            Token {ttype: TokenType::OPERATOR, tvalue: "*".to_string()},
            Token {ttype: TokenType::OPERATOR, tvalue: "++".to_string()},
            Token {ttype: TokenType::OPERATOR, tvalue: "+=".to_string()},
            Token {ttype: TokenType::OPERATOR, tvalue: "-=".to_string()},
            Token {ttype: TokenType::OPERATOR, tvalue: "!=".to_string()},
            Token {ttype: TokenType::OPERATOR, tvalue: "!=".to_string()},
            Token {ttype: TokenType::OPERATOR, tvalue: "^^".to_string()},
            Token {ttype: TokenType::OPERATOR, tvalue: "!^".to_string()},
            Token {ttype: TokenType::OPERATOR, tvalue: "&".to_string()},
            Token {ttype: TokenType::OPERATOR, tvalue: "%=".to_string()},
            Token {ttype: TokenType::OPERATOR, tvalue: "||".to_string()},
        ];
        for is in srcs.iter().enumerate() {
            let (pos, s) = is;
            let test_tk = Token::make_operator(s);
            let res_tk = ress.get(pos).unwrap();
            print!("test_tk: {:?} ", test_tk);
            println!("res_tk: {:?}", res_tk);
            assert_token(test_tk, res_tk.ttype.clone(), res_tk.tvalue.clone());
        }

    }

}
