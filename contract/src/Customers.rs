// File: customers.rs
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, env, near_bindgen, AccountId,PanicOnDefault};
use near_sdk::collections::{LookupMap, Vector};

use crate::Types::{Role, BankStatus, KycStatus, DataHashStatus, User, Customer};
// use crate::Helpers::Helpers;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Customers {
    pub customer_list: Vector<AccountId>,
    pub customers: LookupMap<AccountId, Customer>,
}

#[near_bindgen]
impl Customers {
    // Events

    // Modifiers
    pub fn is_valid_customer(&self, id_: AccountId) {
        assert_ne!(id_, env::current_account_id(), "Id is Empty");
        assert!(self.customers.contains_key(&id_), "User Id Empty");
        assert!(!self.customers.get(&id_).unwrap().email.is_empty(), "User Email Empty");
    }

    // Support Functions
    pub fn customer_exists(&self, id_: AccountId) -> bool {
        assert_ne!(id_, env::current_account_id(), "Id is Empty");
        if let Some(customer) = self.customers.get(&id_) {
            !customer.email.is_empty()
        } else {
            false
        }
    }

    // Contract Functions
    pub fn get_customer_details(&self, id_: AccountId) -> Customer {
        self.customers.get(&id_).expect("Customer does not exist")
    }

    pub fn update_profile(&mut self, name_: String, email_: String, mobile_: u32) {
        let id_ = env::signer_account_id();
        if let Some(mut customer) = self.customers.get(&id_) {
            customer.name = name_.clone();
            customer.email = email_.clone();
            customer.mobile_number = mobile_.clone();
            self.customers.insert(&id_, &customer);
            log!("CustomerDataUpdated: {:?}, {:?}, {:?}", id_, name_, email_);
        }
    }

    pub fn add_customer(&mut self, customer_: Customer) {
        let id_ = customer_.id_.clone();
        self.customers.insert(&id_, &customer_);
        self.customer_list.push(&id_);
        log!("CustomerAdded: {:?}, {:?}, {:?}", id_, customer_.name, customer_.email);
    }

    pub fn update_kyc_done_by(&mut self, id_: AccountId) {
        assert_ne!(id_, env::current_account_id(), "Customer Id Empty");
        if let Some(mut customer) = self.customers.get(&id_) {
            customer.kyc_verified_by = env::signer_account_id();
            self.customers.insert(&id_, &customer);
        }
    }

    pub fn update_data_hash(&mut self, hash_: String, current_time_: u32) {
        let id_ = env::signer_account_id();
        if let Some(mut customer) = self.customers.get(&id_) {
            customer.data_hash = hash_.clone();
            customer.data_updated_on = current_time_.clone();
            self.customers.insert(&id_, &customer);
            log!("DataHashUpdated: {:?}, {:?}, {:?}", id_, customer.name, hash_);
        }
    }

    pub fn search_customers(
        &self,
        id_: AccountId,
        customers_: Vec<AccountId>,
    ) -> (bool, Customer) {
        let mut found_ = false;
        let mut customer_: Customer = Customer {
            name: "".to_string(),
            email: "".to_string(),
            id_: AccountId::new_unchecked("".to_string()),
            mobile_number: 0,
            kyc_verified_by: AccountId::new_unchecked("".to_string()), // Chỉ cần truyền giá trị rỗng của AccountId
            data_hash: "".to_string(),
            data_updated_on: 0,
        };

        for customer_id in customers_.iter() {
            if customer_id == &id_ {
                found_ = true;
                if let Some(found_customer) = self.customers.get(&id_) {
                    customer_ = found_customer;
                }
                break;
            }
        }
        (found_, customer_)
    }
}