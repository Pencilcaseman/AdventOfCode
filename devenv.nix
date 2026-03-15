{ pkgs, lib, config, inputs, ... }:
{
  # https://devenv.sh/packages/
  packages = with pkgs; [
    git
    jujutsu
  ];

  languages = {
    rust = {
      enable = true;
      channel = "nightly";
      lsp.enable = true;
      # mold.enable = true;
    };

    c.enable = true;
    cplusplus.enable = true;
  };

  # https://devenv.sh/tests/
  enterTest = ''
    echo "Running tests"
    git --version | grep --color=auto "${pkgs.git.version}"
  '';
}
