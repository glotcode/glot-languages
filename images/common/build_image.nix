{ pkgs, name, tag, installedPackages, runCommands ? "" }:

let
  codeRunnerSrc =
    builtins.fetchGit {
      url = "git@github.com:glotcode/code-runner.git";
      ref = "main";
      rev = "cd48dde66a6218d6db919db883ced959a8786a4b";
    };

  codeRunner =
    (import "${codeRunnerSrc}/Cargo.nix" { pkgs = pkgs; }).rootCrate.build;

  initializeShadow = [
    (pkgs.writeTextDir "etc/shadow" ''
      root:!x:::::::
    '')

    (pkgs.writeTextDir "etc/passwd" ''
      root:x:0:0::/root:/dev/null
    '')

    (pkgs.writeTextDir "etc/group" ''
      root:x:0:
    '')

    (pkgs.writeTextDir "etc/gshadow" ''
      root:x::
    '')
  ];

  commonPackages = [
    pkgs.bash
    pkgs.coreutils
    pkgs.shadow
  ];
in
pkgs.dockerTools.buildImage {
  name = "glot/python";
  tag = "beta";
  created = "now";

  contents =
    pkgs.lib.concatLists [
      initializeShadow
      commonPackages
      installedPackages
    ];

  runAsRoot = ''
    #!/bin/bash
    ${pkgs.shadow}/bin/groupadd glot
    ${pkgs.shadow}/bin/useradd -m -d /home/glot -g glot -s /bin/bash glot
    ${runCommands}
  '';

  config = {
    Cmd = [ "${codeRunner}/bin/code-runner" "--path" "/home/glot"];
  };
}
