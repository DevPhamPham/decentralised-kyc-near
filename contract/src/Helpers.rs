// File: helpers.rs
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, AccountId};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
// #[serde(crate = "near_sdk::serde")]
pub struct Helpers {}

#[near_bindgen]
impl Helpers {
    // Contract Methods

    /// List of customers, who are linked to the current bank.
    /// Data will be sent in pages to avoid the more gas fee.
    /// pageNumber: page number for which data is needed (1, 2, 3, ..., n)
    /// users_: User Id's who are linked to the requested bank
    /// Returns: Total pages available and List of banks in the current page
    pub fn get_indexes(page_number: u32, users: Vec<AccountId>) -> (u32, u32, u32, u32) {
        let reminder = (users.len() % 25) as u32;
        let pages = users.len() as u32 / 25 + if reminder > 0 { 1 } else { 0 };
    
        let page_length = 25;
        let start_index = 25 * (page_number - 1);
        let end_index = 25 * page_number;
    
        let (page_length, end_index) = if page_number > pages {
            // Page requested is not existing
            (0, 0)
        } else if page_number == pages && reminder > 0 {
            // Last page where we don't have 25 records
            (reminder, users.len() as u32)
        } else {
            (page_length, end_index)
        };
    
        (pages, page_length, start_index, end_index)
    }
    
    /// Internal function which doesn't alter any stage or read any data.
    /// Used to compare the string operations.
    /// a: string-1 that is to be compared
    /// b: string-2 that is to be compared
    /// Returns: boolean value to say if both strings matched or not
    // pub fn compare_strings(&self, a: String, b: String) -> bool {
    //     a == b
    // }

    /// Internal function used to concatenate two addresses.
    /// a: address-1
    /// b: address-2 that needs to be appended
    /// Returns: string value after concatenating
    pub fn append(a: AccountId, b: AccountId) -> AccountId {
        AccountId::new_unchecked(format!("{}{}", a.clone(), b.clone()).into())
    }
}
