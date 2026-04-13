#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Env};

#[test]
fn test_init() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GuardianVault);
    let client = GuardianVaultClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let guardian_1 = Address::generate(&env);
    let guardians = Vec::from_array(&env, [guardian_1]);

    client.init(&owner, &guardians);
    // Add logic to verify storage here
}
