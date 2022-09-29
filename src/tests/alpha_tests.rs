#[cfg(test)]
// 字符符号获取测试
mod alpha_test {
    use crate::utils::assistant::Assistant;

    #[test]
    fn assiatant_letter() {
        assert_eq!(true, Assistant.is_letter('a'));
        assert_eq!(true, Assistant.is_letter('l'));
        assert_eq!(true, Assistant.is_letter('D'));
        assert_eq!(false, Assistant.is_letter('.'));
        assert_eq!(false, Assistant.is_letter(';'));
    }

    #[test]
    fn assiatant_number() {
        assert_eq!(true, Assistant.is_number('0'));
        assert_eq!(true, Assistant.is_number('8'));
        assert_eq!(false, Assistant.is_number('a'));
    }

    #[test]
    fn assiatant_literal() {
        assert_eq!(true, Assistant.is_literal('a'));
        assert_eq!(true, Assistant.is_literal('_'));
        assert_eq!(true, Assistant.is_literal('A'));
        assert_eq!(false, Assistant.is_letter('&'));
    }

    #[test]
    fn assiatant_operator() {
        assert_eq!(true, Assistant.is_operator('+'));
        assert_eq!(true, Assistant.is_operator('-'));
        assert_eq!(true, Assistant.is_operator('&'));
        assert_eq!(false, Assistant.is_operator('a'));
        assert_eq!(true, Assistant.is_operator('|'));
        assert_eq!(true, Assistant.is_operator('^'));
        assert_eq!(true, Assistant.is_operator('/'));
        assert_eq!(true, Assistant.is_operator('*'));
        assert_eq!(true, Assistant.is_operator(';'));
        assert_eq!(false, Assistant.is_operator('9'));
    }
}
