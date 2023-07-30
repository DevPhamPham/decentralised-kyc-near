use std::ptr::null;

// File: kyc.rs
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet, Vector};
use near_sdk::{log, near_bindgen, Balance, Promise, env, AccountId, PanicOnDefault};
// use near_sdk::serde::{Deserialize, Serialize};

use crate::Types::{Role, BankStatus, KycStatus, DataHashStatus, User, Customer, Bank, KycRequest};
use crate::Customers::Customers;
use crate::Banks::Banks;
use crate::Helpers::Helpers;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
// #[warn(non_snake_case)]
pub struct KYC {
    admin: AccountId,
    user_list: UnorderedSet<AccountId>,
    users: LookupMap<AccountId, User>,
    kyc_requests: LookupMap<String, KycRequest>,
    bank_customers: LookupMap<AccountId, Vec<AccountId>>,
    customer_banks: LookupMap<AccountId, Vec<AccountId>>,
    banks: Banks,
    customers: Customers,
}

#[near_bindgen]
impl KYC {
    // Creates a new instance of KYC contract with the provided admin details.
    #[init]
    pub fn new(name_: String, email_: String) -> Self {
        let admin = env::predecessor_account_id();
        let mut user_list = UnorderedSet::new(b"u".to_vec());
        user_list.insert(&admin);

        let user = User {
            name: name_,
            email: email_,
            id_: admin.clone(),
            role: Role::Admin,
            status: BankStatus::Active,
        };

        let mut users = LookupMap::new(b"u");
        users.insert(&admin, &user);

        KYC {
            admin,
            user_list,
            users,
            kyc_requests: LookupMap::new(b"kr".try_to_vec().unwrap()),
            bank_customers: LookupMap::new(b"bc".try_to_vec().unwrap()),
            customer_banks: LookupMap::new(b"cb".try_to_vec().unwrap()),
            banks: Banks::default(),
            customers: Customers::default(),
        }
    }

    // Modifiers

    pub fn is_admin(&self){
        assert_eq!(env::predecessor_account_id(), self.admin.clone(), "Only admin is allowed");
    }

    // // Support functions

    pub fn kyc_request_exists(&self, req_id_: String) -> bool {
        assert!(!req_id_.is_empty(), "Request Id empty");
        self.kyc_requests.contains_key(&req_id_)
    }

    #[allow(unused_variables)]
    pub fn get_kyc_requests(
        &self,
        page_number: u32,
        is_for_bank: bool,
    ) -> (u32, Vec<KycRequest>) {
        let page_number_u32: u32 = page_number.into();
        assert!(page_number_u32 > 0, "PN should be > zero");
        let users = if is_for_bank {
            self.bank_customers.get(&env::signer_account_id()).unwrap()
        } else {
            self.customer_banks.get(&env::signer_account_id()).unwrap()
        };
        let (pages, page_length, start_index, end_index) = Helpers::get_indexes(page_number_u32, users.clone());
        let mut list: Vec<KycRequest> = Vec::with_capacity(page_length as usize);
        for i in start_index..end_index {
            let key = if is_for_bank {
                Helpers::append(env::signer_account_id(), users[i as usize].clone())
            } else {
                Helpers::append(users[i as usize].clone(), env::signer_account_id())
            };            
            list.push(self.kyc_requests.get(&key.to_string()).unwrap());
        }
        (pages.into(), list)
    }

    // Admin Interface
    pub fn get_all_bank_kyc(&self, page_number: u32) -> (u32, Vec<Bank>) {
        // let banks_contract = Banks::default(); 
        self.banks.get_all_banks(page_number) 
    }

    pub fn add_bank_kyc(&mut self, bank: Bank) {
        assert!(env::predecessor_account_id() == self.admin, "Only admin is allowed");
        
        // Call the add_bank function from the Banks contract
        // let mut banks_contract = Banks::default(); 
        self.banks.add_bank(bank.clone());

        // Adding to common list
        self.users.insert(
            &bank.id_,
            &User {
                name: bank.name.clone(),
                email: bank.email.clone(),
                id_: bank.id_.clone(),
                role: Role::Bank,
                status: BankStatus::Active,
            },
        );

        self.user_list.insert(&bank.id_);
    }

