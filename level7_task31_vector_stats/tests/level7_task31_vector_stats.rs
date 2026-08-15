use level7_task31_vector_stats::*;

#[test]
fn test_vector_stats() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let (min, max, sum, avg) = vector_stats(v);
    assert_eq!(min, 1);
    assert_eq!(max, 10);
    assert_eq!(sum, 55);
    assert!((avg - 5.5).abs() < 1e-10);
}

#[test]
fn test_vector_stats_single() {
    let v = vec![42];
    let (min, max, sum, avg) = vector_stats(v);
    assert_eq!(min, 42);
    assert_eq!(max, 42);
    assert_eq!(sum, 42);
    assert!((avg - 42.0).abs() < 1e-10);
}
