// File: repres.rs
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, env, near_bindgen, AccountId,PanicOnDefault};
use near_sdk::collections::{LookupMap, Vector};

use crate::Types::{Role, COStatus, KycStatus, DataHashStatus, User, Repre};
// use crate::Helpers::Helpers;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Repres {
    pub repre_list: Vector<AccountId>,
    pub repres: LookupMap<AccountId, Repre>,
}

#[near_bindgen]
impl Repres {
    // Events

    // Modifiers
    pub fn is_valid_repre(&self, id_: AccountId) {
        assert_ne!(id_, env::current_account_id(), "Id is Empty");
        assert!(self.repres.contains_key(&id_), "User Id Empty");
        assert!(!self.repres.get(&id_).unwrap().email.is_empty(), "User Email Empty");
    }

    // Support Functions
    pub fn repre_exists(&self, id_: AccountId) -> bool {
        assert_ne!(id_, env::current_account_id(), "Id is Empty");
        if let Some(repre) = self.repres.get(&id_) {
            !repre.email.is_empty()
        } else {
            false
        }
    }

    // Contract Functions
    pub fn get_repre_details(&self, id_: AccountId) -> Repre {
        self.repres.get(&id_).expect("Repre does not exist")
    }

    pub fn update_profile(&mut self, name_: String, email_: String, mobile_: u32) {
        let id_ = env::signer_account_id();
        if let Some(mut repre) = self.repres.get(&id_) {
            repre.name = name_.clone();
            repre.email = email_.clone();
            repre.mobile_number = mobile_.clone();
            self.repres.insert(&id_, &repre);
            log!("RepreDataUpdated: {:?}, {:?}, {:?}", id_, name_, email_);
        }
    }

    pub fn add_repre(&mut self, repre_: Repre) {
        let id_ = repre_.id_.clone();
        self.repres.insert(&id_, &repre_);
        self.repre_list.push(&id_);
        log!("RepreAdded: {:?}, {:?}, {:?}", id_, repre_.name, repre_.email);
    }

    pub fn update_kyc_done_by(&mut self, id_: AccountId) {
        assert_ne!(id_, env::current_account_id(), "Repre Id Empty");
        if let Some(mut repre) = self.repres.get(&id_) {
            repre.kyc_verified_by = env::signer_account_id();
            self.repres.insert(&id_, &repre);
        }
    }

    pub fn update_data_hash(&mut self, hash_: String, current_time_: u32) {
        let id_ = env::signer_account_id();
        if let Some(mut repre) = self.repres.get(&id_) {
            repre.data_hash = hash_.clone();
            repre.data_updated_on = current_time_.clone();
            self.repres.insert(&id_, &repre);
            log!("DataHashUpdated: {:?}, {:?}, {:?}", id_, repre.name, hash_);
        }
    }

    pub fn search_repres(
        &self,
        id_: AccountId,
        repres_: Vec<AccountId>,
    ) -> (bool, Repre) {
        let mut found_ = false;
        let mut repre_: Repre = Repre {
            name: "".to_string(),
            email: "".to_string(),
            id_: AccountId::new_unchecked("".to_string()),
            mobile_number: 0,
            kyc_verified_by: AccountId::new_unchecked("".to_string()), // Chỉ cần truyền giá trị rỗng của AccountId
            data_hash: "".to_string(),
            data_updated_on: 0,
        };

        for repre_id in repres_.iter() {
            if repre_id == &id_ {
                found_ = true;
                if let Some(found_repre) = self.repres.get(&id_) {
                    repre_ = found_repre;
                }
                break;
            }
        }
        (found_, repre_)
    }
}