    pub fn update_bank_details(&mut self, id_: AccountId, email_: String, name_: String) {
        assert!(env::predecessor_account_id() == self.admin, "Only admin is allowed");

        // Call the update_bank function from the Banks contract
        // let mut banks_contract = Banks::default(); 
        self.banks.update_bank(id_.clone(), email_.clone(), name_.clone());

        // Updating in common list
        if let Some(mut user) = self.users.get(&id_) {
            user.name = name_;
            user.email = email_;
        }
    }

    pub fn activate_deactivate_bank_kyc(&mut self, id_: AccountId, make_active_: bool) {
        assert!(env::predecessor_account_id() == self.admin, "Only admin is allowed");

        // Call the activate_deactivate_bank function from the Banks contract
        // let mut banks_contract = Banks::default(); 
        let new_status = self.banks.activate_deactivate_bank(id_.clone(), make_active_);

        // Updating in common list
        if let Some(mut user) = self.users.get(&id_) {
            user.status = new_status;
        }
    }

    // Bank Interface

    pub fn get_customers_of_bank(&self, page_number: u32) -> (u32, Vec<KycRequest>) {
        assert!(page_number > 0, "PN should be > 0");
        // Call the getKYCRequests function from the KYC contract
        self.get_kyc_requests(page_number, true)
    }

    pub fn add_kyc_request(
        &mut self,
        customer_: Customer,
        current_time_: u32,
        notes_: String,
    ) {
        self.banks.is_valid_bank(env::predecessor_account_id());
        let req_id_ = Helpers::append(env::signer_account_id(), customer_.clone().id_);

        // Check if kyc request already exists
        assert!(!self.kyc_request_exists(req_id_.clone().to_string()), "User had kyc req.");

        // Create the kyc request
        let kyc_request = KycRequest {
            id_: req_id_.clone().to_string(),
            user_id_: customer_.clone().id_,
            customer_name: customer_.clone().name,
            bank_id_: env::signer_account_id(),
            bank_name: self.banks.get_single_bank(env::signer_account_id()).unwrap().name,
            data_hash: customer_.clone().data_hash,
            updated_on: current_time_,
            status: KycStatus::Pending,
            data_request: DataHashStatus::Pending,
            additional_notes: notes_,
        };

        // Add kyc request to the lookup map
        self.kyc_requests.insert(&req_id_.clone().to_string(), &kyc_request);

        // Update the customer-bank relationship
        if let Some(mut bank_customers) = self.bank_customers.get(&env::signer_account_id()) {
            bank_customers.push(customer_.clone().id_);
            self.bank_customers.insert(&env::signer_account_id(), &bank_customers);
        } else {
            let mut vec = Vec::new();
            vec.push(customer_.clone().id_);
            self.bank_customers.insert(&env::signer_account_id(), &vec);
        }

        if let Some(mut customer_banks) = self.customer_banks.get(&customer_.id_) {
            customer_banks.push(env::signer_account_id());
            self.customer_banks.insert(&customer_.id_, &customer_banks);
        } else {
            let mut vec = Vec::new();
            vec.push(env::signer_account_id());
            self.customer_banks.insert(&customer_.id_, &vec);
        }

        // Emit event
        log!(
                "KycRequestAdded: {:?}, {:?}, {:?}",
                req_id_,
                kyc_request.bank_name,
                customer_.clone().name

        );

        // Add the customer to the common list if not already present
        if !self.users.contains_key(&customer_.id_) {
            let user = User {
                name: customer_.clone().name,
                email: customer_.clone().email,
                id_: customer_.clone().id_,
                role: Role::Customer,
                status: BankStatus::Active,
            };
            self.users.insert(&customer_.id_, &user);
            self.user_list.insert(&customer_.id_);
        }
    }

