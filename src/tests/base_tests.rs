#[cfg(test)]
mod base_tests {
    use crate::base::base::PPUTKIterator;

    #[test]
    fn base_test1() {
        let src = "ABCDEFGHIJKLNM".chars().collect::<Vec<char>>();
        let mut ppuk = PPUTKIterator::new(src.iter());

        assert_eq!(ppuk.next().unwrap(), &'A');
        assert_eq!(ppuk.next().unwrap(), &'B');
        assert_eq!(ppuk.next().unwrap(), &'C');
        assert_eq!(ppuk.next().unwrap(), &'D');
        assert_eq!(ppuk.next().unwrap(), &'E');

        ppuk.put_back();
        ppuk.put_back();
        ppuk.put_back();

        assert_eq!(ppuk.peek().unwrap(), &'C');
        assert_eq!(ppuk.peek().unwrap(), &'C');
        assert_eq!(ppuk.peek().unwrap(), &'C');

        assert_eq!(ppuk.next().unwrap(), &'C');
        assert_eq!(ppuk.next().unwrap(), &'D');
        assert_eq!(ppuk.next().unwrap(), &'E');

        assert_eq!(ppuk.next().unwrap(), &'F');
        assert_eq!(ppuk.peek().unwrap(), &'G');
        ppuk.put_back();
        assert_eq!(ppuk.peek().unwrap(), &'F');
        assert_eq!(ppuk.next().unwrap(), &'F');

        while let Some(s) = ppuk.peek() {
            println!("{:?}", s);
            ppuk.next();
        }
    }
}
