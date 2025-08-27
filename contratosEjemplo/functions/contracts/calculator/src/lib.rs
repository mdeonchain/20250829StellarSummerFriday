#![no_std]
use soroban_sdk::{contract, contractimpl, vec, Env, String, symbol_short};

#[contract]
pub struct CalculatorContract;


#[contractimpl]
impl CalculatorContract {
     pub fn greet(env: Env) -> String {
        String::from_str(&env, "Hello Soroban!")
    }
    
    pub fn add(env: Env, a: i32, b: i32) -> i32 {
        a + b
    }
    
    pub fn multiply(env: Env, a: i32, b: i32) -> i32 {
        a * b
    }
    
    pub fn calculate_and_store(env: Env, a: i32, b: i32) -> i32 {
        let result = a + b;
        env.storage().instance().set(&symbol_short!("last_calc"), &result);
        result
    }
    
    pub fn get_last_calculation(env: Env) -> i32 {
        env.storage().instance().get(&symbol_short!("last_calc")).unwrap_or(0)
    }
}

mod test;
