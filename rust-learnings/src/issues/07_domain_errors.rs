// ============================================================
// ISSUE 7 — Domain Errors
// Concept: Error Handling with `thiserror`
// Difficulty: Intermediate
//
// Your Task:
// Instead of using generic Strings or Option types, idiomatic Rust
// uses custom Error enums. Using the `thiserror` crate, define a
// domain error for a banking application.
//
// TODO: Implement the `BankError` enum using the `#[derive(thiserror::Error)]` macro.
// TODO: Refactor the `withdraw` function to return `Result<u64, BankError>`.
//
// Resources: https://docs.rs/thiserror/latest/thiserror/
// ============================================================

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum BankError {
    // TODO: Add #[error("...")] variants for:
    // 1. InsufficientFunds (should show current balance and requested amount)
    // 2. AccountClosed
}

pub struct Account {
    pub balance: u64,
    pub is_closed: bool,
}

pub fn withdraw(account: &mut Account, amount: u64) -> Result<u64, String> {
    if account.is_closed {
        return Err("Account is closed".to_string());
    }

    if account.balance < amount {
        return Err(format!("Insufficient funds: balance is {}, requested {}", account.balance, amount));
    }

    account.balance -= amount;
    Ok(account.balance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insufficient_funds() {
        let mut acc = Account { balance: 50, is_closed: false };
        // let res = withdraw(&mut acc, 100);
        // assert_eq!(res, Err(BankError::InsufficientFunds { balance: 50, amount: 100 }));
    }
}
