use clone_manager_integrity::LinkTypes;
use hdk::prelude::*;

use crate::utils::{create_link_relaxed, delete_link_relaxed};

#[hdk_extern]
pub fn announce_as_clone_provider_for_request(clone_request: EntryHash) -> ExternResult<()> {
    let my_pub_key = agent_info()?.agent_initial_pubkey;

    info!(
        "Announcing as clone provider for {} with pub key {}.",
        clone_request, my_pub_key
    );

    create_link_relaxed(
        clone_request,
        my_pub_key,
        LinkTypes::CloneProviderForRequest,
        (),
    )?;

    Ok(())
}

#[hdk_extern]
pub fn retract_as_clone_provider_for_request(clone_request: EntryHash) -> ExternResult<()> {
    let my_pub_key = agent_info()?.agent_initial_pubkey;

    info!(
        "Retracting as clone provider for {} with pub key {}.",
        clone_request, my_pub_key
    );

    let links = get_links(
        GetLinksInputBuilder::try_new(clone_request, LinkTypes::CloneProviderForRequest)?.build(),
    )?;

    for link in links {
        let Some(agent) = link.target.into_agent_pub_key() else {
            continue;
        };

        if agent.eq(&my_pub_key) {
            delete_link_relaxed(link.create_link_hash)?;
        }
    }

    Ok(())
}

#[hdk_extern]
pub fn get_clone_providers_for_request(clone_request: EntryHash) -> ExternResult<Vec<AgentPubKey>> {
    let links = get_links(
        GetLinksInputBuilder::try_new(clone_request, LinkTypes::CloneProviderForRequest)?.build(),
    )?;

    let providers_pub_keys = links
        .into_iter()
        .filter_map(|link| link.target.into_agent_pub_key())
        .collect();

    Ok(providers_pub_keys)
}
