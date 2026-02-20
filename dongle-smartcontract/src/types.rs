use soroban_sdk::{contracttype, Address, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Project(u64),
    Review(u64, Address), // Project ID and Reviewer Address
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Project {
    pub id: u64,
    pub owner: Address,
    pub name: String,
    pub description: String,
    pub category: String,
    pub website: Option<String>,
    pub logo_cid: Option<String>,
    pub metadata_cid: Option<String>,
}