use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, AccountId};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Role {
    Admin, // 0
    CO, // 1
    Repre, // 2
}

#[derive(Debug,BorshDeserialize, BorshSerialize, Deserialize, Serialize, PartialEq, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum COStatus {
    Active, // 0
    Inactive, // 1
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum KycStatus {
    Pending, // 0
    KYCVerified, // 1
    KYCFailed, // 2
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum DataHashStatus {
    Pending, // 0
    Approved, // 1
    Rejected, // 2
}

// #[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct User {
    pub name: String,
    pub email: String,
    pub id_: AccountId,
    pub role: Role,
    pub status: COStatus,
}

// #[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Repre {
    pub name: String,
    pub email: String,
    pub mobile_number: u32,
    pub id_: AccountId,
    pub kyc_verified_by: AccountId, // Address of the CO only if KYC gets verified
    pub data_hash: String, // Documents will be stored in decentralised storage & a hash will be created for the same
    pub data_updated_on: u32,
}

// #[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CO {
    pub name: String,
    pub email: String,
    pub id_: AccountId,
    pub npoid_code: String, //viết tắt của Nonprofit Organization ID
    pub kyc_count: u16, // How many KCY's did this CO completed so far
    pub status: COStatus, // RBI, we call "admin" here can disable the CO at any instance
}

// #[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct KycRequest {
    pub id_: String, // Combination of Repre Id & CO is going to be unique
    pub user_id_: AccountId,
    pub repre_name: String,
    pub co_id_: AccountId,
    pub co_name: String,
    pub data_hash: String,
    pub updated_on: u32,
    pub status: KycStatus,
    pub data_request: DataHashStatus, // Get approval from user to access the data
    pub additional_notes: String, // Notes that can be added if KYC verification fails  OR
    // if repre rejects the access & CO wants to re-request with some message
}
