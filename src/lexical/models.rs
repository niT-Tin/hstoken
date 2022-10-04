use std::collections::HashMap;

use crate::utils::assistant::Assistant;

use self::Tokens::{Token, TokenType};
use itertools::MultiPeek;
use std::slice::Iter;


/// 词法分析相关model
pub mod Tokens {
    #[derive(Debug, Clone)]
    pub enum TokenType {
        VARIABLE, // 变量
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
        ERROR,   // 占位错误Token
    }

    #[derive(Debug, Clone)]
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

// 用于记录哪些状态下能够产生结果，应对最终数字后没有其他字符
lazy_static! {
    static ref res_map: HashMap<i32, TokenType> = HashMap::from([
        (1, TokenType::OCT),
        (4, TokenType::INTEGER),
        (5, TokenType::FLOAT),
        (6, TokenType::FLOAT),
        (7, TokenType::OCT),
        (9, TokenType::HEX),
    ]);
}

// 关键字
lazy_static! {
    static ref KEYWORDS: [String; 22] = [
        // 定义函数关键字
        String::from("fun"),
        String::from("function"),
        String::from("func"),
        String::from("fn"),

        // 定义变量关键字
        String::from("val"),
        String::from("var"),
        String::from("let"),

        // 具体类型关键字
        String::from("int"),
        String::from("float"),
        String::from("string"),
        String::from("hex"), // 十六进制类型
        String::from("oct"), // 八进制类型

        // 程序流转关键字
        String::from("if"),
        String::from("elif"),
        String::from("else"),
        String::from("while"),
        String::from("loop"),
        String::from("for"),
        String::from("return"),

        // 值类型关键字
        String::from("tRUE"), // true 建议使用 ^^ 表达式代替
        String::from("fALSE"), // false 建议使用 !^ 表达式代替
        String::from("void"),

    ];
}

type TP = Tokens::TokenType;

impl Token {
    /*
     * 判断是否为值变量
     */
    pub fn is_scalar(&self) -> bool {
        matches!(
            self.ttype,
            TP::INTEGER | TP::FLOAT | TP::STRING | TP::BOOLEAN | TP::HEX | TP::OCT
        )
    }

    /*
     * 判断是否为数字
     */
    pub fn is_number(&self) -> bool {
        matches!(self.ttype, TP::INTEGER | TP::FLOAT)
    }


    /*
     * 判断是否为操作符
     **/
    pub fn is_operator(&self) -> bool {
        matches!(self.ttype, TP::OPERATOR)
    }

    /*
     * 判断是否为变量
     **/
    pub fn is_variable(&self) -> bool {
        matches!(self.ttype, TP::VARIABLE)
    }

    /*
     * 从源代码构造变量或者关键字
     */
    pub fn make_var_keyword(it: &mut MultiPeek<Iter<char>>) -> Token {
    // pub fn make_var_keyword(src: &mut std::slice::Iter<'_, char>) -> Token {
        // 定义用于承接结果的空字符串
        let mut token = String::from("");
        // 如果字符流不为空的话不断循环
        while let Some(&c) = it.peek() {
            // 因为进入此函数时说明第一个字符已经不是数字，所以此处
            // 判断c是不是literal而不是先判断c是不是letter
            if Assistant.is_literal(c) {
                // 如果是字面值，直接添加进结果
                token.push(*c);
            } else {
                break;
            }
            it.next();
        }
        // 单独判断是否为tRUE或者fALSE
        if token == String::from("tRUE") || token == String::from("fALSE") {
            Token {
                ttype: TP::BOOLEAN,
                tvalue: token,
            }
        // 判断结果是否在关键字列表中
        } else if KEYWORDS.contains(&token) {
            Token {
                ttype: TP::KEYWORD,
                tvalue: token,
            }
        } else {
            Token {
                ttype: TP::VARIABLE,
                tvalue: token,
            }
        }
    }

