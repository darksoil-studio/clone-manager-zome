{ inputs, ... }:

{
  perSystem = { inputs', system, self', ... }: rec {
    packages.clone_manager =
      inputs.holochain-utils.outputs.builders.${system}.rustZome {
        workspacePath = inputs.self.outPath;
        crateCargoToml = ./Cargo.toml;
        excludedCrates = [ "clone_manager_utils" ];
      };

    builders.clone_manager = { provider }:
      inputs.holochain-utils.outputs.builders.${system}.rustZome {
        workspacePath = inputs.self.outPath;
        crateCargoToml = ./Cargo.toml;
        excludedCrates = [ "clone_manager_utils" ];
        zomeEnvironmentVars = {
          CLONE_PROVIDER = "${builtins.toString provider}";
        };
      };

    packages.clone_manager_provider =
      builders.clone_manager { provider = true; };
  };
}

