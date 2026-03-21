{ pkgs, lib, config, inputs, ... }:
{
  packages = with pkgs; [
    git
    jujutsu
  ];

  languages = {
    rust = {
      enable = true;
      channel = "nightly";
      lsp.enable = true;
    };

    c.enable = true;
    cplusplus.enable = true;
  };
}
