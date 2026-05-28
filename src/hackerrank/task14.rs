// https://www.hackerrank.com/challenges/bon-appetit/problem
fn bon_appetit_result(bill: &[i32], k: i32, b: i32) -> String {
    let actual: i32 = bill.iter().enumerate()
        .filter(|&(i, _)| i as i32 != k)
        .map(|(_, &v)| v)
        .sum::<i32>() / 2;
    if b == actual {
        "Bon Appetit".to_string()
    } else {
        (b - actual).to_string()
    }
}

#[allow(dead_code)]
fn bon_appetit(bill: &[i32], k: i32, b: i32) {
    println!("{}", bon_appetit_result(bill, k, b));
}

#[test]
fn test0() {
    assert_eq!(bon_appetit_result(&[3, 10, 2, 9], 1, 12), "5");
    assert_eq!(bon_appetit_result(&[3, 10, 2, 9], 1, 7), "Bon Appetit");
}