{ pkgs }:

let
  codeRunnerSrc =
    builtins.fetchGit {
      url = "https://github.com/glotcode/glot-images.git";
      ref = "main";
      rev = "15aae9e6b3473f42ed66898be9d100d9d78e04a7";
    };

  codeRunner =
    import "${codeRunnerSrc}/Cargo.nix" { pkgs = pkgs; };
in
codeRunner.rootCrate.build
