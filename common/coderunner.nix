{ pkgs }:

let
  codeRunnerSrc =
    builtins.fetchGit {
      url = "https://github.com/glotcode/code-runner.git";
      ref = "main";
      rev = "c473f13189b3a54271a4586dcb4fc022550fa029";
    };

  codeRunner =
    import "${codeRunnerSrc}/Cargo.nix" { pkgs = pkgs; };
in
codeRunner.rootCrate.build
