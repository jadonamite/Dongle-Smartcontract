use crate::types::{Project, DataKey};
use crate::errors::ContractError;
use soroban_sdk::{Env, Address, String};

pub struct ProjectRegistry;

impl ProjectRegistry {

    /// this is to Retrieve the project from persistent storage

    pub fn get_project(env: &Env, project_id: u64) -> Option<Project> {
        env.storage().persistent().get(&DataKey::Project(project_id))
    }

    pub fn update_project(
        env: &Env, 
        project_id: u64, 
        caller: Address, 
        name: String,
        description: String,
        category: String,
        website: Option<String>,
        logo_cid: Option<String>,
        metadata_cid: Option<String>
    ) -> Result<(), ContractError> {

      
        // Proves 'caller' is who they claim to be via cryptographic signature.
        caller.require_auth();

     
        // Tries to find the project; returns Error if it doesn't exist.
        let mut project: Project = Self::get_project(env, project_id)
            .ok_or(ContractError::ProjectNotFound)?;

       
        // Compares the authenticated 'caller' with the stored 'project.owner'.
        if caller != project.owner {
            return Err(ContractError::NotProjectOwner);
        }

   
        // If we reached here, the user IS the owner.
        project.name = name;
        project.description = description;
        project.category = category;
        project.website = website;
        project.logo_cid = logo_cid;
        project.metadata_cid = metadata_cid;

      
        // Save the updated object back to the ledger.
        env.storage().persistent().set(&DataKey::Project(project_id), &project);

        Ok(())
    }
}