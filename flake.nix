{
  description = "Template for Holochain app development";

  inputs = {
    holonix.url = "github:holochain/holonix/main-0.5";
    nixpkgs.follows = "holonix/nixpkgs";

    holochain-utils.url = "github:darksoil-studio/holochain-utils/main-0.5";
    holochain-utils.inputs.holonix.follows = "holonix";
  };

  nixConfig = {
    extra-substituters = [
      "https://holochain-ci.cachix.org"
      "https://darksoil-studio.cachix.org"
    ];
    extra-trusted-public-keys = [
      "holochain-ci.cachix.org-1:5IUSkZc0aoRS53rfkvH9Kid40NpyjwCMCzwRTXy+QN8="
      "darksoil-studio.cachix.org-1:UEi+aujy44s41XL/pscLw37KEVpTEIn8N/kn7jO8rkc="
    ];
  };

  outputs = inputs:
    inputs.holonix.inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        ./zomes/integrity/clone_manager/zome.nix
        ./zomes/coordinator/clone_manager/zome.nix
        # Just for testing purposes
        ./workdir/dna.nix
        ./workdir/happ.nix
        inputs.holochain-utils.outputs.flakeModules.builders
      ];

      systems = builtins.attrNames inputs.holonix.devShells;
      perSystem = { inputs', config, pkgs, system, ... }: {
        devShells.default = pkgs.mkShell {
          inputsFrom = [
            inputs'.holochain-utils.devShells.synchronized-pnpm
            inputs'.holonix.devShells.default
          ];

          packages = [
            inputs'.holochain-utils.packages.holochain
            inputs'.holochain-utils.packages.hc-scaffold-zome
          ];
        };
        devShells.npm-ci = inputs'.holochain-utils.devShells.synchronized-pnpm;

        packages.scaffold = pkgs.symlinkJoin {
          name = "scaffold-remote-zome";
          paths = [ inputs'.holochain-utils.packages.scaffold-remote-zome ];
          buildInputs = [ pkgs.makeWrapper ];
          postBuild = ''
            wrapProgram $out/bin/scaffold-remote-zome \
              --add-flags "clone-manager-zome \
                --integrity-zome-name clone_manager_integrity \
                --coordinator-zome-name clone_manager \
                --remote-zome-git-url github:darksoil-studio/clone-manager-zome \
                --remote-npm-package-name @darksoil-studio/clone-manager-zome \
                --remote-zome-git-branch main-0.5 \
                --context-element clone-manager-context \
                --context-element-import @darksoil-studio/clone-manager-zome/dist/elements/clone-manager-context.js" 
          '';
        };
      };
    };
}
