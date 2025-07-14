use clone_manager_integrity::*;
use hdk::prelude::*;

use crate::utils::{create_link_relaxed, create_relaxed, delete_link_relaxed, delete_relaxed};

#[hdk_extern]
pub fn create_clone_request(clone_request: CloneRequest) -> ExternResult<EntryHash> {
    let entry_hash = hash_entry(&clone_request)?;
    create_relaxed(EntryTypes::CloneRequest(clone_request))?;
    let path = Path::from("all_clone_requests");
    create_link_relaxed(
        path.path_entry_hash()?,
        entry_hash.clone(),
        LinkTypes::AllCloneRequests,
        (),
    )?;
    Ok(entry_hash)
}

#[hdk_extern]
pub fn get_clone_request(clone_request_hash: EntryHash) -> ExternResult<Option<CloneRequest>> {
    let Some(details) = get_details(clone_request_hash, GetOptions::default())? else {
        return Ok(None);
    };
    match details {
        Details::Entry(details) => {
            let entry = CloneRequest::try_from(details.entry)?;
            Ok(Some(entry))
        }
        _ => Err(wasm_error!(WasmErrorInner::Guest(
            "Malformed get details response".to_string()
        ))),
    }
}

#[hdk_extern]
pub fn delete_clone_request(clone_request_hash: EntryHash) -> ExternResult<()> {
    let path = Path::from("all_clone_requests");
    let links = get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllCloneRequests)?
            .build(),
    )?;
    for link in links {
        if let Some(hash) = link.target.into_entry_hash() {
            if hash == clone_request_hash {
                delete_link_relaxed(link.create_link_hash)?;
            }
        }
    }

    let Some(Details::Entry(entry_details)) =
        get_details(clone_request_hash, GetOptions::default())?
    else {
        return Err(wasm_error!("Clone request not found"));
    };
    for create in entry_details.actions {
        delete_relaxed(create.hashed.hash)?;
    }

    Ok(())
}

#[hdk_extern]
pub fn get_all_deletes_for_clone_request(
    original_clone_request_hash: EntryHash,
) -> ExternResult<Option<Vec<SignedActionHashed>>> {
    let Some(details) = get_details(original_clone_request_hash, GetOptions::default())? else {
        return Ok(None);
    };
    match details {
        Details::Record(_) => Err(wasm_error!(WasmErrorInner::Guest(
            "Malformed details".into()
        ))),
        Details::Entry(record_details) => Ok(Some(record_details.deletes)),
    }
}

#[hdk_extern]
pub fn get_oldest_delete_for_clone_request(
    original_clone_request_hash: EntryHash,
) -> ExternResult<Option<SignedActionHashed>> {
    let Some(mut deletes) = get_all_deletes_for_clone_request(original_clone_request_hash)? else {
        return Ok(None);
    };
    deletes.sort_by(|delete_a, delete_b| {
        delete_a
            .action()
            .timestamp()
            .cmp(&delete_b.action().timestamp())
    });
    Ok(deletes.first().cloned())
}
