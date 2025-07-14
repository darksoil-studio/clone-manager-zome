pub use clone_manager_types::CloneRequest;
use hdi::prelude::*;

pub fn validate_create_link_clone_provider_for_request(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let Some(_) = base_address.into_entry_hash() else {
        return Ok(ValidateCallbackResult::Invalid(String::from(
            "Base address for a CloneProviderForRequest link must be an EntryHash",
        )));
    };
    let Some(_) = target_address.into_agent_pub_key() else {
        return Ok(ValidateCallbackResult::Invalid(String::from(
            "Target address for a CloneProviderForRequest link must be an AgentPubKey",
        )));
    };
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_clone_provider_for_request(
    action: DeleteLink,
    original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    if action.author.ne(&original_action.author) {
        return Ok(ValidateCallbackResult::Invalid(String::from(
            "Only authors can delete their own clone provider for request links",
        )));
    }

    Ok(ValidateCallbackResult::Valid)
}
