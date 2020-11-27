{ pkgs, name, tag, installedPackages, runCommands ? "" }:

let
  codeRunnerSrc =
    builtins.fetchGit {
      url = "git@github.com:glotcode/code-runner.git";
      ref = "main";
      rev = "d0a985b4f8a27a5e0df47bd34f939323d1b81154";
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
  ];
in
pkgs.dockerTools.buildImage {
  name = name;
  tag = tag;
  created = "now";

  contents =
    pkgs.lib.concatLists [
      initializeShadow
      commonPackages
      installedPackages
    ];

  runAsRoot = ''
    ${pkgs.stdenv.shell}
    ${pkgs.shadow}/bin/groupadd glot
    ${pkgs.shadow}/bin/useradd -m -d /home/glot -g glot -s /bin/bash glot
    ${pkgs.coreutils}/bin/mkdir /tmp
    ${pkgs.coreutils}/bin/chmod 1777 /tmp
    ${runCommands}
  '';

  config = {
    Cmd = [ "${codeRunner}/bin/code-runner" "--path" "/home/glot"];
  };
}
