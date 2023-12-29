# Build Chain

``
git checkout v0.9.40
./scripts/init.sh
cargo build --release
``

# Run Dev Chain

``
polkadot --dev
``

## Local Two-node Testnet

``
polkadot --chain=polkadot-local --alice -d /tmp/alice
``