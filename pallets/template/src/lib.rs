#![cfg_attr(not(feature = "std"), no_std)]


// Only manufacturing unit will all the hash of the product..
// Check if hash is found in the storage that means the product is authentic.
// Should have mechanism to add manufacturing unit
// Need to think..if a product is used then do we need to delete the product from the chain?



pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;


#[frame_support::pallet]
pub mod pallet {
	use frame_support::inherent::Vec;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn clubmember)]
	pub type ProductsHash<T: Config> = StorageValue<_, Vec<T::Hash>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {

	}

	// Error inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {

	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		// Add hash of the product..
		#[pallet::weight(10_000)]
		pub fn add_product(origin: OriginFor<T>, hash: T::AccountId) -> DispatchResult {
			Ok(())
		}
	}
}

