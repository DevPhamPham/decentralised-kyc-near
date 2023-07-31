// File: COs.rs
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen, AccountId, PanicOnDefault};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};

use crate::Types::{Role, COStatus, KycStatus, DataHashStatus, User, Repre, CO, KycRequest};
use crate::Helpers::Helpers;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct COs {
    pub co_list: Vec<near_sdk::AccountId>,
    pub cos: UnorderedMap<near_sdk::AccountId, CO>,
}


#[near_bindgen]
impl COs {

    // Modifiers
    pub fn is_valid_co(&self, id_: AccountId) {
        assert_ne!(id_, AccountId::new_unchecked("".to_string()), "CO not found");
        let co = self.cos.get(&id_).expect("CO not found");
        assert_eq!(co.id_, id_, "co not found");
        assert_eq!(co.status, COStatus::Active, "CO is not active");
    }

    // Contract Methods

    pub fn get_all_cos(&self, page_number: u32) -> (u32, Vec<CO>) {
        assert!(page_number > 0, "PN should be > 0");
        let (pages, page_length, start_index, end_index) =
            Helpers::get_indexes(page_number, self.co_list.clone());
    
        let mut cos_list = Vec::with_capacity(page_length as usize);
        for i in start_index..end_index {
            let co_id = &self.co_list[i as usize];
            let co = self.cos.get(co_id).expect("CO not found");
            cos_list.push(co);
        }
    
        (pages, cos_list)
    }
    
    // pub fn get_single_co(&self, id_: AccountId) -> CO {
    //     assert_ne!(id_, AccountId::new_unchecked("".to_string()), "CO Id Empty");
    //     let co = self.cos.get(&id_).expect("CO not found");
    //     co
    // }
    pub fn get_single_co(&self, id_: AccountId) -> Option<CO> {
        assert_ne!(id_, AccountId::new_unchecked("".to_string()), "CO Id Empty");
        self.cos.get(&id_)
    }    
    
    pub fn add_co(&mut self, co: CO) {
        let id_ = co.id_.clone();
        assert!(self.cos.insert(&id_, &co).is_none(), "CO exists");
        self.co_list.push(id_.clone());
        log!(
            "COAdded: {:?}, {:?}, {:?}, {:?}",
            id_,
            co.name,
            co.email,
            co.npoid_code
        );
    }
    
    pub fn update_co(&mut self, id_: AccountId, email_: String, name_: String) {
        assert_ne!(id_, AccountId::new_unchecked("".to_string()), "CO not found");
    
        if let Some(mut co) = self.cos.get(&id_) {
            co.name = name_.clone();
            co.email = email_.clone();
            self.cos.insert(&id_, &co);
            log!("COUpdated: {:?}, {:?}, {:?}", id_, name_, email_);
        } else {
            log!("CO not found");
        }
    }
    
    pub fn activate_deactivate_co(&mut self, id_: AccountId, make_active_: bool) -> COStatus {
        assert_ne!(id_, AccountId::new_unchecked("".to_string()), "CO not found");
    
        if let Some(mut co) = self.cos.get(&id_) {
            if make_active_ && co.status == COStatus::Inactive {
                co.status = COStatus::Active;
                self.cos.insert(&id_, &co);
                log!("COActivated: {:?}, {:?}", id_, co.name);
                COStatus::Active
            } else if !make_active_ && co.status == COStatus::Active {
                co.status = COStatus::Inactive;
                self.cos.insert(&id_, &co);
                log!("CODeactivated: {:?}, {:?}", id_, co.name);
                COStatus::Inactive
            } else {
                co.status
            }
        } else {
            log!("CO not found");
            COStatus::Inactive
        }
    }
    
    pub fn update_kyc_count(&mut self, id_: AccountId) {
        assert_ne!(id_, AccountId::new_unchecked("".to_string()), "CO not found");
    
        if let Some(mut co) = self.cos.get(&id_) {
            co.kyc_count += 1;
            self.cos.insert(&id_, &co);
        } else {
            log!("CO not found");
        }
    }
    
}