    /*
     * 从源代码构造字符串
     */
    pub fn make_string(it: &mut MultiPeek<Iter<char>>) -> Token {
        // 定义用于承接结果的空字符串
        let mut token = String::from("");
        // 状态机状态
        enum STATE {
            START, // 初始状态
            SQUOT, // 单引号状态
            DQUOT, // 双引号状态
        }
        let mut state = STATE::START;
        while let Some(&c) = it.peek() {
            match state {
                STATE::START => {
                    if (*c).eq(&'"') {
                        state = STATE::DQUOT;
                    } else if (*c).eq(&'\'') {
                        state = STATE::SQUOT;
                    }
                    token.push(*c);
                    it.next();
                }
                STATE::SQUOT => {
                    if (*c).eq(&'\'') {
                        token.push(*c);
                        it.next();
                        break;
                    }
                    token.push(*c);
                    it.next();
                }
                STATE::DQUOT => {
                    if (*c).eq(&'"') {
                        token.push(*c);
                        it.next();
                        break;
                    }
                    token.push(*c);
                    it.next();
                }
            }
        }
        Token {
            ttype: TP::STRING,
            tvalue: token,
        }
    }

    /*
     * 从源代码构造操作符
     */
    pub fn make_operator(it: &mut MultiPeek<Iter<char>>) -> Token {
        let mut token = String::from("");
        let mut state = 0;
        while let Some(&c) = it.peek() {
            match state {
                0 => {
                    match *c {
                       '+'  => state = 1,
                        '-' => state = 2,
                        '*' => state = 3,
                        '/' => state = 4,
                        '!' => state = 5,
                        '=' => state = 6,
                        '<' => state = 7,
                        '>' => state = 8,
                        '%' => state = 9,
                        '^' => state = 10,
                        '|' => state = 11,
                        _ => state = 12,
                    }
                }
                1 => {
                    if (*c).eq(&'+') || (*c).eq(&'=') {
                        token.push(*c);
                        break;
                    } else {
                        break;
                    }
                },
                2 => {
                    if (*c).eq(&'-') || (*c).eq(&'=') {
                        token.push(*c);
                        break;
                    } else {
                        break;
                    }
                },
                3 | 4 | 6 | 9 => {
                    if (*c).eq(&'=') {
                        token.push(*c);
                        break;
                    } else {
                        break;
                    }
                },
                5 => {
                    if (*c).eq(&'^') || (*c).eq(&'=') {
                        token.push(*c);
                        break;
                    } else {
                        break;
                    }
                },
                7 => {
                    if (*c).eq(&'<') || (*c).eq(&'>') || (*c).eq(&'=') {
                        token.push(*c);
                        break;
                    } else {
                        break;
                    }
                },
                8 => {
                    if (*c).eq(&'>') || (*c).eq(&'=') {
                        token.push(*c);
                        break;
                    } else {
                        break;
                    }
                },
                10 => {
                    if (*c).eq(&'^') || (*c).eq(&'=') {
                        token.push(*c);
                        break;
                    } else {
                        break;
                    }
                },
                11 => {
                    if (*c).eq(&'|') || (*c).eq(&'=') {
                        token.push(*c);
                        break;
                    } else {
                        break;
                    }
                },
                _ => return Token{ttype: TP::ERROR, tvalue: "outer".to_string()}
            }
            token.push(*c);
            it.next();
        }
        Token {
            ttype: TP::OPERATOR,
            tvalue: token,
        }
    }

