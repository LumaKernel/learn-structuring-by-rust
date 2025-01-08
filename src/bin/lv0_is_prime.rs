fn is_prime(n: u64) -> bool {
    todo!()
}

#[test]
fn test_is_prime_0() {
    assert!(!is_prime(0));
    assert!(!is_prime(1));
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(!is_prime(4));
    assert!(is_prime(5));
    assert!(!is_prime(6));
    assert!(is_prime(7));
    assert!(!is_prime(8));
    assert!(!is_prime(9));
    assert!(!is_prime(10));
    assert!(is_prime(11));
    assert!(!is_prime(100));
    assert!(is_prime(131));
}

#[test]
fn test_is_prime_ex() {
    assert!(is_prime(1000000007));
    assert!(!is_prime(10000000000000007));
    assert!(!is_prime(39817941929192493));
    assert!(is_prime(39817941929192509));
    assert!(is_prime(69617748183112201));
}

fn main() {
    println!("{}", is_prime(11));
}
