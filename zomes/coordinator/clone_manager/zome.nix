{ inputs, ... }:

{
  perSystem = { inputs', system, self', ... }: {
    packages.clone_manager =
      inputs.tnesh-stack.outputs.builders.${system}.rustZome {
        workspacePath = inputs.self.outPath;
        crateCargoToml = ./Cargo.toml;
        excludedCrates = [ "clone_manager_utils" ];
      };

  };
}

