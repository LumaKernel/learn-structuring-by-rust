fn square_sum(v: &[i64]) -> u64 {
    todo!()
}

#[test]
fn test_square_sum() {
    assert_eq!(square_sum(&vec![1, 2, 3]), 14);
    assert_eq!(square_sum(&vec![-1, 9, 4, -6]), 134);
    assert_eq!(square_sum(&vec![999, -111, 3342, 5455]), 41936311);
    assert_eq!(square_sum(&vec![]), 0);
    assert_eq!(square_sum(&vec![1]), 1);
    assert_eq!(square_sum(&vec![-1]), 1);
    assert_eq!(square_sum(&(0..999).collect::<Vec<_>>()), 331835499);
}

fn main() {
    println!("{}", square_sum(&vec![1, 2, 3]));
}
