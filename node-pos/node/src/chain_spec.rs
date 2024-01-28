use hex_literal::hex;

use node_primitives::*;
use node_template_runtime::{
	constants::currency::*, AccountId, BabeConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
	ImOnlineConfig, MaxNominations, SessionConfig, SessionKeys, Signature, StakerStatus,
	StakingConfig, SudoConfig, SystemConfig, BABE_GENESIS_EPOCH_CONFIG, WASM_BINARY,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};

// The URL for the telemetry server.
const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

fn session_keys(babe: BabeId, grandpa: GrandpaId, im_online: ImOnlineId) -> SessionKeys {
	SessionKeys { babe, grandpa, im_online }
}

/// Generate an Babe authority key.
pub fn authority_keys_from_seed(s: &str) -> (AccountId, AccountId, BabeId, GrandpaId, ImOnlineId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", s)),
		get_account_id_from_seed::<sr25519::Public>(s),
		get_from_seed::<BabeId>(s),
		get_from_seed::<GrandpaId>(s),
		get_from_seed::<ImOnlineId>(s),
	)
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				vec![],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
				vec![],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		None,
		// Extensions
		None,
	))
}

pub fn staging_network_config() -> ChainSpec {
	let boot_nodes = vec![];

	ChainSpec::from_genesis(
		"Substrate Terry",
		"terry_network",
		ChainType::Live,
		staging_network_config_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
		None,
		None,
		None,
		Default::default(),
	)
}

