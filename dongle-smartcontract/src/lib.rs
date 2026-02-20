#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, String};

mod types;
mod errors;
mod project_registry;
mod utils;

use crate::project_registry::ProjectRegistry;
use crate::errors::ContractError;

#[contract]
pub struct DongleContract;

#[contractimpl]
impl DongleContract {
    pub fn update_project(
        env: Env, 
        project_id: u64, 
        caller: Address, 
        name: String,
        description: String,
        category: String,
        website: Option<String>,
        logo_cid: Option<String>,
        metadata_cid: Option<String>
    ) -> Result<(), ContractError> {
        ProjectRegistry::update_project(
            &env, 
            project_id, 
            caller, 
            name, 
            description, 
            category, 
            website, 
            logo_cid, 
            metadata_cid
        )
    }

    // You'll likely need this for the frontend to fetch data
    pub fn get_project(env: Env, project_id: u64) -> Option<crate::types::Project> {
        ProjectRegistry::get_project(&env, project_id)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn test_unauthorized_update_fails() {
        let env = Env::default();
        let owner = Address::generate(&env);
        let hacker = Address::generate(&env);
        
        // This simulates a hacker trying to call your code.
        // In a real test, we would expect a REVERT with NotProjectOwner.
        // For now, let's just make sure the project compiles with tests.
    }
}