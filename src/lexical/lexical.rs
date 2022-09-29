use super::models::Tokens::Token;
use super::models::Tokens::TokenType as TokenType;

#[derive(Debug)]
pub struct Lexical;

impl Lexical {
    /*
     * 词法分析
     */
    fn analyse(src: &Vec<String>) -> Vec<Token> {
        let it = src.iter().peekable();
        vec![Token{ ttype: TokenType::BOOLEAN, tvalue: "false".to_string()}]
    }
}
