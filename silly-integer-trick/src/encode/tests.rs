use super::*;

#[test]
fn test_round_trip() {
    assert_eq!(encode(3, 5), 1944);
    assert_eq!(decode(1944), (3, 5));
}
