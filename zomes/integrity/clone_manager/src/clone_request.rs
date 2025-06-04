pub use clone_manager_types::CloneRequest;
use hdi::prelude::*;

pub fn validate_create_clone_request(
    _action: EntryCreationAction,
    _clone_request: CloneRequest,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_update_clone_request(
    _action: Update,
    _clone_request: CloneRequest,
    _original_action: EntryCreationAction,
    _original_clone_request: CloneRequest,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "Clone Requests cannot be updated".to_string(),
    ))
}

pub fn validate_delete_clone_request(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_clone_request: CloneRequest,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_create_link_all_clone_requests(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let entry_hash = target_address
        .into_entry_hash()
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "No action hash associated with link".to_string()
        )))?;
    let entry = must_get_entry(entry_hash)?;
    let Ok(_clone_request) = crate::CloneRequest::try_from(entry.content) else {
        return Ok(ValidateCallbackResult::Invalid(
            "Linked action must reference an entry".to_string(),
        ));
    };
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_all_clone_requests(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}
