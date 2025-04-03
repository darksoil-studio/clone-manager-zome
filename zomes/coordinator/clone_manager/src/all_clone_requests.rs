use clone_manager_integrity::*;
use hdk::prelude::*;
use std::collections::BTreeMap;

#[hdk_extern]
pub fn get_all_clone_requests() -> ExternResult<BTreeMap<EntryHashB64, CloneRequest>> {
    let path = Path::from("all_clone_requests");
    let links = get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllCloneRequests)?
            .build(),
    )?;
    let get_inputs: Vec<GetInput> = links
        .into_iter()
        .filter_map(|link| link.target.into_entry_hash())
        .map(|entry_hash| GetInput::new(entry_hash.into(), GetOptions::default()))
        .collect();
    let maybe_records = HDK.with(|hdk| hdk.borrow().get(get_inputs))?;
    let clone_requests = maybe_records
        .into_iter()
        .filter_map(|r| r)
        .filter_map(|r| {
            let Some(entry) = r.entry().as_option().cloned() else {
                return None;
            };
            let Ok(clone_request) = CloneRequest::try_from(entry) else {
                return None;
            };
            let Some(entry_hash) = r.action().entry_hash().clone() else {
                return None;
            };
            return Some((entry_hash.clone().into(), clone_request));
        })
        .collect();
    Ok(clone_requests)
}
