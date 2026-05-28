// https://www.hackerrank.com/challenges/grading/problem
fn grade_student(grade: i32) -> i32 {
    let next_multiple = (grade / 5 + 1) * 5;
    if grade < 38 || next_multiple - grade >= 3 {
        grade
    } else {
        next_multiple
    }
}

#[allow(dead_code)]
fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades.iter().map(|&g| grade_student(g)).collect()
}

#[test]
fn test0() {
    let input = vec![73, 67, 38, 33];
    let real = grading_students(&input);
    let expected = vec![75, 67, 40, 33];
    assert_eq!(real, expected);
}