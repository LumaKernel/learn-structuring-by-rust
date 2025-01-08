fn is_prime(n: u64) -> bool {
    todo!()
}

#[test]
fn test_is_prime_0() {
    assert_eq!(is_prime(0), false);
    assert_eq!(is_prime(1), false);
    assert_eq!(is_prime(2), true);
    assert_eq!(is_prime(3), true);
    assert_eq!(is_prime(4), false);
    assert_eq!(is_prime(5), true);
    assert_eq!(is_prime(6), false);
    assert_eq!(is_prime(7), true);
    assert_eq!(is_prime(8), false);
    assert_eq!(is_prime(9), false);
    assert_eq!(is_prime(10), false);
    assert_eq!(is_prime(11), true);
    assert_eq!(is_prime(100), false);
    assert_eq!(is_prime(131), true);
}

#[test]
fn test_is_prime_ex() {
    assert_eq!(is_prime(1000000007), true);
    assert_eq!(is_prime(10000000000000007), false);
    assert_eq!(is_prime(39817941929192493), false);
    assert_eq!(is_prime(39817941929192509), true);
    assert_eq!(is_prime(69617748183112201), true);
}

fn main() {
    println!("{}", is_prime(11));
}
