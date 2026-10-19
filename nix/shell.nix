{
  craneLib,
  rust-analyzer-nightly,
  taplo,
}:
craneLib.devShell {
  packages = [
    rust-analyzer-nightly
    taplo
  ];
}
