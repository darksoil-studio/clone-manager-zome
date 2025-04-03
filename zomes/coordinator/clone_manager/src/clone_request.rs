use clone_manager_integrity::*;
use hdk::prelude::*;

#[hdk_extern]
pub fn create_clone_request(clone_request: CloneRequest) -> ExternResult<Record> {
    let clone_request_hash = create_entry(&EntryTypes::CloneRequest(clone_request.clone()))?;
    let record = get(clone_request_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the newly created CloneRequest".to_string())
    ))?;
    let path = Path::from("all_clone_requests");
    create_link(
        path.path_entry_hash()?,
        clone_request_hash.clone(),
        LinkTypes::AllCloneRequests,
        (),
    )?;
    Ok(record)
}

#[hdk_extern]
pub fn get_clone_request(clone_request_hash: ActionHash) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(clone_request_hash, GetOptions::default())? else {
        return Ok(None);
    };
    match details {
        Details::Record(details) => Ok(Some(details.record)),
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
                delete_link(link.create_link_hash)?;
            }
        }
    }

    let Some(Details::Entry(entry_details)) =
        get_details(clone_request_hash, GetOptions::default())?
    else {
        return Err(wasm_error!("Clone request not found"));
    };
    for create in entry_details.actions {
        delete_entry(create.hashed.hash)?;
    }

    Ok(())
}

#[hdk_extern]
pub fn get_all_deletes_for_clone_request(
    original_clone_request_hash: ActionHash,
) -> ExternResult<Option<Vec<SignedActionHashed>>> {
    let Some(details) = get_details(original_clone_request_hash, GetOptions::default())? else {
        return Ok(None);
    };
    match details {
        Details::Entry(_) => Err(wasm_error!(WasmErrorInner::Guest(
            "Malformed details".into()
        ))),
        Details::Record(record_details) => Ok(Some(record_details.deletes)),
    }
}

#[hdk_extern]
pub fn get_oldest_delete_for_clone_request(
    original_clone_request_hash: ActionHash,
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
