{ inputs, ... }:

{
  perSystem = { inputs', lib, self', system, ... }: {
    packages.clone_manager_test_happ =
      inputs.holochain-nix-builders.outputs.builders.${system}.happ {
        happManifest = ./happ.yaml;

        dnas = {
          # Include here the DNA packages for this hApp, e.g.:
          # my_dna = inputs'.some_input.packages.my_dna;
          # This overrides all the "bundled" properties for the hApp manifest 
          clone_manager_test = self'.packages.clone_manager_test_dna;
        };
      };
  };
}
