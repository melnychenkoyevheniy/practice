// https://www.hackerrank.com/challenges/kangaroo/problem
fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> &'static str {
    if v1 == v2 {
        if x1 == x2 { "YES" } else { "NO" }
    } else if (x2 - x1) % (v1 - v2) == 0 && (x2 - x1) / (v1 - v2) > 0 {
        "YES"
    } else {
        "NO"
    }
}

#[test]
fn test0() {
    assert_eq!(kangaroo(0, 3, 4, 2), "YES");
    assert_eq!(kangaroo(0, 2, 5, 3), "NO");
}