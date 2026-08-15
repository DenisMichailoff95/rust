use level10_task46_bank::*;

#[test]
fn test_bank_deposit() {
    let mut acc = BankAccount { owner: String::from("Alice"), balance: 0.0 };
    assert!(acc.deposit(100.0).is_ok());
    assert!((acc.balance - 100.0).abs() < 1e-10);
}

#[test]
fn test_bank_deposit_negative() {
    let mut acc = BankAccount { owner: String::from("Alice"), balance: 0.0 };
    assert!(acc.deposit(-10.0).is_err());
}

#[test]
fn test_bank_withdraw() {
    let mut acc = BankAccount { owner: String::from("Alice"), balance: 100.0 };
    assert!(acc.withdraw(50.0).is_ok());
    assert!((acc.balance - 50.0).abs() < 1e-10);
}

#[test]
fn test_bank_withdraw_insufficient() {
    let mut acc = BankAccount { owner: String::from("Alice"), balance: 10.0 };
    assert!(acc.withdraw(50.0).is_err());
    assert!((acc.balance - 10.0).abs() < 1e-10);
}

#[test]
fn test_bank_withdraw_negative() {
    let mut acc = BankAccount { owner: String::from("Alice"), balance: 100.0 };
    assert!(acc.withdraw(-5.0).is_err());
}
