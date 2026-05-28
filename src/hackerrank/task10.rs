// https://www.hackerrank.com/challenges/sock-merchant/problem
fn sock_merchant(_n: i32, ar: &[i32]) -> i32 {
    let mut counts = std::collections::HashMap::new();
    for &s in ar {
        *counts.entry(s).or_insert(0) += 1;
    }
    counts.values().map(|&c| c / 2).sum()
}

#[test]
fn test0() {
    assert_eq!(sock_merchant(9, &[10, 20, 20, 10, 10, 30, 50, 10, 20]), 3);
}