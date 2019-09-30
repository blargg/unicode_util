with import <nixpkgs> {};
let
  commandName = "unicode-utils";
in
rustPlatform.buildRustPackage rec {
  name = "${commandName}-${version}";
  version = "0.1.0";

  src = ./.;

  # TODO generate commpletion scripts, and add them like the following
  # postInstall = ''
  #   mkdir -p $out/share/{bash-completion/completions,zsh/site-functions,fish/vendor_completions.d}
  #   $out/bin/${commandName} generate-completions --bash > $out/share/bash-completion/completions/${commandName}
  #   $out/bin/${commandName} generate-completions --zsh > $out/share/zsh/site-functions/_${commandName}
  #   $out/bin/${commandName} generate-completions --fish > $out/share/fish/vendor_completions.d/${commandName}.fish
  # '';

  cargoSha256 = "1pq8wdzcrsacqxmn85n8gmh7hl0vf2xr0dpawc790x9qs335fw1k";

  meta = with stdenv.lib; {
    description = "Search unicode characters, emoji, symbols, and more";
    license = with licenses; [ mit ];
    platforms = platforms.all;
  };
}
