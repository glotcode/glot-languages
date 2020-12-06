{
  pkgs,
  name,
  tag,
  installedPackages,
  run ? "",
  env ? [],
  keepContentsDirlinks ? false
}:

let
  codeRunnerSrc =
    builtins.fetchGit {
      url = "git@github.com:glotcode/code-runner.git";
      ref = "main";
      rev = "8735bfb77f601f082a0513de351f462dc4bb050c";
    };

  codeRunner =
    (import "${codeRunnerSrc}/Cargo.nix" { pkgs = pkgs; }).rootCrate.build;

  commonPackages = [
    pkgs.bash
    pkgs.coreutils
    pkgs.glibcLocales
  ];

  commonEnv = [
    "LANG=en_US.UTF-8"
    "LOCALE_ARCHIVE=${pkgs.glibcLocales}/lib/locale/locale-archive"
  ];
in
pkgs.dockerTools.buildImage {
  name = name;
  tag = tag;
  created = "now";

  contents =
    pkgs.lib.concatLists [
      commonPackages
      installedPackages
    ];

  diskSize = 8192;
  keepContentsDirlinks = keepContentsDirlinks;

  runAsRoot = ''
    ${pkgs.stdenv.shell}
    ${pkgs.dockerTools.shadowSetup}
    ${pkgs.shadow}/bin/groupadd glot
    ${pkgs.shadow}/bin/useradd -m -d /home/glot -g glot -s /bin/bash glot
    ${pkgs.coreutils}/bin/mkdir /tmp
    ${pkgs.coreutils}/bin/chmod 1777 /tmp
    ${run}
  '';

  config = {
    Env =
      pkgs.lib.concatLists [
        commonEnv
        env
      ];

    Cmd = [ "${codeRunner}/bin/code-runner" "--path" "/home/glot"];
  };
}
