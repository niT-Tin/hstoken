/// 词法分析相关model
pub mod Tokens {
    #[derive(Debug)]
    pub enum TokenType {
        VRAIABLE, // 变量
        KEYWORD,  // 关键字

        BRACKET, // 括号
        LITERAL, // 字面值

        OPERATOR, // 操作符

        FLOAT,   // 浮点型
        STRING,  // 字符串型
        BOOLEAN, // 布尔值
        INTEGER, // 整型
        HEX,     // 十六进制
        OCT,     // 八进制
    }

    #[derive(Debug)]
    pub struct Token {
        pub ttype: TokenType, // Token类型
        pub tvalue: String,   // Token值
    }

    /*
     * 正则表达式类型
     */
    #[derive(Debug)]
    pub enum RXT {
        NUMBER,   // 数字
        LETTER,   // 字符
        LITERAL,  // 字面值
        OPERATOR, // 操作符
    }
}

// 关键字
lazy_static! {
    static ref KEYWORDS: [&'static str; 15] = [
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
    ];
}

type TP = Tokens::TokenType;

impl Tokens::Token {
    /*
     * 判断是否为值变量
     */
    fn is_scalar(&self) -> bool {
        matches!(
            self.ttype,
            TP::INTEGER | TP::FLOAT | TP::STRING | TP::BOOLEAN | TP::HEX | TP::OCT
        )
    }

    /*
     * 判断是否为变量
     **/
    fn is_variable(&self) -> bool {
        matches!(self.ttype, TP::VRAIABLE)
    }

    /*
     * 从源代码构造变量或者关键字
     */
    fn make_var_keyword(src: &Vec<char>) -> Tokens::Token {
        Tokens::Token{ ttype: TP::BOOLEAN, tvalue: String::from("") }
    }

    /*
     * 从源代码构造字符串
     */
    fn make_string(src: &Vec<char>) -> Tokens::Token {
        Tokens::Token{ ttype: TP::BOOLEAN, tvalue: String::from("") }
    }

    /*
     * 从源代码构造操作符
     */
    fn make_operator(src: &Vec<char>) -> Tokens::Token {
        Tokens::Token{ ttype: TP::BOOLEAN, tvalue: String::from("") }
    }

    /*
     * 从源代码构造数字
     */
    fn make_number(src: &Vec<char>) -> Tokens::Token {
        Tokens::Token{ ttype: TP::BOOLEAN, tvalue: String::from("") }
    }
}
