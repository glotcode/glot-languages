{ pkgs }:

let
  codeRunnerSrc =
    builtins.fetchGit {
      url = "https://github.com/glotcode/code-runner.git";
      ref = "main";
      rev = "3b87378c36bd457624cc15c1c2f0dac32f33e69f";
    };

  codeRunner =
    import "${codeRunnerSrc}/Cargo.nix" { pkgs = pkgs; };
in
codeRunner.rootCrate.build
