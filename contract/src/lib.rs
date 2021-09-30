/*
 * This is an example of a Rust smart contract with two simple, symmetric functions:
 *
 * 1. set_greeting: accepts a greeting, such as "howdy", and records it for the user (account_id)
 *    who sent the request
 * 2. get_greeting: accepts an account_id and returns the greeting saved for it, defaulting to
 *    "Hello"
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://github.com/near/near-sdk-rs
 *
 */

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)

use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_sdk::collections::LazyOption;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{Vector, UnorderedSet, LookupMap};
use near_sdk::{env, near_bindgen, AccountId, Balance, BorshStorageKey, PanicOnDefault, Promise, PromiseResult};
use near_sdk::serde::Serialize;
use near_sdk::json_types::ValidAccountId;
use near_sdk::MockedBlockchain;
// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
// #[near_bindgen]
// #[derive(BorshDeserialize, BorshSerialize)]
// pub struct Welcome {
//     records: LookupMap<String, String>,
// }

// impl Default for Welcome {
//   fn default() -> Self {
//     Self {
//       records: LookupMap::new(b"a".to_vec()),
//     }
//   }
// }
const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,1.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V8715.34,0,0,0,200.66,72.5h0A15.35,15.35,0,1Z'/%3E%3C/g%3E%3C/svg%3E";

// #[near_bindgen]
// impl Welcome {
//     pub fn set_greeting(&mut self, message: String) {
//         let account_id = env::signer_account_id();

//         // Use env::log to record logs permanently to the blockchain!
//         env::log(format!("Saving greeting '{}' for account '{}'", message, account_id,).as_bytes());

//         self.records.insert(&account_id, &message);
//     }

//     // `match` is similar to `switch` in other languages; here we use it to default to "Hello" if
//     // self.records.get(&account_id) is not yet defined.
//     // Learn more: https://doc.rust-lang.org/book/ch06-02-match.html#matching-with-optiont
//     pub fn get_greeting(&self, account_id: String) -> String {
//         match self.records.get(&account_id) {
//             Some(greeting) => greeting,
//             None => "Hello".to_string(),
//         }
//     }
// // }
// #[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Animal {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[near_bindgen]
impl Animal {
    #[init]
    pub fn new_default_meta(owner_id: ValidAccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Example NEAR non-fungible token".to_string(),
                symbol: "EXAMPLE".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }
    #[init]
    pub fn new(owner_id: ValidAccountId, metadata: NFTContractMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
        }
    }

    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        token_owner_id: ValidAccountId,
        token_metadata: TokenMetadata,
    ) -> Token {
        self.tokens.mint(token_id, token_owner_id, Some(token_metadata))
    }

}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct AnimalFight{
    pub accounts:LookupMap<AccountId, String>,
    pub animals: LookupMap<String, u32>,


}

