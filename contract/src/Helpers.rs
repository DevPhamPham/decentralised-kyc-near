// File: helpers.rs
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, AccountId};

// #[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
// #[serde(crate = "near_sdk::serde")]
pub struct Helpers {}

// #[near_bindgen]
impl Helpers {
    // Contract Methods

    pub fn get_indexes(page_number: u32, users: Vec<AccountId>) -> (u32, u32, u32, u32) {
        let reminder = (users.len() % 25) as u32;
        let pages = users.len() as u32 / 25 + if reminder > 0 { 1 } else { 0 };
    
        let page_length = 25;
        let start_index = 25 * (page_number - 1);
        let end_index = 25 * page_number;
    
        let (page_length, end_index) = if page_number > pages {
            (0, 0)
        } else if page_number == pages && reminder > 0 {
            (reminder, users.len() as u32)
        } else {
            (page_length, end_index)
        };
    
        (pages, page_length, start_index, end_index)
    }
    
    pub fn append(a: AccountId, b: AccountId) -> AccountId {
        AccountId::new_unchecked(format!("{}{}", a.clone(), b.clone()).into())
    }
}
