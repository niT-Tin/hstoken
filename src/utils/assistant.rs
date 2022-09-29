use regex::Regex;

lazy_static! {
    /*
     * 数字正则表达式
     */
    static ref RENUMBER: Regex = Regex::new(r"^[0-9]$").unwrap();

    /*
     * 字符正则表达式
     */
    static ref RELETTER: Regex = Regex::new(r"^[a-zA-Z]$").unwrap();
    /*
     * 字面值正则表达式
     */
    static ref RELITERAL: Regex = Regex::new(r"^[_0-9a-zA-Z]$").unwrap();
    /*
     * 运算符正则表达式
     */
    static ref REOPERATOR: Regex = Regex::new(r"^[+\-\\*/|!<>&^,;]$").unwrap();
    // static ref REOPERATOR: Regex = Regex::new(r"(\+|-|\*|/|\||!|<|>|&|\^|,|;)").unwrap();
    /*
     * 匹配[0-9]的数字
     */
    static ref RANGE09: Regex = Regex::new(r"[0-9]$").unwrap();
    /*
     * 匹配[1-9]的数字
     */
    static ref RANGE19: Regex = Regex::new(r"[1-9]$").unwrap();
    /*
     * 匹配[0-7]的数字
     */
    static ref RANGE07: Regex = Regex::new(r"[0-7]$").unwrap();
    /*
     * 匹配[1-7]的数字
     */
    static ref RANGE17: Regex = Regex::new(r"[1-7]$").unwrap();
    /*
     * 匹配十六进制数字面值
     */
    static ref RANGE09AF: Regex = Regex::new(r"[0-9A-F]$").unwrap();
}

pub struct Assistant;

impl Assistant {
    // 判断字符是否为数字
    pub fn is_number(&self, src: &char) -> bool {
        RENUMBER.is_match(&src.to_string())
    }

    // 判断字符是否为字符
    pub fn is_letter(&self, src: &char) -> bool {
        RELETTER.is_match(&src.to_string())
    }

    // 判读字符是否为字面值
    pub fn is_literal(&self, src: &char) -> bool {
        RELITERAL.is_match(&src.to_string())
    }

    // 判读字符是否为操作符
    pub fn is_operator(&self, src: &char) -> bool {
        REOPERATOR.is_match(&src.to_string())
    }

    pub fn in09(&self, src: &char) -> bool {
        RANGE09.is_match(&src.to_string())
    }

    pub fn in19(&self, src: &char) -> bool {
        RANGE19.is_match(&src.to_string())
    }

    pub fn in07(&self, src: &char) -> bool {
        RANGE07.is_match(&src.to_string())
    }

    pub fn in17(&self, src: &char) -> bool {
        RANGE17.is_match(&src.to_string())
    }

    pub fn in09af(&self, src: &char) -> bool {
        RANGE09AF.is_match(&src.to_string())
    }
}
