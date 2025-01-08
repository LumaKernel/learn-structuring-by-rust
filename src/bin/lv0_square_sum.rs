fn square_sum(v: &[i64]) -> i64 {
    todo!()
}

#[test]
fn test_square_sum() {
    assert_eq!(square_sum(&vec![1, 2, 3]), 14);
    assert_eq!(square_sum(&vec![-1, 9, 4, -6]), 122);
}

fn main() {
    println!("{}", square_sum(&vec![1, 2, 3]));
}
