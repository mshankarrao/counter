#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod counter {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Counter {
     // Storage Declaration
     value: i32,
    }

    impl Counter {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            // Contract Constructor
            Self { value: init_value }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self { value: 0 }
        }

        #[ink(message)]
        pub fn get_counter(&self) -> i32 {
            self.value
        }

        #[ink(message)]
        pub fn increment_counter(&mut self) {
            self.value += 1;
        }

        #[ink(message)]
        pub fn decrement_counter(&mut self) {
            self.value -= 1;
        }

    }

}
