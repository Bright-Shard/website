{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {
  name = "brightshard-website-shell";

  packages =
    with pkgs;
    with python313Packages;
    [
      brotli
      fonttools
    ];
}
