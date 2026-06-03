{
    description = "Flake for Holochain library development";

    inputs = {
        holonix.url = "github:holochain/holonix?ref=main-0.6";
        nixpkgs.follows = "holonix/nixpkgs";
        flake-parts.follows = "holonix/flake-parts";
    };

    outputs = inputs @ { flake-parts, holonix, ... }:
    flake-parts.lib.mkFlake { inherit inputs; }
    {
        systems = builtins.attrNames holonix.devShells;
        perSystem = { config, pkgs, system, ... }: {
            devShells.default = pkgs.mkShell {
                inputsFrom = [
                    holonix.devShells.${system}.default
                ];
                packages = [
                    (pkgs.python3.withPackages (python-pkgs: [
                        python-pkgs.pip
                    ]))
                ];
            };
        };
    };
}
