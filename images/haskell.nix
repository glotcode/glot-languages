let
  pkgs =
    import ../common/nixpkgs.nix;

  build_image =
    import ../common/build_image.nix;

  haskellPackages =
    pkgs.haskellPackages.ghcWithPackages (ps: [
      ps.aeson
      ps.async
      ps.attoparsec
      ps.case-insensitive
      ps.cgi
      ps.clock
      ps.exceptions
      ps.fgl
      ps.formatting
      ps.free
      ps.hashable
      ps.html
      ps.mtl
      ps.multipart
      ps.network
      ps.network-uri
      ps.ObjectName
      ps.old-locale
      ps.old-time
      ps.parallel
      ps.parsec
      ps.primitive
      ps.QuickCheck
      ps.random
      ps.regex-base
      ps.regex-compat
      ps.regex-posix
      ps.scientific
      ps.split
      ps.StateVar
      ps.stm
      ps.syb
      ps.text
      ps.tf-random
      ps.transformers
      ps.transformers-compat
      ps.unordered-containers
      ps.vector
      ps.xhtml
      ps.zlib
    ]);
in
build_image {
  pkgs = pkgs;
  name = "glot/haskell";
  tag = "latest";
  installedPackages = [
    haskellPackages
  ];
}