    /*
     * 从源代码构造数字
     */
    pub fn make_number(it: &mut MultiPeek<Iter<char>>) -> Token {
        let mut token = String::from("");
        let mut state = 0;

        while let Some(&c) = it.peek() {
            match state {
                0 => {
                    if Assistant.in19(c) {
                        state = 4;
                    } else if (*c).eq(&'+') || (*c).eq(&'-') {
                        state = 2;
                    } else if (*c).eq(&'0') {
                        state = 1;
                    } else if (*c).eq(&'.') {
                        state = 3;
                    }
                }
                1 => {
                    if (*c).eq(&'0') {
                        state = 1;
                    } else if Assistant.in17(c) {
                        state = 7;
                    } else if (*c).eq(&'.') {
                        state = 3;
                    } else if (*c).eq(&'x') {
                        state = 9;
                    } else if !Assistant.in17(c) && Assistant.is_number(c) {
                        // TODO 抛出错误取代返回的占位Token
                        return Token {
                            ttype: Tokens::TokenType::ERROR,
                            tvalue: String::from("1"),
                        };
                    } else {
                        return Token {
                            ttype: TP::OCT,
                            tvalue: token,
                        };
                    }
                }
                2 => {
                    if Assistant.in19(c) {
                        state = 4;
                    } else if (*c).eq(&'0') {
                        state = 1;
                    } else if (*c).eq(&'.') {
                        state = 3;
                    } else {
                        // TODO 抛出错误取代返回的占位Token
                        return Token {
                            ttype: Tokens::TokenType::ERROR,
                            tvalue: String::from("2"),
                        };
                    }
                }
                3 => {
                    if Assistant.in09(c) {
                        state = 6;
                    } else {
                        // TODO 抛出错误取代返回的占位Token
                        return Token {
                            ttype: Tokens::TokenType::ERROR,
                            tvalue: String::from("3"),
                        };
                    }
                }
                4 => {
                    if Assistant.in09(c) {
                        state = 4;
                    } else if (*c).eq(&'.') {
                        state = 5;
                    } else {
                        return Token {
                            ttype: Tokens::TokenType::INTEGER,
                            tvalue: token,
                        };
                    }
                }
                5 => {
                    if (*c).eq(&'.') {
                        // TODO 抛出错误取代返回的占位Token
                        return Token {
                            ttype: Tokens::TokenType::ERROR,
                            tvalue: String::from("5"),
                        };
                    } else if Assistant.in09(c) {
                        state = 6;
                    } else {
                        return Token {
                            ttype: Tokens::TokenType::FLOAT,
                            tvalue: token,
                        };
                    }
                }
                6 => {
                    if (*c).eq(&'.') {
                        // TODO 抛出错误取代返回的占位Token
                        return Token {
                            ttype: Tokens::TokenType::ERROR,
                            tvalue: String::from("6"),
                        };
                    } else if Assistant.in09(c) {
                        state = 6;
                    } else {
                        return Token {
                            ttype: Tokens::TokenType::FLOAT,
                            tvalue: token,
                        };
                    }
                }
                7 => {
                    if (*c).eq(&'.') {
                        // TODO 抛出错误取代返回的占位Token
                        return Token {
                            ttype: Tokens::TokenType::ERROR,
                            tvalue: String::from("7"),
                        };
                    } else if Assistant.in07(c) {
                        state = 7;
                    } else {
                        return Token {
                            ttype: Tokens::TokenType::OCT,
                            tvalue: token,
                        };
                    }
                }
                9 => {
                    if (*c).eq(&'.') {
                        // TODO 抛出错误取代返回的占位Token
                        return Token {
                            ttype: Tokens::TokenType::ERROR,
                            tvalue: String::from("9"),
                        };
                    } else if Assistant.in09af(c) {
                        state = 9;
                    } else if !Assistant.in09af(c) && Assistant.is_literal(c) {
                        // TODO 抛出错误取代返回的占位Token
                        return Token {
                            ttype: Tokens::TokenType::ERROR,
                            tvalue: String::from("9"),
                        };
                    } else {
                        return Token {
                            ttype: Tokens::TokenType::HEX,
                            tvalue: token,
                        };
                    }
                }
                _ => {
                    // TODO 抛出错误取代返回的占位Token
                    return Token {
                        ttype: Tokens::TokenType::ERROR,
                        tvalue: String::from("O"),
                    };
                }
            }
            token.push(*c);
            it.next();
        }
        // 判断是从那个有结果的分支出来的
        match res_map.get(&state) {
            // 如果是有结果的分支，则返回对应结果
            Some(s) => Token { ttype: (*s).clone(), tvalue: token },
            // 如果没有结果则返回错误
            None => Token {
                ttype: TP::ERROR,
                tvalue: String::from("outer"),
            },
        }
    }
}