fn staging_network_config_genesis() -> GenesisConfig {
	let wasm_binary = WASM_BINARY.expect(
		"Development wasm binary is not available. This means the client is built with \
		 `SKIP_WASM_BUILD` flag and it is only usable for production chains. Please rebuild with \
		 the flag disabled.",
	);

	// ./target/release/substrate-stencil key generate --scheme Sr25519 --password-interactive
	// export SECRET='exit expect balance picnic struggle party garden resource despair fold addict
	// for i in 1 2 3 4; do for j in stash controller; do ./target/release/substrate-stencil key inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in babe; do ./target/release/substrate-stencil key inspect "$SECRET//$i//$j -- --sr25519"; done; done
	// for i in 1 2 3 4; do for j in grandpa; do ./target/release/substrate-stencil key inspect "$SECRET//$i//$j -- --ed25519"; done; done
	// for i in 1 2 3 4; do for j in im_online; do ./target/release/substrate-stencil key inspect "$SECRET//$i//$j -- --sr25519"; done; done
	let initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId)> = vec![
		(
			// 5Grpw9i5vNyF6pbbvw7vA8pC5Vo8GMUbG8zraLMmAn32kTNH
			hex!["d41e0bf1d76de368bdb91896b0d02d758950969ea795b1e7154343ee210de649"].into(),
			// 5Gsxw51KiwWByePT32chsPJfjzUjZ2kxNcpBKPxzwJgbs3EK
			hex!["d4fc37befb8b781c81d6c625f0feabca216e30a4dea31041d657b8fba531f24d"].into(),
			// 5CyArZHDXVE4iALz35AKU6uJHgr4AMDzcbRck9gwP8mgSAbL
			hex!["2803a508af686bc66899daa88f74da81aed9277316a24714aa8603cabe713e34"]
				.unchecked_into(),
			// 5HGZs8y7TAvQ2JrSmqCTEEdi9ptPtf2qZsKNf8drpTdwQGda
			hex!["e639357e903f283b806073cd8a61c1d7bd66e293cd5b7c700540ca846f66f17b"]
				.unchecked_into(),
			// 5GMt2Ys7gf41wLqvvXPc39TXUJrxtBej5atktXkasmevfJtq
			hex!["be0a579a955b4130dd09ba9ecce31857013f90e89f78a1430e62f05849f3e078"]
				.unchecked_into(),
		),
		(
			// 5H3nAs4E13wX2QWmdtksL1MEPfLHi4bkaMrpCpf6Karxc7b5
			hex!["dc78716eda98d7dcd2a076dc2028e67e7502fc123a526ddf6703ae174dbac835"].into(),
			// 5EjrvKc7KTEEXbnnJvSuCXq1jmjArJG4ya4RhtXbvBFDMUD7
			hex!["76558bdfff9d6102af16642aab4ebbbdc8ad4aafcd23a0a5080214d4b354920c"].into(),
			// 5G9EF6fPNLXC8fck4g9Wvt3KNwFfqdvbBNH75pgAAKgcERqa
			hex!["b46434cc5231eed048033802896d32101f56fd17ea2770273510ea0556fa4410"]
				.unchecked_into(),
			// 5FWRs7m4U2ubqPQmb6JyeuYD8tafeqw4ReoNWGFL3a9FH7FY
			hex!["98533f00f4ed9f43a07ba05f5f4ef04b95be253bfecc55e44331d6ea53f77603"]
				.unchecked_into(),
			// 5C8LYJS2t74FquY3t4pCcPnjnTq5jLdUsgP9qoNUzgfxAxa6
			hex!["02c53928cd60dfd27c3bc0e5bec3250cf570d1ebae578487bb636acc58b88266"]
				.unchecked_into(),
		),
		(
			// 5F4sjmu1A84fKUhHPrCvLnA8GWaCzHF93oLdekSpL8ZVf89i
			hex!["84d5f69712589177c6ef580d4513462ae0620fa88f71fe39f2ee890ddeea3645"].into(),
			// 5EnYqxeFUmSds5TuTcmwzDQdNtGVyNgM9W5LvNSqx5BSxSPD
			hex!["78626fa1c8afcb95a1aefec3717261ec23ef9a9935b8c9ea2bd8b9098b123c5d"].into(),
			// 5GvTVhw25uR1kJSdZzutMPoKGLg5zqvbz1V3Vneng8GzrZXH
			hex!["d6e2dbf3cbe089c34d3b1901cd9f839d88b77d7177b9326aec69e433d797bd7d"]
				.unchecked_into(),
			// 5Ef6RhANS1TwVM3FK2tXF19XgFhFaGZw7MmFMRNc1AHGT1N8
			hex!["72b2c9366f4979f178e9c7cac88aa7b9426412a1ec4e61dead3cde97ed414223"]
				.unchecked_into(),
			// 5ETtPqnR3Hcy5sZfXwLaDXGXqAvK3R6kCs8qMbcYC7XHU3Mf
			hex!["6a26955f570c0be74ab3c73b66c900c3defaf76a1aadf60a00dbfd3962f1f93c"]
				.unchecked_into(),
		),
		(
			// 5GgJichQYJqYNzmnrXkkzAj5gk1K4g3zeXUwQZrjiFJLvCMP
			hex!["cc17e0485f0aacbd5f693f3686e1c740214fe5389473766569c0ac62aab11934"].into(),
			// 5GisHk4ySDmxK28HQLNV8zfezdegTcnPaQYFuuoWvyDxzDPK
			hex!["ce0c02de113e7a80d64d459ab4b65a05692d759ea556e400a3b168843affe154"].into(),
			// 5GBkQbhb89PyAT1GGVfyCzZGrXFzDdhEugPTZmLp6aaHQBow
			hex!["b6503ccabad4551f2518f5c0334a66daa97154acfad6c4726befc19bcfe1a22f"]
				.unchecked_into(),
			// 5Fdyi1DNnGaHUU6M4kyCRX5nYydEKLsXqKsRECZrxyWmpwe5
			hex!["9e15281924218d5eb0d6393896cc4a7cc4dde2021ca62612e428d044fb40044c"]
				.unchecked_into(),
			// 5H1nPpGgQzQsybF3Y4Fsn5W8siCy88fayyR4R1ojWxYQwQia
			hex!["daf2b43a7bff8baf256c1c1fd86b29fff56fc56751e4d410a2a20d2234213a01"]
				.unchecked_into(),
		),
	];

	// generated with secret: ./target/release/substrate-stencil key inspect "$secret"/fir
	let root_key: AccountId = hex![
		// 5Ct1SLsTZB4bMeKCdchDWL2oN7hNTpjmrVqDsrWqspERFSDo
		"2413b72fb065b202a7b84c9a663c167c1435ec0814edf40a3c7ccbcf7a23ba33"
	]
	.into();

	let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

	testnet_genesis(wasm_binary, initial_authorities, vec![], root_key, endowed_accounts, true)
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId)>,
	initial_nominators: Vec<AccountId>,
	root_key: AccountId,
	mut endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	// endow all authorities and nominators.
	initial_authorities
		.iter()
		.map(|x| &x.0)
		.chain(initial_nominators.iter())
		.for_each(|x| {
			if !endowed_accounts.contains(x) {
				endowed_accounts.push(x.clone())
			}
		});

	// stakers: all validators and nominators.
	const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
	const STASH: Balance = ENDOWMENT / 1000;
	let mut rng = rand::thread_rng();
	let stakers = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
		.chain(initial_nominators.iter().map(|x| {
			use rand::{seq::SliceRandom, Rng};
			let limit = (MaxNominations::get() as usize).min(initial_authorities.len());
			let count = rng.gen::<usize>() % limit;
			let nominations = initial_authorities
				.as_slice()
				.choose_multiple(&mut rng, count)
				.into_iter()
				.map(|choice| choice.0.clone())
				.collect::<Vec<_>>();
			(x.clone(), x.clone(), STASH, StakerStatus::Nominator(nominations))
		}))
		.collect::<Vec<_>>();

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		babe: BabeConfig { authorities: vec![], epoch_config: Some(BABE_GENESIS_EPOCH_CONFIG) },
		grandpa: GrandpaConfig { authorities: vec![] },
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(x.0.clone(), x.0.clone(), session_keys(x.2.clone(), x.3.clone(), x.4.clone()))
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers,
			..Default::default()
		},
		im_online: ImOnlineConfig { keys: vec![] },
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
	}
}
