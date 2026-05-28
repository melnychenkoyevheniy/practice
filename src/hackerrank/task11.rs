// https://www.hackerrank.com/challenges/diagonal-difference/problem
fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut primary = 0;
    let mut secondary = 0;
    for i in 0..n {
        primary += arr[i][i];
        secondary += arr[i][n - 1 - i];
    }
    (primary - secondary).abs()
}

#[test]
fn test0() {
    let arr = vec![
        vec![11, 2, 4],
        vec![4, 5, 6],
        vec![10, 8, -12],
    ];
    assert_eq!(diagonal_difference(&arr), 15);
}