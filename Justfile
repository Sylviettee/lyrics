build:
  mkdir -p out/linux/{arm64,amd64}

  cross build --release --target aarch64-unknown-linux-musl
  cp target/aarch64-unknown-linux-musl/release/lyrics out/linux/arm64

  cargo build --release --target x86_64-unknown-linux-musl
  cp target/x86_64-unknown-linux-musl/release/lyrics out/linux/amd64

dev: build
  #!/usr/bin/env bash
  if ! podman manifest inspect localhost/lyrics; then
    podman manifest create lyrics
  fi

  podman build --platform linux/amd64,linux/arm64 --manifest lyrics .

release: dev
  podman manifest push localhost/lyrics:latest docker://codeberg.org/sylviettee/lyrics:latest
