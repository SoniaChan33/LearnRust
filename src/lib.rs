pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[test]
fn feature() {}

mod test {
    use crate::add;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
