use crate::base::base::PPUTKIterator;
use crate::utils::assistant::Assistant;

use super::models::Tokens::Token;
use super::models::Tokens::TokenType;

#[derive(Debug)]
pub struct Lexical;

impl Lexical {
    /*
     * 词法分析
     */
    pub fn analyse(src: &Vec<char>) -> Vec<Token> {
        let mut result = Vec::new();
        let iter = src.iter();
        let mut pk_iter = PPUTKIterator::new(iter);
        // 不断peek字符判断不同类型构建不同类型Token
        while let Some(s) = pk_iter.next() {
            // 忽略空白符
            if Assistant.is_blank(s) {
                continue;
            }

            // 判断是否为括号
            if Assistant.is_bracket(s) {
                let tk = Token {
                    ttype: TokenType::BRACKET,
                    tvalue: s.to_string(),
                };
                result.push(tk);
                continue;
            }
            // 字符为letter则构建变量或关键字
            if Assistant.is_letter(s) {
                pk_iter.put_back();
                let tk = Token::make_var_keyword(&mut pk_iter);
                result.push(tk);
                continue;
            }
            // 字符为 ' 或者 " 则构建字符串
            if (*s).eq(&'\'') || (*s).eq(&'"') {
                pk_iter.put_back();
                let tk = Token::make_string(&mut pk_iter);
                result.push(tk);
                continue;
            }

            // 构建简单数字
            if Assistant.is_number(s) {
                pk_iter.put_back();
                let tk = Token::make_number(&mut pk_iter);
                result.push(tk);
                continue;
            }

            // 考虑 3+4, +4 是两种情况
            // 此处局部上下文有关，此处的符号含义需要参考前后的符号
            // 前一个符号如果是数字并且后一个token也是数字，这个加号应该被
            // 定义为操作符，而如果前一个token不是数字，则应该定义为数字
            if (*s).eq(&'+') || (*s).eq(&'-') || (*s).eq(&'.') {
                // ahead为 next的后一个，也就是当前s的后一个字符
                let ahead = pk_iter.peek();
                if let Some(c) = ahead {
                    // 如果后一个符号也为数字
                    if Assistant.is_number(c) {
                        // 获取最后一个token,也就是当前字符的前一个token
                        match result.get(result.len() - 1) {
                            // 当前一个token不存在，或者前一个token为操作符时，
                            // 当前的+，-号表示当前下一个token的正负(此时已经知道下一个
                            // 字符为数字)
                            Some(tk) => {
                                // 前一个token不是操作符
                                // 则开始构建数字
                                if tk.is_operator() {
                                    pk_iter.put_back();
                                    let tk = Token::make_number(&mut pk_iter);
                                    result.push(tk);
                                    continue;
                                }
                            }
                            None => {
                                pk_iter.put_back();
                                // 如果前一个token为空则直接构建数字
                                let tk = Token::make_number(&mut pk_iter);
                                result.push(tk);
                                continue;
                            }
                        };
                    }
                }
            }

            // 删除注释
            if (*s).eq(&'/') {
                if let Some(s) = pk_iter.peek() {
                    // 此处表明是当行双斜线注释 //
                    if (*s).eq(&'/') {
                        loop {
                            if let Some(s) = pk_iter.next() {
                                if (*s).eq(&'\n') {
                                    break;
                                } else {
                                    continue;
                                }
                            }
                        }
                        continue;
                        // 此处表明是多行注释
                    } else if (*s).eq(&'*') {
                        loop {
                            if let Some(_) = pk_iter.next() {
                                let end = pk_iter.peek();
                                match end {
                                    Some(cs) => {
                                        if (*cs).eq(&'*') {
                                            pk_iter.next();
                                            let ed = pk_iter.peek();
                                            match ed {
                                                Some(d) => {
                                                    if (*d).eq(&'/') {
                                                        pk_iter.next();
                                                        break;
                                                    } else {
                                                        continue;
                                                    }
                                                }
                                                None => {
                                                    panic!("Comment error");
                                                }
                                            }
                                        }
                                    }
                                    None => {
                                        panic!("Comment error");
                                    }
                                };
                            }
                        }
                        continue;
                    }
                }
            }

            // 构建操作符
            if Assistant.is_operator(s) {
                pk_iter.put_back();
                let tk = Token::make_operator(&mut pk_iter);
                result.push(tk);
                continue;
            }
            // FOR TEST
            eprintln!("================== 即将报错 已经获得的Token有如下 ==================");
            for (i, tks) in (&result).iter().enumerate() {
                println!("第 {}, 个 token: {:?}", i, tks);
            }
            eprintln!("报错的字符为: {}", *s);
            // 其他类型token直接报错, 中断程序执行
            let error_tk = Token {
                ttype: TokenType::ERROR,
                tvalue: String::from("Error"),
            };
            eprintln!("Unexpected token: {:?}", error_tk);
            panic!("Unexpected token: {:?}", error_tk);
        }
        result
    }
}
