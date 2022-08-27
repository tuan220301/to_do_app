/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

 use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
 use near_sdk::env;
 use near_sdk::json_types::U64;
 use near_sdk::serde::{Deserialize, Serialize};
 use near_sdk::{log, near_bindgen, AccountId};

// Define the default message
// const DEFAULT_TASK: &str = "Hello";

// Define the contract structure
// #[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Task{
    id: u32,
    content: String, 
}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    tasks: Vec<Task>,
}
// pub struct Contract {
//     message: String,
// }
// Define the default, which automatically initializes the contract
// impl Default for Contract{
//     fn default() -> Self{
//         Self{message: DEFAULT_MESSAGE.to_string()}
//     }
// }
impl Default for Contract {
    fn default() -> Self {
        env::panic(b"ToDo contract should be initialized before usage")
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    // Public method - returns the greeting saved, defaulting to DEFAULT_MESSAGE
    pub fn new() -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        Contract { tasks: Vec::new() }
    }
    pub fn get_task(self) -> Vec<Task> {
        return self.tasks;
    }
    // pub fn get_greeting(&self) -> String {
    //     return self.message.clone();
    // }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn add_task(&mut self, content: String) {
        // Use env::log to record logs permanently to the blockchain!
        // log!("Saving greeting {}", message);
        // self.message = message;
        let mut id: u32 = 1;
        let task_clone = self.tasks.clone();
        if task_clone.len() > 0 {
            let len = task_clone.len();
            id = task_clone[len - 1].id + 1;
        }
        let new_task: Task = Task {
            id: id,
            content: content,
            // is_completed: false,
        };
        self.tasks.push(new_task);
    }
    pub fn delete_task(&mut self, id: u32){
        let index = self.tasks.iter().position(|o| o.id == id).unwrap();
        self.tasks.remove(index);
    }
    // pub fn set_greeting(&mut self, message: String) {
    //     // Use env::log to record logs permanently to the blockchain!
    //     log!("Saving greeting {}", message);
    //     self.message = message;
    // }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_greeting() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(
            contract.get_greeting(),
            "Hello".to_string()
        );
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(
            contract.get_greeting(),
            "howdy".to_string()
        );
    }
}
