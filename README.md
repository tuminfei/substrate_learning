# Build Chain

```
git checkout v1.0.0

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

# Front-end Dev

### Install Locally

```bash
# Clone the repository
git clone https://github.com/substrate-developer-hub/substrate-front-end-template.git
cd substrate-front-end-template
yarn install
```

### Usage

You can start the template in development mode to connect to a locally running node

```bash
yarn start
```

You can also build the app in production mode,

```bash
yarn build
```

and open `build/index.html` in your favorite browser.