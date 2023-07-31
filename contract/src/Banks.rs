// File: banks.rs
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen, AccountId, PanicOnDefault};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};

use crate::Types::{Role, BankStatus, KycStatus, DataHashStatus, User, Customer, Bank, KycRequest};
use crate::Helpers::Helpers;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Banks {
    pub bank_list: Vec<near_sdk::AccountId>,
    pub banks: UnorderedMap<near_sdk::AccountId, Bank>,
}


#[near_bindgen]
impl Banks {

    // Modifiers
    pub fn is_valid_bank(&self, id_: AccountId) {
        assert_ne!(id_, AccountId::new_unchecked("".to_string()), "Bank not found");
        let bank = self.banks.get(&id_).expect("Bank not found");
        assert_eq!(bank.id_, id_, "Bank not found");
        assert_eq!(bank.status, BankStatus::Active, "Bank is not active");
    }

    // Contract Methods

    pub fn get_all_banks(&self, page_number: u32) -> (u32, Vec<Bank>) {
        assert!(page_number > 0, "PN should be > 0");
        let (pages, page_length, start_index, end_index) =
            Helpers::get_indexes(page_number, self.bank_list.clone());
    
        let mut banks_list = Vec::with_capacity(page_length as usize);
        for i in start_index..end_index {
            let bank_id = &self.bank_list[i as usize];
            let bank = self.banks.get(bank_id).expect("Bank not found");
            banks_list.push(bank);
        }
    
        (pages, banks_list)
    }
    
    // pub fn get_single_bank(&self, id_: AccountId) -> Bank {
    //     assert_ne!(id_, AccountId::new_unchecked("".to_string()), "Bank Id Empty");
    //     let bank = self.banks.get(&id_).expect("Bank not found");
    //     bank
    // }
    pub fn get_single_bank(&self, id_: AccountId) -> Option<Bank> {
        assert_ne!(id_, AccountId::new_unchecked("".to_string()), "Bank Id Empty");
        self.banks.get(&id_)
    }    
    
    pub fn add_bank(&mut self, bank: Bank) {
        let id_ = bank.id_.clone();
        assert!(self.banks.insert(&id_, &bank).is_none(), "Bank exists");
        self.bank_list.push(id_.clone());
        log!(
            "BankAdded: {:?}, {:?}, {:?}, {:?}",
            id_,
            bank.name,
            bank.email,
            bank.npoid_code
        );
    }
    
    pub fn update_bank(&mut self, id_: AccountId, email_: String, name_: String) {
        assert_ne!(id_, AccountId::new_unchecked("".to_string()), "Bank not found");
    
        if let Some(mut bank) = self.banks.get(&id_) {
            bank.name = name_.clone();
            bank.email = email_.clone();
            self.banks.insert(&id_, &bank);
            log!("BankUpdated: {:?}, {:?}, {:?}", id_, name_, email_);
        } else {
            log!("Bank not found");
        }
    }
    
    pub fn activate_deactivate_bank(&mut self, id_: AccountId, make_active_: bool) -> BankStatus {
        assert_ne!(id_, AccountId::new_unchecked("".to_string()), "Bank not found");
    
        if let Some(mut bank) = self.banks.get(&id_) {
            if make_active_ && bank.status == BankStatus::Inactive {
                bank.status = BankStatus::Active;
                self.banks.insert(&id_, &bank);
                log!("BankActivated: {:?}, {:?}", id_, bank.name);
                BankStatus::Active
            } else if !make_active_ && bank.status == BankStatus::Active {
                bank.status = BankStatus::Inactive;
                self.banks.insert(&id_, &bank);
                log!("BankDeactivated: {:?}, {:?}", id_, bank.name);
                BankStatus::Inactive
            } else {
                bank.status
            }
        } else {
            log!("Bank not found");
            BankStatus::Inactive
        }
    }
    
    pub fn update_kyc_count(&mut self, id_: AccountId) {
        assert_ne!(id_, AccountId::new_unchecked("".to_string()), "Bank not found");
    
        if let Some(mut bank) = self.banks.get(&id_) {
            bank.kyc_count += 1;
            self.banks.insert(&id_, &bank);
        } else {
            log!("Bank not found");
        }
    }
    
}
