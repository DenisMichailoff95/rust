use level4_task18_student::*;

#[test]
fn test_student_average() {
    let s = Student {
        name: String::from("Alice"),
        grades: vec![5, 4, 3],
    };
    assert!((s.average_grade() - 4.0).abs() < 1e-10);
}

#[test]
fn test_add_grade() {
    let mut s = Student {
        name: String::from("Alice"),
        grades: vec![5, 4],
    };
    s.add_grade(3);
    assert_eq!(s.grades, vec![5, 4, 3]);
}
