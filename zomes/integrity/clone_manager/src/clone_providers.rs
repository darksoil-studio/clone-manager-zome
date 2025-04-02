use hdi::prelude::*;
pub use clone_manager_types::CloneRequest;

pub fn validate_create_link_clone_providers(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let Some(_) = base_address.into_entry_hash() else {
        return Ok(ValidateCallbackResult::Invalid(String::from(
            "Base address for a CloneProviders link must be an entry hash",
        )));
    };
    let Some(_) = target_address.into_agent_pub_key() else {
        return Ok(ValidateCallbackResult::Invalid(String::from(
            "Base address for a CloneProviders link must be an AgentPubKey",
        )));
    };
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_clone_providers(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}
