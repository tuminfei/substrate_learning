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

## Homework

### Lesson 1 - POE

```bash
cd homework/pallets/poe

cargo build --release
```

### Lesson 2 - Kitties

```bash
cd homework/pallets/kitties

cargo build --release
```

### Lesson 3 - Kitties

```bash
cd homework/pallets/kitties

cargo build --release
```

add balance for test

```rust
// set balance
assert_ok!(Balances::force_set_balance(RuntimeOrigin::root(), account_id, 100000000000));
```

### Lesson 4 - Offchain

```bash
cd homework/pallets/offchain

cargo build --release
```

frontend

```bash
cd homework_frontend/offchain

npm i

ts-node index.ts
```


```rust
#[pallet::call_index(3)]
#[pallet::weight(100)]
pub fn extrinsic(origin: OriginFor<T>, number: u64) -> DispatchResult {
	let who = ensure_signed(origin)?;

	let key = Self::derived_key(frame_system::Module::<T>::block_number());
	let data = IndexingData(b"submit_number_unsigned".to_vec(), number);
	offchain_index::set(&key, &data.encode());

	log::info!("OCW ==> in extrinsic submit_number_unsigned: {:?}", number);
	Ok(())
}
```

### Lesson 5 - Smart Contract

```bash
cd ink-playground/erc20

cargo contract build
```

### Lesson 6 - Testing & Deploy Online

```bash
cd cd homework

cargo +nightly build --release --features runtime-benchmarks

./target/release/node-template benchmark pallet \
	--chain dev \
	--execution wasm  \
	--wasm-execution compiled \
	--pallet pallet_poe   --extrinsic "*" \
	--steps 20 --repeat 10 \
	--output ./pallets/poe/src/weights.rs \
	--template .maintain/frame-weight-template.hbs

```