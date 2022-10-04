#[cfg(test)]
// 字符符号获取测试
mod alpha_test {
    use crate::utils::assistant::Assistant;

    #[test]
    fn assistant_letter() {
        assert_eq!(true, Assistant.is_letter(&'a'));
        assert_eq!(true, Assistant.is_letter(&'l'));
        assert_eq!(true, Assistant.is_letter(&'D'));
        assert_eq!(false, Assistant.is_letter(&'.'));
        assert_eq!(false, Assistant.is_letter(&';'));
    }

    #[test]
    fn assistant_number() {
        assert_eq!(true, Assistant.is_number(&'0'));
        assert_eq!(true, Assistant.is_number(&'8'));
        assert_eq!(false, Assistant.is_number(&'a'));
    }

    #[test]
    fn assistant_literal() {
        assert_eq!(true, Assistant.is_literal(&'a'));
        assert_eq!(true, Assistant.is_literal(&'_'));
        assert_eq!(true, Assistant.is_literal(&'A'));
        assert_eq!(false, Assistant.is_letter(&'&'));
    }

    #[test]
    fn assistant_operator() {
        assert_eq!(true, Assistant.is_operator(&'+'));
        assert_eq!(true, Assistant.is_operator(&'-'));
        assert_eq!(true, Assistant.is_operator(&'&'));
        assert_eq!(false, Assistant.is_operator(&'a'));
        assert_eq!(true, Assistant.is_operator(&'|'));
        assert_eq!(true, Assistant.is_operator(&'^'));
        assert_eq!(true, Assistant.is_operator(&'/'));
        assert_eq!(true, Assistant.is_operator(&'*'));
        assert_eq!(true, Assistant.is_operator(&';'));
        assert_eq!(false, Assistant.is_operator(&'9'));
    }

    #[test]
    fn assistant_range() {
        assert_eq!(true, Assistant.in07(&'6'));
        assert_eq!(true, Assistant.in07(&'7'));
        assert_eq!(true, Assistant.in07(&'0'));
        assert_eq!(false, Assistant.in07(&'8'));
        // 字母o
        assert_eq!(false, Assistant.in07(&'o'));


        assert_eq!(true, Assistant.in09(&'8'));
        assert_eq!(true, Assistant.in09(&'9'));
        assert_eq!(true, Assistant.in09(&'0'));
        assert_eq!(false, Assistant.in09(&'A'));


        assert_eq!(true, Assistant.in17(&'6'));
        assert_eq!(false, Assistant.in17(&'8'));
        assert_eq!(true, Assistant.in17(&'1'));
        assert_eq!(false, Assistant.in17(&'0'));
        assert_eq!(false, Assistant.in17(&'A'));

        assert_eq!(true, Assistant.in19(&'8'));
        assert_eq!(true, Assistant.in19(&'9'));
        assert_eq!(true, Assistant.in19(&'1'));
        assert_eq!(false, Assistant.in19(&'0'));
        assert_eq!(false, Assistant.in19(&'A'));


        assert_eq!(true, Assistant.in09af(&'0'));
        assert_eq!(true, Assistant.in09af(&'9'));
        assert_eq!(true, Assistant.in09af(&'1'));
        assert_eq!(true, Assistant.in09af(&'A'));
        assert_eq!(true, Assistant.in09af(&'F'));
        assert_eq!(false, Assistant.in09af(&'Z'));
    }

    #[test]
    fn assistant_blank_bracket() {
        assert_eq!(true, Assistant.is_blank(&' '));
        assert_eq!(true, Assistant.is_blank(&'\n'));
        assert_eq!(false, Assistant.is_blank(&'s'));

        assert_eq!(true, Assistant.is_bracket(&'('));
        assert_eq!(true, Assistant.is_bracket(&'{'));
        assert_eq!(true, Assistant.is_bracket(&'['));
        assert_eq!(false, Assistant.is_bracket(&'a'));
        assert_eq!(true, Assistant.is_bracket(&']'));
        assert_eq!(true, Assistant.is_bracket(&')'));
        assert_eq!(false, Assistant.is_bracket(&'9'));
        assert_eq!(false, Assistant.is_bracket(&' '));
        assert_eq!(true, Assistant.is_bracket(&'}'));
    }
}
