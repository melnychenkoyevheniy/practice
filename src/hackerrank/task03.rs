// https://www.hackerrank.com/challenges/staircase/problem
fn staircase_line(n: i32, i: i32) -> String {
    format!("{}{}", " ".repeat((n - i) as usize), "#".repeat(i as usize))
}

#[allow(dead_code)]
fn staircase(n: i32) {
    for i in 1..=n {
        println!("{}", staircase_line(n, i));
    }
}

#[test]
fn test0() {
    assert_eq!(staircase_line(4, 1), "   #");
    assert_eq!(staircase_line(4, 2), "  ##");
    assert_eq!(staircase_line(4, 3), " ###");
    assert_eq!(staircase_line(4, 4), "####");
}