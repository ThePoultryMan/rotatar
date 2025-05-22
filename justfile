run:
  cargo run --no-default-features -F iced-frontend -- --frontend iced --config ./test/test_config.json

run-tauri:
  cd tauri-frontend && bun run dev & cargo run --no-default-features -F tauri-frontend -- --frontend tauri --config ./test/test_config.json

build:
  cargo build --features release
  cd target/debug && mv "rotatar.exe" "tauri-frontend.exe"
  cd tauri-frontend && bun run tauri build --debug

build-release:
  cargo build --release --features release
  cd target/release && mv "rotatar.exe" "tauri-frontend.exe"
  cd tauri-frontend && bun run tauri build
