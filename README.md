# Build Chain

```
git checkout v0.9.40

./scripts/init.sh

cargo build --release
```

## Rust Version

```
rustup install nightly-2022-11-15

rustup default nightly-2022-11-15

rustup target add wasm32-unknown-unknown
```

# Run Dev Chain

```
polkadot --dev
```

## Local Two-node Testnet

```
polkadot --chain=polkadot-local --alice -d /tmp/alice
```