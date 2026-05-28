// https://www.hackerrank.com/challenges/apple-and-orange/problem
fn count_in_range(tree: i32, s: i32, t: i32, fruits: &[i32]) -> usize {
    fruits.iter().filter(|&&d| (tree + d) >= s && (tree + d) <= t).count()
}

#[allow(dead_code)]
fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    println!("{}", count_in_range(a, s, t, apples));
    println!("{}", count_in_range(b, s, t, oranges));
}

#[test]
fn test0() {
    assert_eq!(count_in_range(5, 7, 11, &[-2, 2, 1]), 1);
    assert_eq!(count_in_range(15, 7, 11, &[5, -6]), 1);
}