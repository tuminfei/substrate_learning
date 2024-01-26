#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use codec::{Decode, Encode};
use frame_system::offchain::{
	AppCrypto, CreateSignedTransaction, SendUnsignedTransaction, SignedPayload, Signer,
	SigningTypes,
};
use sp_runtime::{
	offchain::{http, Duration},
	transaction_validity::{InvalidTransaction, TransactionValidity, ValidTransaction},
	BoundedVec, RuntimeDebug,
};

use core::primitive::str;
use frame_support::dispatch::Vec;
use scale_info::TypeInfo;
use serde::{Deserialize, Deserializer};
use sp_core::{crypto::KeyTypeId, ConstU32};
use sp_io::offchain_index;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"btc!");

pub const ONCHAIN_TX_KEY: &[u8] = b"my_pallet::indexing1";

pub const ONCHAIN_WEATHER_TX_KEY: &[u8] = b"weather::indexing1";

pub mod crypto {
	use super::KEY_TYPE;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		traits::Verify,
		MultiSignature, MultiSigner,
	};
	app_crypto!(sr25519, KEY_TYPE);

	pub struct TestAuthId;

	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}

	// implemented for mock runtime in test
	impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
		for TestAuthId
	{
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[derive(Debug, Encode, Decode, Default)]
	struct IndexingData(Vec<u8>, u64);

	#[derive(Debug, Encode, Decode, Default)]
	struct WeatherIndexingData(BoundedVec<u8, ConstU32<9>>);

	#[derive(Deserialize, Encode, Decode, Clone, PartialEq, Eq, TypeInfo, Debug)]
	pub struct WeatherInfo {
		#[serde(deserialize_with = "de_string_to_bounded_bytes")]
		pub shiku: BoundedVec<u8, ConstU32<32>>,
		#[serde(deserialize_with = "de_string_to_bounded_bytes")]
		pub pm25: BoundedVec<u8, ConstU32<32>>,
		#[serde(deserialize_with = "de_string_to_bounded_bytes")]
		pub wendu: BoundedVec<u8, ConstU32<32>>,
	}

	pub fn de_string_to_bounded_bytes<'de, D>(
		de: D,
	) -> Result<BoundedVec<u8, ConstU32<32>>, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s: &str = Deserialize::deserialize(de)?;
		Ok(BoundedVec::<u8, ConstU32<32>>::try_from(s.as_bytes().to_vec())
			.map_err(|_| serde::de::Error::custom("BoundedVec error"))?)
	}

	pub fn de_vec_to_bounded_vec<'de, D>(
		de: D,
	) -> Result<BoundedVec<WeatherInfo, ConstU32<10>>, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s: Vec<WeatherInfo> = Deserialize::deserialize(de)?;
		let a = s
			.into_iter()
			.take(10)
			.collect::<Vec<WeatherInfo>>()
			.try_into()
			.map_err(|_| serde::de::Error::custom("BoundedVec error"))?;

		Ok(a)
	}

	#[derive(Deserialize, Debug, Encode, Decode)]
	pub struct ApiResponse {
		pub status: i32,
		#[serde(deserialize_with = "de_vec_to_bounded_vec")]
		pub data: BoundedVec<WeatherInfo, ConstU32<10>>,
	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
	pub struct TxPayload<Public> {
		weather_data: BoundedVec<WeatherInfo, ConstU32<10>>,
		public: Public,
	}

	impl<T: SigningTypes> SignedPayload<T> for TxPayload<T::Public> {
		fn public(&self) -> T::Public {
			self.public.clone()
		}
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	// pub trait Config: frame_system::Config +
	// frame_system::offchain::SendTransactionTypes<Call<Self>> {
	pub trait Config: frame_system::Config + CreateSignedTransaction<Call<Self>> {
		/// The identifier type for an offchain worker.
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn next_offchain_id)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type NextOffchainId<T> = StorageValue<_, u128, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored {
			something: u32,
			who: T::AccountId,
		},
		CityCodeStored {
			city_code: BoundedVec<u8, ConstU32<9>>,
			who: T::AccountId,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored { something, who });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn unsigned_extrinsic_with_signed_payload(
			origin: OriginFor<T>,
			payload: TxPayload<T::Public>,
			_signature: T::Signature,
		) -> DispatchResult {
			ensure_none(origin)?;

			log::info!(
				"OCW ==> in call unsigned_extrinsic_with_signed_payload: {:?}",
				payload.weather_data
			);
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(100)]
		pub fn extrinsic(origin: OriginFor<T>, number: u64) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let key = Self::derived_key(frame_system::Module::<T>::block_number());
			let data = IndexingData(b"submit_number_unsigned".to_vec(), number);
			offchain_index::set(&key, &data.encode());

			log::info!("OCW ==> {:?} in extrinsic submit_number_unsigned: {:?}", who, number);
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(0)]
		pub fn set_offchain_city(
			origin: OriginFor<T>,
			city_code: BoundedVec<u8, ConstU32<9>>,
		) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			log::info!("EXTRINSIC ==> set_offchain_city: {:?}", city_code);
			let data = WeatherIndexingData(city_code.clone());

			log::info!("EXTRINSIC ==> set key: {:?}", ONCHAIN_WEATHER_TX_KEY);
			log::info!(
				"EXTRINSIC ==> set value: {:?}",
				sp_std::str::from_utf8(&city_code).unwrap()
			);
			sp_io::offchain_index::set(&ONCHAIN_WEATHER_TX_KEY, &data.encode());

			Self::deposit_event(Event::CityCodeStored { city_code, who: _who });
			Ok(())
		}
	}

	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		/// Validate unsigned call to this module.
		///
		/// By default unsigned transactions are disallowed, but implementing the validator
		/// here we make sure that some particular calls (the ones produced by offchain worker)
		/// are being whitelisted and marked as valid.
		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			const UNSIGNED_TXS_PRIORITY: u64 = 100;
			let valid_tx = |provide| {
				ValidTransaction::with_tag_prefix("my-pallet")
					.priority(UNSIGNED_TXS_PRIORITY) // please define `UNSIGNED_TXS_PRIORITY` before this line
					.and_provides([&provide])
					.longevity(3)
					.propagate(true)
					.build()
			};

			// match call {
			// 	Call::submit_data_unsigned { key: _ } => valid_tx(b"my_unsigned_tx".to_vec()),
			// 	_ => InvalidTransaction::Call.into(),
			// }

			match call {
				Call::unsigned_extrinsic_with_signed_payload { ref payload, ref signature } => {
					if !SignedPayload::<T>::verify::<T::AuthorityId>(payload, signature.clone()) {
						return InvalidTransaction::BadProof.into();
					}
					valid_tx(b"unsigned_extrinsic_with_signed_payload".to_vec())
				},
				_ => InvalidTransaction::Call.into(),
			}
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		/// Offchain worker entry point.
		fn offchain_worker(block_number: BlockNumberFor<T>) {
			// let value: u64 = 42;
			// // This is your call to on-chain extrinsic together with any necessary parameters.
			// let call = Call::submit_data_unsigned { key: value };

			// // `submit_unsigned_transaction` returns a type of `Result<(), ()>`
			// //	 ref: https://paritytech.github.io/substrate/master/frame_system/offchain/struct.SubmitTransaction.html
			// _ = SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into())
			// 	.map_err(|_| {
			// 	log::error!("OCW ==> Failed in offchain_unsigned_tx");
			// });

			// if block_number % 2u32.into() != Zero::zero() {
			// 	// odd
			// 	let key = Self::derive_key(block_number);
			// 	let val_ref = StorageValueRef::persistent(&key);

			// 	//  get a local random value
			// 	let random_slice = sp_io::offchain::random_seed();

			// 	//  get a local timestamp
			// 	let timestamp_u64 = sp_io::offchain::timestamp().unix_millis();

			// 	// combine to a tuple and print it
			// 	let value = (random_slice, timestamp_u64);
			// 	log::info!("OCW ==> in odd block, value to write: {:?}", value);

			// 	struct StateError;

			// 	//  write or mutate tuple content to key
			// 	let res = val_ref.mutate(|val: Result<Option<([u8;32], u64)>, StorageRetrievalError>|
			// -> Result<_, StateError> {         match val {
			//             Ok(Some(_)) => Ok(value),
			//             _ => Ok(value),
			//         }
			//     });

			// 	match res {
			// 		Ok(value) => {
			// 			log::info!("OCW ==> in odd block, mutate successfully: {:?}", value);
			// 		},
			// 		Err(MutateStorageError::ValueFunctionFailed(_)) => (),
			// 		Err(MutateStorageError::ConcurrentModification(_)) => (),
			// 	}
			// } else {
			// 	// even
			// 	let key = Self::derive_key(block_number - 1u32.into());
			// 	let mut val_ref = StorageValueRef::persistent(&key);

			// 	// get from db by key
			// 	if let Ok(Some(value)) = val_ref.get::<([u8; 32], u64)>() {
			// 		// print values
			// 		log::info!("OCW ==> in even block, value read: {:?}", value);
			// 		// delete that key
			// 		val_ref.clear();
			// 	}
			// }

			// let number: u64 = 42;
			// // Retrieve the signer to sign the payload
			// let signer = Signer::<T, T::AuthorityId>::any_account();

			// // `send_unsigned_transaction` is returning a type of `Option<(Account<T>, Result<(),
			// // ()>)>`. 	 The returned result means:
			// // 	 - `None`: no account is available for sending transaction
			// // 	 - `Some((account, Ok(())))`: transaction is successfully sent
			// // 	 - `Some((account, Err(())))`: error occurred when sending the transaction
			// if let Some((_, res)) = signer.send_unsigned_transaction(
			// 	// this line is to prepare and return payload
			// 	|acct| Payload { number, public: acct.public.clone() },
			// 	|payload, signature| Call::unsigned_extrinsic_with_signed_payload {
			// 		payload,
			// 		signature,
			// 	},
			// ) {
			// 	match res {
			// 		Ok(()) => {
			// 			log::info!("OCW ==> unsigned tx with signed payload successfully sent.");
			// 		},
			// 		Err(()) => {
			// 			log::error!("OCW ==> sending unsigned tx with signed payload failed.");
			// 		},
			// 	};
			// } else {
			// 	// The case of `None`: no account is available for sending
			// 	log::error!("OCW ==> No local account available");
			// }

			// let key = Self::derived_key(block_number);
			// let storage_ref = StorageValueRef::persistent(&key);

			// if let Ok(Some(data)) = storage_ref.get::<IndexingData>() {
			// 	log::info!("local storage data: {:?}, {:?}", &data.0, data.1);
			// } else {
			// 	log::info!("Error reading from local storage.");
			// }

			let city_code = Self::get_city_code_from_storage();
			if let Ok(info) = Self::fetch_weather_info(city_code) {
				log::info!("OCW ==> Weather Info: {:?}", info);

				// Retrieve the signer to sign the payload
				let signer = Signer::<T, T::AuthorityId>::any_account();

				if let Some((_, res)) = signer.send_unsigned_transaction(
					// this line is to prepare and return payload
					|acct| TxPayload { weather_data: info.clone(), public: acct.public.clone() },
					|payload, signature| Call::unsigned_extrinsic_with_signed_payload {
						payload,
						signature,
					},
				) {
					match res {
						Ok(()) => {
							log::info!(
								"OCW ==> unsigned tx with signed payload successfully sent."
							);
						},
						Err(()) => {
							log::error!("OCW ==> sending unsigned tx with signed payload failed.");
						},
					};
				} else {
					log::error!("OCW ==> No local account available");
				}
			} else {
				log::info!("OCW ==> Error while fetch weather info!");
			}

			log::info!("OCW ==> Leave from offchain workers!: {:?}", block_number);
		}
	}

	impl<T: Config> Pallet<T> {
		// #[deny(clippy::clone_double_ref)]
		// fn derive_key(block_number: BlockNumberFor<T>) -> Vec<u8> {
		// 	block_number.using_encoded(|encoded_bn| {
		// 		b"node-template::storage::"
		// 			.iter()
		// 			.chain(encoded_bn)
		// 			.copied()
		// 			.collect::<Vec<u8>>()
		// 	})
		// }
		fn derived_key(block_number: BlockNumberFor<T>) -> Vec<u8> {
			block_number.using_encoded(|encoded_bn| {
				ONCHAIN_TX_KEY
					.into_iter()
					.chain(b"/".into_iter())
					.chain(encoded_bn)
					.copied()
					.collect::<Vec<u8>>()
			})
		}

		fn fetch_weather_info(
			city_code: BoundedVec<u8, ConstU32<9>>,
		) -> Result<BoundedVec<WeatherInfo, ConstU32<10>>, http::Error> {
			// prepare for send request
			let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(8_000));
			let url = Self::get_url(city_code);
			let url = sp_std::str::from_utf8(&url).map_err(|_| {
				log::warn!("No UTF8 body");
				http::Error::Unknown
			})?;
			let request = http::Request::get(url);
			let pending = request
				.add_header("User-Agent", "Substrate-Offchain-Worker")
				.deadline(deadline)
				.send()
				.map_err(|_| http::Error::IoError)?;
			let response =
				pending.try_wait(deadline).map_err(|_| http::Error::DeadlineReached)??;
			if response.code != 200 {
				log::warn!("Unexpected status code: {}", response.code);
				return Err(http::Error::Unknown)
			}
			let body = response.body().collect::<Vec<u8>>();
			let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
				log::warn!("No UTF8 body");
				http::Error::Unknown
			})?;

			// parse the response str
			let weather_response: ApiResponse =
				serde_json::from_str(body_str).map_err(|_| http::Error::Unknown)?;

			Ok(weather_response.data)
		}

		fn get_url(city_code: BoundedVec<u8, ConstU32<9>>) -> Vec<u8> {
			let mut result = Vec::from("http://t.weather.sojson.com/api/weather/city/".as_bytes());
			result.extend_from_slice(city_code.as_slice());
			result
		}

		fn get_city_code_from_storage() -> BoundedVec<u8, ConstU32<9>> {
			let mut result = BoundedVec::<u8, ConstU32<9>>::try_from(b"1".to_vec()).unwrap();
			if let Some(city_code) =
				sp_runtime::offchain::storage::StorageValueRef::persistent(ONCHAIN_WEATHER_TX_KEY)
					.get::<WeatherIndexingData>()
					.unwrap_or_else(|_| {
						log::info!("OCW ==> Error while fetching data from offchain storage!");
						None
					}) {
				result = city_code.0;
			}
			result
		}
	}
}
