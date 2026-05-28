// https://www.hackerrank.com/challenges/birthday-cake-candles/problem
fn birthday_cake_candles(candles: &[i32]) -> i32 {
    let max = *candles.iter().max().unwrap();
    candles.iter().filter(|&&c| c == max).count() as i32
}

#[test]
fn test0() {
    assert_eq!(birthday_cake_candles(&[3, 2, 1, 3]), 2);
}