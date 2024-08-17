{ pkgs }:

let
  codeRunnerSrc =
    builtins.fetchGit {
      url = "https://github.com/glotcode/code-runner.git";
      ref = "main";
      rev = "f5ed5bce956e4da607bbb94d692678f3a807a917";
    };

  codeRunner =
    import "${codeRunnerSrc}/Cargo.nix" { pkgs = pkgs; };
in
codeRunner.rootCrate.build
