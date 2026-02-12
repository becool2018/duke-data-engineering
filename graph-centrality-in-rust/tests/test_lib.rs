use trust::add;
use trust::div;
use trust::mul;
use trust::sub;

#[test]
pub fn test_add() {
    assert_eq!(add(1, 2), 3);
}

#[test]
pub fn test_sub() {
    assert_eq!(sub(1, 2), -1);
}

#[test]
pub fn test_mul() {
    assert_eq!(mul(2, 2), 4);
}

#[test]
pub fn test_div() {
    assert_eq!(div(4, 2), 2);
}
