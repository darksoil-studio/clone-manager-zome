use hdi::prelude::*;

#[derive(Clone, PartialEq)]
#[hdk_entry_helper]
pub struct CloneRequest {
    pub dna_modifiers: DnaModifiers,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewCloneRequest {
    pub clone_request_hash: EntryHash,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CloneRequestWithDeleted {
    pub clone_request: CloneRequest,
    pub deleted: bool,
}
