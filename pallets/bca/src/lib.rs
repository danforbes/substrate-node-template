#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use sp_std::prelude::*;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ClassData {
    pub name: Vec<u8>,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TokenData {
    pub name: Vec<u8>,
}

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_std::prelude::*;

    #[pallet::config]
    pub trait Config:
        frame_system::Config
        + orml_nft::Config<ClassData = crate::ClassData, TokenData = crate::TokenData>
    {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn something)]
    pub type Something<T> = StorageValue<_, u32>;

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId", <T as orml_nft::Config>::ClassId = "ClassId", <T as orml_nft::Config>::TokenId = "TokenId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        CollectionCreated(<T as orml_nft::Config>::ClassId, T::AccountId),
        TokenMinted(
            <T as orml_nft::Config>::ClassId,
            <T as orml_nft::Config>::TokenId,
        ),
    }

    #[pallet::error]
    pub enum Error<T> {
        CollectionNotFound,
        NotCollectionOwner,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(T::BlockWeights::get().max_block / 100)]
        pub fn create_collection(
            origin: OriginFor<T>,
            metadata: Vec<u8>,
            class_data: T::ClassData,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let collection_id = orml_nft::Pallet::<T>::create_class(&who, metadata, class_data)?;
            Self::deposit_event(Event::CollectionCreated(collection_id, who));
            Ok(().into())
        }

        #[pallet::weight(T::BlockWeights::get().max_block / 100)]
        pub fn mint(
            origin: OriginFor<T>,
            collection_id: T::ClassId,
            metadata: Vec<u8>,
            token_data: T::TokenData,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let collection = orml_nft::Pallet::<T>::classes(collection_id)
                .ok_or(Error::<T>::CollectionNotFound)?;
            ensure!(collection.owner == who, Error::<T>::NotCollectionOwner);
            let token_id = orml_nft::Pallet::<T>::mint(&who, collection_id, metadata, token_data)?;
            Self::deposit_event(Event::TokenMinted(collection_id, token_id));
            Ok(().into())
        }
    }
}
