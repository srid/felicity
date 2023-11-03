default:
    @just --list

# Auto-format the source tree
fmt:
    dx fmt -f src/main.rs
    treefmt

# Run the project locally
watch $RUST_BACKTRACE="1":
    dx serve

# CI=true for https://github.com/tauri-apps/tauri/issues/3055#issuecomment-1624389208)
bundle $CI="true":
    # HACK (change PWD): Until https://github.com/DioxusLabs/dioxus/issues/1283
    cd assets && dx bundle --release
    nix run nixpkgs#eza -- -T ./dist/bundle/macos/felicity.app
    rm -rf /Applications/felicity.app/
    mv dist/bundle/macos/felicity.app /Applications/
