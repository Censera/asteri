{
	description = "asteri* dev env";

	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
	};

	outputs = { self, nixpkgs }:
	let
	system = "x86_64-linux";
	pkgs = nixpkgs.legacyPackages.${system};
	in
	{
		devShells.${system}.default = pkgs.mkShell {
		buildInputs = with pkgs; [
			rustc
      cargo
      libiconv
		];

		shellHook = ''
			clear
      echo "asteri*"
			echo -e "\n--------------------------------------------------------------\n"
			ls
			echo -e "\n--------------------------------------------------------------\n"
		'';
		};
	};
}
