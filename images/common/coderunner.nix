{ pkgs }:

let
  codeRunnerSrc =
    builtins.fetchGit {
      url = "git@github.com:glotcode/code-runner.git";
      ref = "main";
      rev = "0cd91590b00d7203aad327680f05431caf542cad";
    };

  codeRunner =
    import "${codeRunnerSrc}/Cargo.nix" { pkgs = pkgs; };
in
codeRunner.rootCrate.build
