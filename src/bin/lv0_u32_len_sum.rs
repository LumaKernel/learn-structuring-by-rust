// 各自然数の桁数の和
// The sum of each number's length in decimal.
fn u32_len_sum(v: &[u32]) -> usize {
    todo!()
}

#[test]
fn test_u32_len_sum() {
    assert_eq!(u32_len_sum(&vec![1, 2, 3]), 3);
    assert_eq!(u32_len_sum(&vec![13, 0, 13482]), 8);
    assert_eq!(u32_len_sum(&vec![1000000000, 1000000000, 1000000000]), 30);
    assert_eq!(u32_len_sum(&(0..10000).collect::<Vec<u32>>()), 38890);
}

fn main() {
    println!("{}", u32_len_sum(&vec![1, 2, 3]));
}
