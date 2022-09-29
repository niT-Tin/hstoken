use regex::Regex;

lazy_static! {
    /*
     * 数字正则表达式
     */
    static ref RENUMBER: Regex = Regex::new(r"^[0-9]$").unwrap();

    /*
     * 字符正则表达式
     */
    static ref RELETTER: Regex = Regex::new(r"^[_a-zA-Z]$").unwrap();
    /*
     * 字面值正则表达式
     */
    static ref RELITERAL: Regex = Regex::new(r"^[_0-9a-zA-Z]$").unwrap();
    /*
     * 运算符正则表达式
     */
    static ref REOPERATOR: Regex = Regex::new(r"[+-\*/|!<>&^]$").unwrap();
}


pub struct Assistant;

impl Assistant {
    // 判断字符是否为数字
    fn is_number(&self, src: char) -> bool {
        RENUMBER.is_match(&src.to_string())
    }

    // 判断字符是否为字符
    fn is_letter(&self, src: char) -> bool {
        RELETTER.is_match(&src.to_string())
    }

    // 判读字符是否为字面值
    fn is_literal(&self, src: char) -> bool {
        RELITERAL.is_match(&src.to_string())
    }

    // 判读字符是否为操作符
    fn is_operator(&self, src: char) -> bool {
        REOPERATOR.is_match(&src.to_string())
    }
}
