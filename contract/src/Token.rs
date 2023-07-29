use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{log, near_bindgen, Balance, Promise, env, AccountId, PanicOnDefault};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};

const INITIAL_SUPPLY: Balance = 100_000_000_000; // Số dư ban đầu của token

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Token {
    owner_id: AccountId,
    balances: LookupMap<AccountId, Balance>,
    name: String,
    symbol: String,
}

#[near_bindgen]
impl Token {
    #[init]
    pub fn new_token(owner_id: AccountId, name: String, symbol: String) -> Self {
        let mut balances = near_sdk::collections::LookupMap::new(b"b".to_vec());
        balances.insert(&owner_id, &INITIAL_SUPPLY);
        Self {
            owner_id,
            balances,
            name,
            symbol,
        }
    }

    #[payable]
    pub fn transfer(&mut self, receiver_id: AccountId, amount: Balance) {
        // let sender_id = env::predecessor_account_id();
        let sender_id = self.owner_id.clone();
        assert!(self.balances.contains_key(&sender_id), "Sender does not own any tokens");
        let sender_balance = self.balances.get(&sender_id).unwrap();
        assert!(sender_balance >= amount, "Sender does not have enough balance to transfer");

        self.balances.insert(&sender_id, &(sender_balance - amount));

        let receiver_balance = self.balances.get(&receiver_id).unwrap_or(0);
        self.balances.insert(&receiver_id, &(receiver_balance + amount));

        log!("Transferred {} tokens from {} to {}", amount, sender_id, receiver_id);
    }

    pub fn get_balance(&self, account_id: AccountId) -> Balance {
        self.balances.get(&account_id).unwrap_or(0)
    }

    pub fn mint(&mut self, account_id: AccountId, amount: Balance) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only the owner can mint new tokens");
        let balance = self.balances.get(&account_id).unwrap_or(0);
        self.balances.insert(&account_id, &(balance + amount));
    }

    pub fn burn(&mut self, account_id: AccountId, amount: Balance) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only the owner can burn tokens");
        let balance = self.balances.get(&account_id).unwrap_or(0);
        assert!(balance >= amount, "Account does not have enough tokens to burn");
        self.balances.insert(&account_id, &(balance - amount));
    }

    pub fn get_owner_id(&self) -> AccountId {
        self.owner_id.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_symbol(&self) -> String {
        self.symbol.clone()
    }

}
