use std::collections::BTreeMap;

use anyhow::anyhow;
use clone_manager_types::CloneRequest;
use hdk::prelude::{
    CloneCellId, DnaModifiers, DnaModifiersOpt, Entry, EntryHash, EntryHashB64, RoleName,
    YamlProperties,
};
use holochain_client::{
    AdminWebsocket, AppWebsocket, CellInfo, ClonedCell, ExternIO, SerializedBytes, ZomeCallTarget,
};
use holochain_types::{
    app::{CreateCloneCellPayload, DisableCloneCellPayload, EnableCloneCellPayload},
    EntryHashed,
};

pub async fn reconcile_cloned_cells(
    admin_ws: &AdminWebsocket,
    app_ws: &AppWebsocket,
    clone_manager_zome_role: RoleName,
    role_to_clone: RoleName,
) -> anyhow::Result<()> {
    let clone_requests: BTreeMap<EntryHashB64, CloneRequest> = app_ws
        .call_zome(
            ZomeCallTarget::RoleName(clone_manager_zome_role.clone()),
            "clone_manager".into(),
            "get_all_clone_requests".into(),
            ExternIO::encode(())?,
        )
        .await?
        .decode()?;

    let Some(app_info) = app_ws.app_info().await? else {
        return Err(anyhow!("App is not installed."));
    };

    log::info!(
        "Reconciling cloned cells. Current clone requests: {:?}",
        clone_requests
    );

    let cloned_cells: Vec<ClonedCell> = app_info
        .cell_info
        .get(&role_to_clone)
        .cloned()
        .unwrap_or(vec![])
        .into_iter()
        .filter_map(|cell_info| match cell_info {
            CellInfo::Cloned(cloned) => Some(cloned),
            _ => None,
        })
        .collect();

    for (entry_hash, clone_request) in clone_requests.iter() {
        let existing_clone = cloned_cells
            .iter()
            .find(|cell| cell.dna_modifiers.eq(&clone_request.dna_modifiers));

        if let Some(existing_clone) = existing_clone {
            if !existing_clone.enabled {
                app_ws
                    .enable_clone_cell(EnableCloneCellPayload {
                        clone_cell_id: CloneCellId::CloneId(existing_clone.clone_id.clone()),
                    })
                    .await?;

                log::info!("Reenabled cloned cell: {:?}.", existing_clone);

                let _r: () = app_ws
                    .call_zome(
                        ZomeCallTarget::RoleName(clone_manager_zome_role.clone()),
                        "clone_manager".into(),
                        "announce_as_clone_provider_for_request".into(),
                        ExternIO::encode(EntryHash::from(entry_hash.clone()))?,
                    )
                    .await?
                    .decode()?;
            } else {
                log::info!("Cell is already enabled: doing nothing.");
            }
        } else {
            let properties =
                YamlProperties::try_from(clone_request.dna_modifiers.properties.clone())?;
            let cloned_cell = app_ws
                .create_clone_cell(CreateCloneCellPayload {
                    role_name: role_to_clone.clone(),
                    modifiers: DnaModifiersOpt {
                        network_seed: Some(clone_request.dna_modifiers.network_seed.clone()),
                        properties: Some(properties.clone()),
                    },
                    membrane_proof: None,
                    name: None,
                })
                .await?;
            app_ws
                .enable_clone_cell(EnableCloneCellPayload {
                    clone_cell_id: CloneCellId::CloneId(cloned_cell.clone_id.clone()),
                })
                .await?;

            let dna_def = admin_ws
                .get_dna_definition(cloned_cell.cell_id.dna_hash().clone())
                .await?;

            if let Some((first_zome, _)) = dna_def.coordinator_zomes.first() {
                app_ws
                    .call_zome(
                        ZomeCallTarget::CellId(cloned_cell.cell_id.clone()),
                        first_zome.clone(),
                        "init".into(),
                        ExternIO::encode(())?,
                    )
                    .await?;
            }

            log::info!("New cloned cell: {cloned_cell:?}.");

            let _r: () = app_ws
                .call_zome(
                    ZomeCallTarget::RoleName(clone_manager_zome_role.clone()),
                    "clone_manager".into(),
                    "announce_as_clone_provider_for_request".into(),
                    ExternIO::encode(EntryHash::from(entry_hash.clone()))?,
                )
                .await?
                .decode()?;
        }
    }

    // Disable cells that are not longer requested to exist

    for cloned_cell in cloned_cells {
        // If the cell is already disabled, we don't need to disable it again
        if !cloned_cell.enabled {
            continue;
        }

        if clone_requests
            .values()
            .find(|clone_request| clone_request.dna_modifiers.eq(&cloned_cell.dna_modifiers))
            .is_none()
        {
            log::info!(
                "CloneRequest for role {} with DNA hash {} and modifiers {:?} does not longer exist. Disabling the cell.",
                role_to_clone, cloned_cell.cell_id.dna_hash(), cloned_cell.dna_modifiers
            );
            app_ws
                .disable_clone_cell(DisableCloneCellPayload {
                    clone_cell_id: CloneCellId::CloneId(cloned_cell.clone_id),
                })
                .await?;

            let entry_hashed = EntryHashed::from_content_sync(Entry::app(
                SerializedBytes::try_from(CloneRequest {
                    dna_modifiers: cloned_cell.dna_modifiers,
                })?,
            )?);

            // TODO: be careful when adding more properties to CloneRequest:
            // can the EntryHash be computed on the fly and passed in as an input to `delete_clone_provided_for_request()`?
            let _r: () = app_ws
                .call_zome(
                    ZomeCallTarget::RoleName(clone_manager_zome_role.clone()),
                    "clone_manager".into(),
                    "retract_as_clone_provider_for_request".into(),
                    ExternIO::encode(entry_hashed.hash)?,
                )
                .await?
                .decode()?;
        }
    }

    Ok(())
}

pub fn dna_modifiers(cell: &CellInfo) -> DnaModifiers {
    match cell {
        CellInfo::Provisioned(provisioned) => provisioned.dna_modifiers.clone(),
        CellInfo::Cloned(cloned) => cloned.dna_modifiers.clone(),
        CellInfo::Stem(stem) => stem.dna_modifiers.clone(),
    }
}
