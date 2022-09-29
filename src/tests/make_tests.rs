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
        let src1: Vec<char> = "\"This is a double quote string awfawefawef\"".chars().collect();
        let src2: Vec<char> = "'This is a single quote string awfawefawef'".chars().collect();
        let src3: Vec<char> = "'This is a single quote \"string contains double quote'".chars().collect();
        let src4: Vec<char> = "'This is a single quote \"string contains\" double quote'".chars().collect();
        let src5: Vec<char> = "\"This is a double quote string contains' single quote\"".chars().collect();
        let src6: Vec<char> = "\"This is a double quote 'string contains' single quote\"".chars().collect();
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

        assert_token(tk1, TokenType::STRING,  "\"This is a double quote string awfawefawef\"".to_string());
        assert_token(tk2, TokenType::STRING,  "'This is a single quote string awfawefawef'".to_string());
        assert_token(tk3, TokenType::STRING,  "'This is a single quote \"string contains double quote'".to_string());
        assert_token(tk4, TokenType::STRING,  "'This is a single quote \"string contains\" double quote'".to_string());
        assert_token(tk5, TokenType::STRING,  "\"This is a double quote string contains' single quote\"".to_string());
        assert_token(tk6, TokenType::STRING,  "\"This is a double quote 'string contains' single quote\"".to_string());
        assert_token(tk7, TokenType::STRING,  "'This is;'".to_string());
        assert_token(tk8, TokenType::STRING,  "\"stop string\"".to_string());

    }

}
