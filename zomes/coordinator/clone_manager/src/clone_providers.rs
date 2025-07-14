use clone_manager_integrity::LinkTypes;
use hdk::prelude::*;

use crate::utils::{create_cap_grant_relaxed, create_link_relaxed};

fn all_providers_path() -> Path {
    Path::from(format!("all_providers"))
}

#[hdk_extern]
pub fn announce_as_provider() -> ExternResult<()> {
    let agent_info = agent_info()?;
    let dna_info = dna_info()?;

    info!(
        "Announcing as a clone manager provider of dna {} with pub key {}.",
        dna_info.hash, agent_info.agent_initial_pubkey
    );

    let path = all_providers_path();

    create_link_relaxed(
        path.path_entry_hash()?,
        agent_info.agent_initial_pubkey,
        LinkTypes::CloneProviders,
        (),
    )?;

    let functions = GrantedFunctions::Listed(BTreeSet::from([
        (
            zome_info()?.name,
            FunctionName::from("available_as_provider"),
        ),
        (zome_info()?.name, FunctionName::from("recv_remote_signal")),
    ]));

    create_cap_grant_relaxed(CapGrantEntry {
        tag: "clone_manager_provider".into(),
        // empty access converts to unrestricted
        access: CapAccess::Unrestricted,
        functions,
    })?;

    Ok(())
}

#[hdk_extern]
pub fn get_clone_providers() -> ExternResult<Vec<AgentPubKey>> {
    let links = get_links(
        GetLinksInputBuilder::try_new(
            all_providers_path().path_entry_hash()?,
            LinkTypes::CloneProviders,
        )?
        .build(),
    )?;

    let providers_pub_keys = links
        .into_iter()
        .filter_map(|link| link.target.into_agent_pub_key())
        .collect();

    Ok(providers_pub_keys)
}
