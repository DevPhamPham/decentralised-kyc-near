use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{log, env, near_bindgen, AccountId, Balance, Promise, PanicOnDefault};
use near_sdk::collections::LookupMap;
use crate::Token::Token;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct TokenMarketplace {
    token_contract: Token,
    token_price: Balance,
}

#[near_bindgen]
impl TokenMarketplace {
    #[init]
    pub fn new_marketplace(token_contract: AccountId, token_price: Balance) -> Self {
        Self {
            token_contract: Token::new_token(token_contract.clone(),"CharityCoin".to_string(),"CRT".to_string()),
            token_price,
        }
    }

    pub fn buy_tokens(&mut self, amount: Balance) {
        let buyer_id = env::predecessor_account_id();
        let required_amount = self.token_price * amount;

        assert!(env::attached_deposit() >= required_amount, "Not enough funds attached to buy tokens");

        Promise::new(self.token_contract.get_owner_id()).transfer(required_amount);

        self.token_contract.transfer(buyer_id.clone(), amount);

        log!("Bought {} tokens for {} NEAR", amount, required_amount);
    }

    pub fn sell_tokens(&mut self, amount: Balance) {
        let seller_id = env::predecessor_account_id();
        let required_amount = self.token_price * amount;

        let seller_balance = self.token_contract.get_balance(seller_id.clone());
        assert!(seller_balance >= amount, "Seller does not have enough tokens to sell");

        self.token_contract.transfer(self.token_contract.get_owner_id(), amount);

        Promise::new(seller_id.clone()).transfer(required_amount);

        log!("Sold {} tokens for {} NEAR", amount, required_amount);
    }

    pub fn get_token_balance(&self, account_id: AccountId) -> Balance {
        self.token_contract.get_balance(account_id)
    }
}
