use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    ProjectNotFound = 1,
    NotProjectOwner = 2,
    ReviewNotFound = 3,
    NotReviewer = 4,
}