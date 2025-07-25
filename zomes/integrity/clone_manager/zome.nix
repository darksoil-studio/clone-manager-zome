{ inputs, ... }:

{
  perSystem = { inputs', system, ... }: {
    packages.clone_manager_integrity =
      inputs.holochain-utils.outputs.builders.${system}.rustZome {
        workspacePath = inputs.self.outPath;
        crateCargoToml = ./Cargo.toml;
        excludedCrates = [ "clone_manager_utils" ];
      };
  };
}

