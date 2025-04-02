use anyhow::anyhow;
use clone_manager_types::CloneRequest;
use hdk::prelude::{CloneCellId, DnaModifiers, DnaModifiersOpt, RoleName, YamlProperties};
use holochain_client::{AdminWebsocket, AppWebsocket, CellInfo, ExternIO, ZomeCallTarget};
use holochain_types::app::{CreateCloneCellPayload, EnableCloneCellPayload};

pub async fn reconcile_cloned_cells(
    admin_ws: &AdminWebsocket,
    app_ws: &AppWebsocket,
    clone_manager_zome_role: RoleName,
    role_to_clone: RoleName,
) -> anyhow::Result<()> {
    let clone_requests: Vec<CloneRequest> = app_ws
        .call_zome(
            ZomeCallTarget::RoleName(clone_manager_zome_role),
            "clone_manager".into(),
            "get_all_clone_requests".into(),
            ExternIO::encode(())?,
        )
        .await?
        .decode()?;

    let Some(app_info) = app_ws.app_info().await? else {
        return Err(anyhow!("App is not installed."));
    };

    let service_providers_cells = app_info
        .cell_info
        .get(&role_to_clone)
        .cloned()
        .unwrap_or(vec![]);

    for clone_request in clone_requests {
        let existing_clone = service_providers_cells
            .iter()
            .find(|cell| dna_modifiers(cell).eq(&clone_request.dna_modifiers));

        if let None = existing_clone {
            clone_cell(&admin_ws, &app_ws, role_to_clone.clone(), clone_request).await?;
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

pub async fn clone_cell(
    admin_ws: &AdminWebsocket,
    app_ws: &AppWebsocket,
    role_to_clone: RoleName,
    clone_request: CloneRequest,
) -> anyhow::Result<()> {
    let properties = YamlProperties::try_from(clone_request.dna_modifiers.properties)?;

    log::info!(
        "New CloneRequest received. Cloning the {} role.",
        role_to_clone
    );

    let cell = app_ws
        .create_clone_cell(CreateCloneCellPayload {
            role_name: role_to_clone,
            modifiers: DnaModifiersOpt {
                network_seed: Some(clone_request.dna_modifiers.network_seed.clone()),
                origin_time: Some(clone_request.dna_modifiers.origin_time),
                quantum_time: Some(clone_request.dna_modifiers.quantum_time),
                properties: Some(properties.clone()),
            },
            membrane_proof: None,
            name: None,
        })
        .await?;
    app_ws
        .enable_clone_cell(EnableCloneCellPayload {
            clone_cell_id: CloneCellId::CloneId(cell.clone_id.clone()),
        })
        .await?;

    let dna_def = admin_ws
        .get_dna_definition(cell.cell_id.dna_hash().clone())
        .await?;

    if let Some((first_zome, _)) = dna_def.coordinator_zomes.first() {
        app_ws
            .call_zome(
                ZomeCallTarget::CellId(cell.cell_id.clone()),
                first_zome.clone(),
                "init".into(),
                ExternIO::encode(())?,
            )
            .await?;
    }

    log::info!("New cloned cell: {cell:?}.");

    Ok(())
}