    pub fn re_request_for_kyc_request(&mut self, id_: AccountId, notes_: String) {
        self.banks.is_valid_bank(env::signer_account_id());

        let req_id_ = Helpers::append(env::signer_account_id(), id_.clone());

        // Check if the KYC request exists
        assert!(self.kyc_request_exists(req_id_.clone().to_string()), "KYC req not found");

        // Check if the customer exists
        assert!(self.customers.customer_exists(id_.clone()), "User not found");

        // Update the KYC request status and additional notes
        if let Some(mut kyc_request) = self.kyc_requests.get(&req_id_.to_string()) {
            // kyc_request.status = KycStatus::Pending; // Uncomment this line if you want to update the status as well
            kyc_request.data_request = DataHashStatus::Pending;
            kyc_request.additional_notes = notes_;
        }

        // Emit event
        let kyc_request = self.kyc_requests.get(&req_id_.to_string()).expect("KYC request not found");
        log!(
                "KycReRequested: {:?}, {:?}, {:?}",
                req_id_,
                kyc_request.bank_name,
                kyc_request.customer_name
        );
    }

    pub fn update_kyc_verification(
        &mut self,
        user_id_: AccountId,
        verified_: bool,
        note_: String,
    ) {
        self.banks.is_valid_bank(env::signer_account_id());

        assert!(env::predecessor_account_id() == self.admin, "Only admin is allowed");
    
        let req_id_ = Helpers::append(env::predecessor_account_id(), user_id_.clone());
        assert!(self.kyc_request_exists(req_id_.clone().to_string()), "User doesn't have KYC req");
    
        let status_ = if verified_ {
            KycStatus::KYCVerified
        } else {
            KycStatus::KYCFailed
        };
    
        let mut kyc_request = self.kyc_requests.get(&req_id_.to_string()).expect("KYC req not found");
        kyc_request.status = status_.clone();
        kyc_request.additional_notes = note_.clone();
    
        self.kyc_requests.insert(&req_id_.to_string(), &kyc_request);
    
        log!(
            "KycStatusChanged: {:?}, {:?}, {:?}, {:?}",
            req_id_,
            user_id_,
            env::predecessor_account_id(),
            status_
        );
    }
    
    pub fn search_customers_kyc(&self, id_: AccountId) -> (bool, Customer, KycRequest) {
        self.banks.is_valid_bank(env::signer_account_id());
        self.customers.is_valid_customer(id_.clone());
    
        let (found_, customer_) = self.customers.search_customers(id_.clone(), self.bank_customers.get(&env::predecessor_account_id()).unwrap().clone());
    
        let request_: Option<KycRequest>;
        if found_ {
            let req_id_ = Helpers::append(env::predecessor_account_id().clone(), id_.clone());
            request_ = self.kyc_requests.get(&req_id_.to_string());
        } else {
            request_ = None;
        }
    
        (found_, customer_, request_.unwrap())
    }

    // Customer Interface

    pub fn get_bank_requests(&self, page_number: u32) -> (u32, Vec<KycRequest>) {
        self.customers.is_valid_customer(env::signer_account_id());

        assert!(page_number > 0, "PN should be > 0");
        // Assuming msg.sender is the customer's account ID (validated by isValidCustomer modifier)
        let (pages, kyc_requests) = self.get_kyc_requests(page_number, false);
        (pages, kyc_requests)
    }

    pub fn action_on_kyc_request(
        &mut self,
        bank_id_: AccountId,
        approve_: bool,
        note_: String,
    ) {
        self.customers.is_valid_customer(env::signer_account_id());
        self.banks.is_valid_bank(bank_id_.clone());

        let req_id_ = Helpers::append(bank_id_.clone(), env::predecessor_account_id().clone());
        assert!(self.kyc_request_exists(req_id_.clone().to_string()), "User doesn't have KYC req");

        let status_ = if approve_ {
           DataHashStatus::Approved
        } else {
           DataHashStatus::Rejected
        };

        // Update the KYC request
        let mut kyc_request = self.kyc_requests.get(&req_id_.to_string()).expect("KYC req not found");
        kyc_request.data_request = status_.clone();
        kyc_request.additional_notes = note_;
        self.kyc_requests.insert(&req_id_.to_string(), &kyc_request);

        log!(
            "action_on_kyc_request: {:?}, {:?}, {:?}, {:?}",
            req_id_, 
            env::predecessor_account_id(), 
            bank_id_, status_);
    }

