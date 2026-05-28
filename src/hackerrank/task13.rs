// https://www.hackerrank.com/challenges/divisible-sum-pairs/problem
fn divisible_sum_pairs(_n: i32, k: i32, ar: &[i32]) -> i32 {
    let mut count = 0;
    for i in 0..ar.len() {
        for j in (i + 1)..ar.len() {
            if (ar[i] + ar[j]) % k == 0 {
                count += 1;
            }
        }
    }
    count
}

#[test]
fn test0() {
    assert_eq!(divisible_sum_pairs(6, 3, &[1, 3, 2, 6, 1, 2]), 5);
}