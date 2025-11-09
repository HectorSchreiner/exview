{
  description = "LogRhythm Alarm System";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Frontend
            nodejs_20
            nodePackages.npm
            
            # Backend
            rustc
            cargo
            rust-analyzer
            clippy
            rustfmt
            
            # Database
            postgresql_16
            
            # Docker
            docker
            docker-compose
          ];

          shellHook = ''
            echo "🚀 LogRhythm Alarm System Development Environment"
            echo "Node: $(node --version)"
            echo "Cargo: $(cargo --version)"
            echo "Docker: $(docker --version)"
          '';
        };
      }
    );
}
