#![cfg(test)]

use super::*;
use soroban_sdk::{vec, Env, String};

#[test]
fn test_greet() {
    let env = Env::default();
    let contract_id = env.register(CalculatorContract, ());
    let client = CalculatorContractClient::new(&env, &contract_id);
    let greeting = client.greet();
    assert_eq!(greeting, String::from_str(&env, "Hello Soroban!"));
}
#[test]
fn test_add() {
    let env = Env::default();
    let contract_id = env.register(CalculatorContract, ());
    let client = CalculatorContractClient::new(&env, &contract_id);

    let result = client.add(&5, &3);
    assert_eq!(result, 8);
}
#[test]
fn test_multiply() {
   let env = Env::default();
    let contract_id = env.register(CalculatorContract, ());
    let client = CalculatorContractClient::new(&env, &contract_id);

    let result = client.multiply(&4, &6);
    assert_eq!(result, 24);
}

#[test]
fn test_calculate_and_store() {
    let env = Env::default();
    let contract_id = env.register(CalculatorContract, ());
    let client = CalculatorContractClient::new(&env, &contract_id);

    let result = client.calculate_and_store(&10, &20);
    assert_eq!(result, 30);
}

#[test]
fn test_get_last_calculation() {
   let env = Env::default();
    let contract_id = env.register(CalculatorContract, ());
    let client = CalculatorContractClient::new(&env, &contract_id);

    // Primero guardamos algo
    client.calculate_and_store(&15, &25);
    
    // Luego lo recuperamos
    let result = client.get_last_calculation();
    assert_eq!(result, 40);
}