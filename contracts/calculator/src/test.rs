#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Logs, Env};

extern crate std;

fn setup_env() -> (Env, CalculatorClient<'static>) {
    let env = Env::default();
    let contract_id = env.register_contract(None, Calculator);
    let client = CalculatorClient::new(&env, &contract_id);
    (env, client)
}

#[test]
fn test_correct_calc() {
    // case when calc execut correct

    let (env, client) = setup_env();

    assert_eq!(client.sum(&1, &2), 3);
    assert_eq!(client.sub(&3, &2), 1);
    assert_eq!(client.mul(&2, &3), 6);
    assert_eq!(client.div(&6, &2), 3);
    assert_eq!(client.div(&6, &0), u32::MAX);

    std::println!("{}", env.logs().all().join("\n"));
}

#[test]
fn test_incorrect_calc() {
    // case when calc not execut correct

    let (env, client) = setup_env();

    assert_eq!(client.sum(&u32::MAX, &1), u32::MAX);
    assert_eq!(client.sub(&8, &10), 2);
    assert_eq!(client.mul(&2, &3), 6);
    assert_eq!(client.div(&0, &9), 0);
    assert_eq!(client.div(&8, &0), u32::MAX);
    std::println!("{}", env.logs().all().join("\n"));
}
