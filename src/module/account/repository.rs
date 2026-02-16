// Copyright 2026 MineHighVN, AXLauncher contributors
// SPDX-License-Identifier: Apache-2.0

use std::sync::Mutex;

use crate::module::account::entity::Account;

// Internal struct
struct RepositoryData {
    accounts: Vec<Account>,
    active_account: Option<Account>,
}

pub struct AccountRepository {
    data: Mutex<RepositoryData>,
}

/// TOOD: implements save user accounts in file instend of memory
impl AccountRepository {
    pub fn new() -> Self {
        Self {
            data: Mutex::new(RepositoryData {
                accounts: Vec::new(),
                active_account: None,
            }),
        }
    }

    /// Get all account
    /// Returns vector of accounts
    pub fn get_all(&self) -> Vec<Account> {
        let lock = self.data.lock().unwrap();
        lock.accounts.clone()
    }

    // Add new account
    pub fn add(&self, account: Account) {
        let mut lock = self.data.lock().unwrap();
        lock.accounts.push(account);
    }

    // Delete account
    pub fn remove(&self, account: &Account) {
        let mut lock = self.data.lock().unwrap();
        lock.accounts.retain(|x| x != account);

        // Reset active if deleted account is active
        if lock.active_account.as_ref() == Some(account) {
            lock.active_account = None;
        }
    }

    // Get active account
    pub fn get_active(&self) -> Option<Account> {
        let lock = self.data.lock().unwrap();
        lock.active_account.clone()
    }

    // Set active account
    pub fn set_active(&self, account: Option<Account>) {
        let mut lock = self.data.lock().unwrap();
        lock.active_account = account;
    }
}
