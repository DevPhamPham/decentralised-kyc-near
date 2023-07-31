// File: kyc.rs
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet,UnorderedMap,Vector};
use near_sdk::{log, near_bindgen, env, AccountId, PanicOnDefault};
// use near_sdk::serde::{Deserialize, Serialize};


use crate::Types::{Role, COStatus, KycStatus, DataHashStatus, User, Repre, CO, KycRequest};
use crate::Repres::Repres;
use crate::COs::COs;
use crate::Helpers::Helpers;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
// #[warn(non_snake_case)]
pub struct KYC {
    admin: AccountId,
    user_list: UnorderedSet<AccountId>,
    users: LookupMap<AccountId, User>,
    kyc_requests: LookupMap<String, KycRequest>,
    co_repres: LookupMap<AccountId, Vec<AccountId>>,
    repre_cos: LookupMap<AccountId, Vec<AccountId>>,
    cos: COs,
    repres: Repres,
}

#[near_bindgen]
impl KYC {
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
            status: COStatus::Active,
        };

        let mut users = LookupMap::new(b"u");
        users.insert(&admin, &user);

        let cos = COs {
            co_list: Vec::new(),
            cos: UnorderedMap::new(b"b".try_to_vec().unwrap()),
        };

        let repres = Repres {
            repre_list: Vector::new(b"cl".try_to_vec().unwrap()),
            repres: LookupMap::new(b"c".try_to_vec().unwrap()),
        };

        KYC {
            admin,
            user_list,
            users,
            kyc_requests: LookupMap::new(b"kr".try_to_vec().unwrap()),
            co_repres: LookupMap::new(b"bc".try_to_vec().unwrap()),
            repre_cos: LookupMap::new(b"cb".try_to_vec().unwrap()),
            cos,
            repres,
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
        is_for_co: bool,
    ) -> (u32, Vec<KycRequest>) {
        let page_number_u32: u32 = page_number.into();
        assert!(page_number_u32 > 0, "PN should be > zero");
        let users = if is_for_co {
            self.co_repres.get(&env::signer_account_id()).unwrap()
        } else {
            self.repre_cos.get(&env::signer_account_id()).unwrap()
        };
        let (pages, page_length, start_index, end_index) = Helpers::get_indexes(page_number_u32, users.clone());
        let mut list: Vec<KycRequest> = Vec::with_capacity(page_length as usize);
        for i in start_index..end_index {
            let key = if is_for_co {
                Helpers::append(env::signer_account_id(), users[i as usize].clone())
            } else {
                Helpers::append(users[i as usize].clone(), env::signer_account_id())
            };            
            list.push(self.kyc_requests.get(&key.to_string()).unwrap());
        }
        (pages.into(), list)
    }

    // Admin Interface
    pub fn get_all_co_kyc(&self, page_number: u32) -> (u32, Vec<CO>) {
        // let cos_contract = COs::default(); 
        self.is_admin();
        self.cos.get_all_cos(page_number) 
    }

    pub fn add_co_kyc(&mut self, co: CO) {
        self.is_admin();
        
        self.cos.add_co(co.clone());

        self.users.insert(
            &co.id_,
            &User {
                name: co.name.clone(),
                email: co.email.clone(),
                id_: co.id_.clone(),
                role: Role::CO,
                status: COStatus::Active,
            },
        );

        self.user_list.insert(&co.id_);
    }

    pub fn update_co_details(&mut self, id_: AccountId, email_: String, name_: String) {
        self.is_admin();
        self.cos.update_co(id_.clone(), email_.clone(), name_.clone());

        if let Some(mut user) = self.users.get(&id_) {
            user.name = name_;
            user.email = email_;
        }
    }

    pub fn activate_deactivate_co_kyc(&mut self, id_: AccountId, make_active_: bool) {
        self.is_admin();
        let new_status = self.cos.activate_deactivate_co(id_.clone(), make_active_);

        if let Some(mut user) = self.users.get(&id_) {
            user.status = new_status;
        }
    }

    // CO Interface

    pub fn get_repres_of_co(&self, page_number: u32) -> (u32, Vec<KycRequest>) {
        assert!(page_number > 0, "PN should be > 0");
        // Call the getKYCRequests function from the KYC contract
        self.get_kyc_requests(page_number, true)
    }

    pub fn add_kyc_request(
        &mut self,
        repre_: Repre,
        current_time_: u32,
        notes_: String,
    ) {
        self.cos.is_valid_co(env::predecessor_account_id());
        let req_id_ = Helpers::append(env::signer_account_id(), repre_.clone().id_);

        assert!(!self.kyc_request_exists(req_id_.clone().to_string()), "User had kyc req.");

        let kyc_request = KycRequest {
            id_: req_id_.clone().to_string(),
            user_id_: repre_.clone().id_,
            repre_name: repre_.clone().name,
            co_id_: env::signer_account_id(),
            co_name: self.cos.get_single_co(env::signer_account_id()).unwrap().name,
            data_hash: repre_.clone().data_hash,
            updated_on: current_time_,
            status: KycStatus::Pending,
            data_request: DataHashStatus::Pending,
            additional_notes: notes_,
        };

        self.kyc_requests.insert(&req_id_.clone().to_string(), &kyc_request);

        if let Some(mut co_repres) = self.co_repres.get(&env::signer_account_id()) {
            co_repres.push(repre_.clone().id_);
            self.co_repres.insert(&env::signer_account_id(), &co_repres);
        } else {
            let mut vec = Vec::new();
            vec.push(repre_.clone().id_);
            self.co_repres.insert(&env::signer_account_id(), &vec);
        }

        if let Some(mut repre_cos) = self.repre_cos.get(&repre_.id_) {
            repre_cos.push(env::signer_account_id());
            self.repre_cos.insert(&repre_.id_, &repre_cos);
        } else {
            let mut vec = Vec::new();
            vec.push(env::signer_account_id());
            self.repre_cos.insert(&repre_.id_, &vec);
        }

        log!(
                "KycRequestAdded: {:?}, {:?}, {:?}",
                req_id_,
                kyc_request.co_name,
                repre_.clone().name

        );

        if !self.users.contains_key(&repre_.id_) {
            let user = User {
                name: repre_.clone().name,
                email: repre_.clone().email,
                id_: repre_.clone().id_,
                role: Role::Repre,
                status: COStatus::Active,
            };
            self.users.insert(&repre_.id_, &user);
            self.user_list.insert(&repre_.id_);
        }
    }

    pub fn re_request_for_kyc_request(&mut self, id_: AccountId, notes_: String) {
        self.cos.is_valid_co(env::signer_account_id());

        let req_id_ = Helpers::append(env::signer_account_id(), id_.clone());

        assert!(self.kyc_request_exists(req_id_.clone().to_string()), "KYC req not found");

        assert!(self.repres.repre_exists(id_.clone()), "User not found");

        if let Some(mut kyc_request) = self.kyc_requests.get(&req_id_.to_string()) {
            kyc_request.data_request = DataHashStatus::Pending;
            kyc_request.additional_notes = notes_;
        }

        let kyc_request = self.kyc_requests.get(&req_id_.to_string()).expect("KYC request not found");
        log!(
                "KycReRequested: {:?}, {:?}, {:?}",
                req_id_,
                kyc_request.co_name,
                kyc_request.repre_name
        );
    }

    pub fn update_kyc_verification(
        &mut self,
        user_id_: AccountId,
        verified_: bool,
        note_: String,
    ) {
        self.cos.is_valid_co(env::signer_account_id());

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
    
    pub fn search_repres_kyc(&self, id_: AccountId) -> (bool, Repre, KycRequest) {
        self.cos.is_valid_co(env::signer_account_id());
        self.repres.is_valid_repre(id_.clone());
    
        let (found_, repre_) = self.repres.search_repres(id_.clone(), self.co_repres.get(&env::predecessor_account_id()).unwrap().clone());
    
        let request_: Option<KycRequest>;
        if found_ {
            let req_id_ = Helpers::append(env::predecessor_account_id().clone(), id_.clone());
            request_ = self.kyc_requests.get(&req_id_.to_string());
        } else {
            request_ = None;
        }
    
        (found_, repre_, request_.unwrap())
    }

    // Repre Interface

    pub fn get_co_requests(&self, page_number: u32) -> (u32, Vec<KycRequest>) {
        self.repres.is_valid_repre(env::signer_account_id());

        assert!(page_number > 0, "PN should be > 0");
        let (pages, kyc_requests) = self.get_kyc_requests(page_number, false);
        (pages, kyc_requests)
    }

    pub fn action_on_kyc_request(
        &mut self,
        co_id_: AccountId,
        approve_: bool,
        note_: String,
    ) {
        self.repres.is_valid_repre(env::signer_account_id());
        self.cos.is_valid_co(co_id_.clone());

        let req_id_ = Helpers::append(co_id_.clone(), env::predecessor_account_id().clone());
        assert!(self.kyc_request_exists(req_id_.clone().to_string()), "User doesn't have KYC req");

        let status_ = if approve_ {
           DataHashStatus::Approved
        } else {
           DataHashStatus::Rejected
        };

        let mut kyc_request = self.kyc_requests.get(&req_id_.to_string()).expect("KYC req not found");
        kyc_request.data_request = status_.clone();
        kyc_request.additional_notes = note_;
        self.kyc_requests.insert(&req_id_.to_string(), &kyc_request);

        log!(
            "action_on_kyc_request: {:?}, {:?}, {:?}, {:?}",
            req_id_, 
            env::predecessor_account_id(), 
            co_id_, status_);
    }

    pub fn update_profile_kyc(&mut self, name_: String, email_: String, mobile_: u32) {
        self.repres.is_valid_repre(env::signer_account_id());

        // assert_eq!(env::predecessor_account_id(), self.users.get(&env::predecessor_account_id()).expect("User not found").id_, "Invalid caller");
        
        let mut user = self.users.get(&env::predecessor_account_id()).expect("User not found");
        user.name = name_.clone();
        user.email = email_.clone();

        self.users.insert(&env::predecessor_account_id(), &user);

        self.repres.update_profile(name_.clone(), email_.clone(), mobile_.clone());
    }

    pub fn update_datahash(&mut self, hash_: String, current_time_: u32) {
        self.repres.is_valid_repre(env::signer_account_id());
        
        // assert_eq!(env::predecessor_account_id(), self.users.get(&env::predecessor_account_id()).expect("User not found").id_, "Invalid caller");
        self.repres.update_data_hash(hash_.clone(), current_time_);
        
        let cos_list_ = self.repre_cos.get(&env::predecessor_account_id()).expect("COs list not found");
        for co_id in cos_list_ {
            let req_id_ = Helpers::append(co_id.clone(), env::predecessor_account_id());
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

    pub fn remove_datahash_permission(&mut self, co_id_: AccountId, notes_: String) {
        self.repres.is_valid_repre(env::signer_account_id());

        // assert_eq!(env::predecessor_account_id(), self.users.get(&env::predecessor_account_id()).expect("User not found").id_, "Invalid caller");
        let req_id_ = Helpers::append(co_id_.clone(), env::predecessor_account_id());
        assert!(self.kyc_request_exists(req_id_.clone().to_string()), "Permission not found");

        let mut kyc_request = self.kyc_requests.get(&req_id_.to_string()).expect("KYC request not found");
        kyc_request.data_request = DataHashStatus::Rejected;
        kyc_request.additional_notes = notes_.clone();
        self.kyc_requests.insert(&req_id_.to_string(), &kyc_request);

        log!(
            "DataHashPermissionChanged: {:?}, {:?}, {:?}, {:?}",
            req_id_,
            env::predecessor_account_id(),
            co_id_,
            DataHashStatus::Rejected,
        );
    }

    pub fn search_cos(&self, co_id_: AccountId) -> (bool, CO, KycRequest) {
        self.repres.is_valid_repre(env::signer_account_id());
        self.cos.is_valid_co(co_id_.clone());

        let mut found_ = false;
        let mut co_: Option<CO> = None;
        let mut request_: Option<KycRequest> = None;
        let cos_ = self.repre_cos.get(&env::predecessor_account_id()).expect("COs not found");

        for i in 0..cos_.len() {
            if cos_[i] == co_id_ {
                found_ = true;
                co_ = self.cos.get_single_co(co_id_.clone());
                let req_id_ = Helpers::append(co_id_.clone(), env::predecessor_account_id());
                request_ = self.kyc_requests.get(&req_id_.to_string());
                break;
            }
        }
        (found_, co_.unwrap(), request_.unwrap())
    }

    // Common Interface

    pub fn who_am_i(&self) -> User {
        let sender_id = env::signer_account_id();
        assert_ne!(sender_id, AccountId::new_unchecked("".to_string()), "Sender Id Empty");
        assert!(self.users.get(&sender_id).is_some(), "User Id Empty");
        self.users.get(&sender_id).unwrap()
    }
    
    pub fn get_repre_details_kyc(&self, id_: AccountId) -> Repre{
        self.repres.is_valid_repre(id_.clone());
        self.repres.get_repre_details(id_)
    }

    pub fn get_co_details(&self, id_: AccountId) -> CO {
        self.cos.is_valid_co(id_.clone());
        self.cos.get_single_co(id_.clone()).unwrap()
    }

}