    pub fn update_profile_kyc(&mut self, name_: String, email_: String, mobile_: u32) {
        self.customers.is_valid_customer(env::signer_account_id());

        // assert_eq!(env::predecessor_account_id(), self.users.get(&env::predecessor_account_id()).expect("User not found").id_, "Invalid caller");
        
        let mut user = self.users.get(&env::predecessor_account_id()).expect("User not found");
        user.name = name_.clone();
        user.email = email_.clone();

        self.users.insert(&env::predecessor_account_id(), &user);

        self.customers.update_profile(name_.clone(), email_.clone(), mobile_.clone());
    }

    pub fn update_datahash(&mut self, hash_: String, current_time_: u32) {
        self.customers.is_valid_customer(env::signer_account_id());
        
        // assert_eq!(env::predecessor_account_id(), self.users.get(&env::predecessor_account_id()).expect("User not found").id_, "Invalid caller");
        self.customers.update_data_hash(hash_.clone(), current_time_);
        
        // Reset KYC verification status for all banks
        let banks_list_ = self.customer_banks.get(&env::predecessor_account_id()).expect("Banks list not found");
        for bank_id in banks_list_ {
            let req_id_ = Helpers::append(bank_id.clone(), env::predecessor_account_id());
            if self.kyc_request_exists(req_id_.clone().to_string()) {
                let mut kyc_request = self.kyc_requests.get(&req_id_.to_string()).expect("KYC request not found");
                kyc_request.data_hash = hash_.clone();
                kyc_request.updated_on = current_time_;
                kyc_request.status = KycStatus::Pending;
                kyc_request.additional_notes = "Updated all my docs".to_string();
                self.kyc_requests.insert(&req_id_.to_string(), &kyc_request);
            }
        }
    }

    pub fn remove_datahash_permission(&mut self, bank_id_: AccountId, notes_: String) {
        self.customers.is_valid_customer(env::signer_account_id());

        // assert_eq!(env::predecessor_account_id(), self.users.get(&env::predecessor_account_id()).expect("User not found").id_, "Invalid caller");
        let req_id_ = Helpers::append(bank_id_.clone(), env::predecessor_account_id());
        assert!(self.kyc_request_exists(req_id_.clone().to_string()), "Permission not found");

        let mut kyc_request = self.kyc_requests.get(&req_id_.to_string()).expect("KYC request not found");
        kyc_request.data_request = DataHashStatus::Rejected;
        kyc_request.additional_notes = notes_.clone();
        self.kyc_requests.insert(&req_id_.to_string(), &kyc_request);

        log!(
            "DataHashPermissionChanged: {:?}, {:?}, {:?}, {:?}",
            req_id_,
            env::predecessor_account_id(),
            bank_id_,
            DataHashStatus::Rejected,
        );
    }

    pub fn search_banks(&self, bank_id_: AccountId) -> (bool, Bank, KycRequest) {
        self.customers.is_valid_customer(env::signer_account_id());
        self.banks.is_valid_bank(bank_id_.clone());

        let mut found_ = false;
        let mut bank_: Option<Bank> = None;
        let mut request_: Option<KycRequest> = None;
        let banks_ = self.customer_banks.get(&env::predecessor_account_id()).expect("Banks not found");

        for i in 0..banks_.len() {
            if banks_[i] == bank_id_ {
                found_ = true;
                bank_ = self.banks.get_single_bank(bank_id_.clone());
                let req_id_ = Helpers::append(bank_id_.clone(), env::predecessor_account_id());
                request_ = self.kyc_requests.get(&req_id_.to_string());
                break;
            }
        }
        (found_, bank_.unwrap(), request_.unwrap())
    }

    // Common Interface

    pub fn who_am_i(&self) -> User {
        let sender_id = env::signer_account_id();
        assert_ne!(sender_id, AccountId::new_unchecked("".to_string()), "Sender Id Empty");
        assert!(self.users.get(&sender_id).is_some(), "User Id Empty");
        self.users.get(&sender_id).unwrap()
    }
    
    pub fn get_customer_details_kyc(&self, id_: AccountId) -> Customer{
        self.customers.is_valid_customer(id_.clone());
        self.customers.get_customer_details(id_)
    }

    pub fn get_bank_details(&self, id_: AccountId) -> Bank {
        self.banks.is_valid_bank(id_.clone());
        self.banks.get_single_bank(id_.clone()).unwrap()
    }
}