{ inputs, ... }:

{
  perSystem = { inputs', self', lib, system, ... }: {
    packages.clone_manager_test_dna =
      inputs.holochain-utils.outputs.builders.${system}.dna {
        dnaManifest = ./dna.yaml;
        zomes = {
          clone_manager_integrity = self'.packages.clone_manager_integrity;
          clone_manager = self'.packages.clone_manager;
        };
      };
  };
}

