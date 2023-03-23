#![cfg_attr(not(feature = "std"), no_std)]


// Only manufacturing unit will allow to add all the hash of the product..
// Check if hash is found in the storage that means the product is authentic.
// Should have mechanism to add manufacturing unit
// Need to think..if a product is used then do we need to delete the product from the chain?
// Anybody can see the barcode on the product and try to copy that barcode and put on the fake product?
// how to generate a unique hash?
// What are the details required to add the manufacturer?

//! We Need to store the products in a different storage(partially_selled_products) which is scanned first..so that no one can copy the product QR.
//! If a user wants to return the Product and we have 15 days for it.until then this product is stored in partially_selled_products.
//! After the return time is over the product goes to sell_product.
//! If product is returned under the given time slot.

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
	pub trait Config: frame_system::Config{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn productshash)]
	pub type ProductsHash<T: Config> = StorageValue<_, Vec<T::Hash>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn manufacturers)]
	pub type Manufacturer<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ProductAdded,
		ManufacturerAdded,
		AuthenticProduct,
	}

	// Error inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		// If random person try to add the hash of the product.
		UnAuthorisedPerson,
		// Product already present
		ProductAlreadyPresent,
		// If a manufacturer is already present
		ManufacturerAlreadyPresent,
		// If product is not present (i.e Unauthentic product)
		UnAuthenticProduct,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		// Add hash of the product..
		// Only authorised person is allowed to perform this task.
		#[pallet::weight(10_000)]
		pub fn add_product(origin: OriginFor<T>, hash: T::Hash) -> DispatchResult {
			let who = ensure_signed(origin.clone())?;

			let all_manufacturer = Manufacturer::<T>::get();
			let _location = all_manufacturer.binary_search(&who).ok().ok_or(Error::<T>::UnAuthorisedPerson)?;

			let mut all_products = ProductsHash::<T>::get();
			let location = all_products.binary_search(&hash).err().ok_or(Error::<T>::ProductAlreadyPresent)?;

			all_products.insert(location, hash);

			ProductsHash::<T>::put(all_products);

			Self::deposit_event(Event::ProductAdded);
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn add_manufacturer(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;

			let mut all_manufacturer = Manufacturer::<T>::get();
			let location = all_manufacturer.binary_search(&who).err().ok_or(Error::<T>::ManufacturerAlreadyPresent)?;

			all_manufacturer.insert(location, who);
			Manufacturer::<T>::put(all_manufacturer);

			Self::deposit_event(Event::<T>::ManufacturerAdded);
			Ok(())
		}

		#[pallet::weight(10_100)]
		pub fn check_authenticity(origin: OriginFor<T>, hash: T::Hash) -> DispatchResult {
			ensure_signed(origin)?;

			let all_products = ProductsHash::<T>::get();
			all_products.binary_search(&hash).ok().ok_or(Error::<T>::UnAuthenticProduct)?;

			Self::deposit_event(Event::<T>::AuthenticProduct);

			Ok(())
		}
	}
}

