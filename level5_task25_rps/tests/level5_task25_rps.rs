use level5_task25_rps::*;

#[test]
fn test_beats() {
    assert!(beats(Move::Rock, Move::Scissors));
    assert!(beats(Move::Paper, Move::Rock));
    assert!(beats(Move::Scissors, Move::Paper));
    assert!(!beats(Move::Rock, Move::Paper));
}
