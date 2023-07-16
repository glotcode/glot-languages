{ pkgs }:

let
  codeRunnerSrc =
    builtins.fetchGit {
      url = "https://github.com/glotcode/code-runner.git";
      ref = "main";
      rev = "f77b7a54ad098e95f67757366125f3c57c1ddcfb";
    };

  codeRunner =
    import "${codeRunnerSrc}/Cargo.nix" { pkgs = pkgs; };
in
codeRunner.rootCrate.build