#[near_bindgen]
impl AnimalFight {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            accounts: LookupMap::new(b"r".to_vec()),
            animals: LookupMap::new(b"a".to_vec()),
        }
    }

    // #[debug_assert!]
    pub fn add_first(&mut self, animal_name: &String) -> String{
        let account_id= env::signer_account_id();
        let name = match self.accounts.get(&account_id){
            None => animal_name,
            Some(r) => return "already exist".to_string()
        };
        // if name.len() > 0{
        //     return "already exist".to_string()
        // }
        let level = match self.animals.get(&animal_name){
            None => 1,
            Some(r) => return "exist name".to_string()
        };

        self.accounts.insert(&account_id, &name);
        self.animals.insert(&animal_name, &level);
        "success".to_string()
    }

    pub fn get_level(&self,animal_name:&String) -> u32{

        let level = match self.animals.get(&animal_name){
            Some(r) => r,
            None => return 0
        };
        return level
    }
    pub fn get_all(&mut self) -> String{
        let account_id= env::signer_account_id();
        
        let name = match self.accounts.get(&account_id){
            Some(r) => r,
            None => return "NO pets".to_string()
        };
        return name
    }

    pub fn remove_animal(&mut self) -> String{
        let account_id= env::signer_account_id();
        
        let name = match self.accounts.get(&account_id){
            Some(r) => r,
            None => return "no chongwu".to_string()
        };
        self.accounts.remove(&account_id);
        self.animals.remove(&name);
        "success".to_string()
    }

    pub fn fight_with(&mut self, my_animal:&String, animal_name: &String) -> String{
        let account_id= env::signer_account_id();
        // let myanimal = match self.accounts.get(&account_id){
        //     Some(r) => r,
        //     None => return "you have no animals".to_string()
        // };

        let mylevel = match self.animals.get(&my_animal){
            Some(r) => r,
            None => return "you have no animals".to_string()
        };

        let enemylevel = match self.animals.get(&animal_name){
            Some(r) => r,
            None => return "you have no animals".to_string()
        };

        if mylevel > enemylevel{
            // self.win(animal_name);
            return "you win".to_string()
        }
        if mylevel == enemylevel{
            return "equal".to_string()
        }
        return "you lost".to_string()
    }

    #[payable]
    pub fn levelup(&mut self, animal_name: &String) -> String {
        assert!(env::attached_deposit() >= 10, "Deposit is too low");
        let mut old = match self.animals.get(animal_name){
            Some(r) => r,
            None => return "no animal".to_string()
        };
        old += 1;
        self.animals.remove(&animal_name);
        self.animals.insert(&animal_name, &old);
        "levelup success".to_string()
    }




}



near_contract_standards::impl_non_fungible_token_core!(Animal, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Animal, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Animal, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for  Animal{
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}
/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 *
 * To run from contract directory:
 * cargo test -- --nocapture
 *
 * From project root, to run in combination with frontend tests:
 * yarn test
 *
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(accountId: AccountId, block_timestamp: u64) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            output_data_receivers: vec![],
            epoch_height: 19,
            is_view: false,
        }
    }

    

  
    #[test]
    fn test__add_first() {
        let mut context = get_context("aa.test".to_string(), 3_600_000_000_000);
        testing_env!(context.clone());
        let mut contract = AnimalFight::new();
        let name = "asdf".to_string();
        assert_eq!(contract.add_first(&name), "success".to_string());
        assert_eq!(contract.add_first(&name), "already exist".to_string());

    }

    #[test]
    fn test__get_name() {
        let mut context = get_context("aa.test".to_string(), 3_600_000_000_000);
        testing_env!(context.clone());
        let accoutid = context.current_account_id.clone();
        let mut contract = AnimalFight::new();
        let name = "asdf".to_string();
        assert_eq!(contract.add_first(&name), "success".to_string());
        assert_eq!(contract.get_all(), "asdf".to_string());

    }
    #[test]
    fn test__fight() {
        let mut context = get_context("aa.test".to_string(), 3_600_000_000_000);
        testing_env!(context.clone());
        let mut contract = AnimalFight::new();
        let name1 = "asdf".to_string();
        let name2 = "aaaa".to_string();
        assert_eq!(contract.add_first(&name1), "success".to_string());


        context.signer_account_id = "dd2.test".to_string();
        testing_env!(context.clone());
        assert_eq!(contract.add_first(&name2), "success");
        
        assert_eq!(contract.fight_with(&name1,&name2), "equal");

    }
    #[test]
    fn test__remove() {
        let mut context = get_context("aa.test".to_string(), 3_600_000_000_000);
        testing_env!(context.clone());
        let mut contract = AnimalFight::new();
        let name = "asdf".to_string();
        assert_eq!(contract.add_first(&name), "success".to_string());
        assert_eq!(contract.remove_animal(), "success".to_string());
    
    }

    #[test]
    fn test__levelup() {
        let mut context = get_context("aa.test".to_string(), 3_600_000_000_000);
        testing_env!(context.clone());
        let mut contract = AnimalFight::new();
        let name = "asdf".to_string();
        assert_eq!(contract.add_first(&name), "success".to_string());
        assert_eq!(contract.levelup(&name), "success".to_string());
    
    }
    
}
