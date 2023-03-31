#![cfg_attr(not(feature = "std"), no_std)]

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

	#[pallet::storage]
	#[pallet::getter(fn partialsellproduct)]
	pub type PartialSellProduct<T: Config> = StorageValue<_, Vec<T::Hash>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn selledproducts)]
	pub type SellProducts<T: Config> = StorageValue<_, Vec<T::Hash>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn retunedproducts)]
	pub type ReturnedProducts<T: Config> = StorageValue<_,Vec<T::Hash>, ValueQuery>;

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
		// If product was not sold.
		UnsoldProduct,
	}

	// Hooks
	// after 15 days product from pending selling should converted to selled.

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

		#[pallet::weight(10_000)]
		pub fn check_authenticity(origin: OriginFor<T>, hash: T::Hash) -> DispatchResult {
			ensure_signed(origin)?;

			// Check the product in fresh added products.
			let mut all_products = ProductsHash::<T>::get();
			let location = all_products.binary_search(&hash).ok().ok_or(Error::<T>::UnAuthenticProduct)?;


			// Now the product is sell for first time.
			// Add in the partial sell product..because it might me return in future.
			let mut partial_sell = PartialSellProduct::<T>::get();
			partial_sell.push(hash);
			PartialSellProduct::<T>::put(partial_sell);

			// Remove this item from freshly added products.
			all_products.remove(location);
			ProductsHash::<T>::put(all_products);

			Self::deposit_event(Event::<T>::AuthenticProduct);

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn refund_products(who: OriginFor<T>, hash: T::Hash) -> DispatchResult {
			ensure_signed(who.clone())?;

			let mut products = PartialSellProduct::<T>::get();
			// check this product is sold or not?
			let location = products.binary_search(&hash).err().ok_or(Error::<T>::UnsoldProduct)?;

			// Now remove the product from partial sale.
			products.remove(location);
			PartialSellProduct::<T>::put(products);

			// Add the product into returned product storage for originality check in manufacturing unit and then again this product is available for sale.
			let mut items = ReturnedProducts::<T>::get();
			items.push(hash);
			ReturnedProducts::<T>::put(items);

			Ok(())
		}



	}
}

