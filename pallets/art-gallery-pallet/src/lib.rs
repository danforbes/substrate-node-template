//! # Art Gallery
//! The module provides implementations for art gallery with
//! non-fungible-tokens.
//!
//! - [`Config`](./trait.Config.html)
//! - [`Call`](./enum.Call.html)
//! - [`Module`](./struct.Module.html)
//!
//! ## Overview
//!
//! This module tightly coupled with NFT module provides basic functions to
//! manage Art Gallery.
//!
//! ### Module Functions
//!
//! - `mint` - Mint NFT(non fungible token)
//! - `burn` - Burn NFT(non fungible token)
//! - `transfer` - Change owner for NFT(non fungible token) with tree hierarchy
//! limitation
//! - `assign` - Add NFT(non fungible token) to gallery hierarchy
//! - `unassign` - Remove NFT(non fungible token) from gallery hierarchy
//! - `mint_and_assign` - Mint NFT(non fungible token) and add to gallery
//! hierarchy

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod benchmarking;

use codec::{Decode, Encode};
use frame_support::{Parameter, decl_error, decl_event, decl_module, decl_storage, ensure, fail, traits::ReservableCurrency};
use frame_support::traits::{
	Currency, LockableCurrency, LockIdentifier, WithdrawReasons,
	Get, ExistenceRequirement, BalanceStatus
};
use pallet_atomic_swap::SwapAction;
use sp_runtime::traits::Saturating;
use frame_system::{ ensure_signed, ensure_root };
use orml_nft::{self as nft};
// use pallet_atomic_swap::{self as atomic_swap};
use sp_runtime::{ 	
	RuntimeDebug,
	traits::{AtLeast32BitUnsigned, Member, Zero},
	DispatchResult, };
use sp_std::prelude::*;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

//const PALLET_ID: LockIdentifier = *b"gallery ";

type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[derive(Encode, Decode, Debug, Clone, Eq, PartialEq)]
pub enum ReportReason {
	None,
	Illegal,
	Plagiarism,
	Duplicate,
	Reported
}

#[derive(Encode, Decode, Clone, Debug, PartialEq)]
pub struct ExtendedInfo {
    pub display_flag: bool,
    pub report: ReportReason,
	pub frozen: bool
}

decl_error! {
	/// Error for art gallery
	pub enum Error for Module<T: Config> {
		/// Collection doesn't exist
		CollectionNotFound,
		/// Token doesn't exist
		TokenNotFound,
		/// Offer doesn't exist
		OfferNotFound,
		/// Sender should equal token owner
		MustBeTokenOwner,
		/// Sender should be collection owner
		MustBeCollectionOwner,
		/// Sender should be collection owner or curator
		MustBeCollectionOwnerOrCurator,
		/// Sender should be curator
		MustBeCurator,
		/// Specified amount is above sender balance
		BalanceNotEnough,
		/// Token Frozen for Swap
		TokenFrozen
	}
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ClassData {

}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TokenData {

} 
pub trait Config: frame_system::Config + nft::Config<ClassData = ClassData, TokenData = TokenData> + pallet_atomic_swap::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	/// The currency trait.
	type Currency: ReservableCurrency<Self::AccountId>;

}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Config>::AccountId,
        ClassId = <T as nft::Config>::ClassId,
		TokenId = <T as nft::Config>::TokenId,
		Balance = BalanceOf<T>
    {
        /// New collection was created
        /// 
        /// # Arguments
        /// 
		/// ClassId: Globally unique identifier of newly created collection.
        CollectionCreated(ClassId),

        /// New item was created.
        /// 
        /// # Arguments
        /// 
		/// ClassId: Id of the collection where item was created.
		/// 
		/// TokenId: Id of an item. Unique within the collection.
        NFTCreated(ClassId, TokenId),

        /// Collection item was burned.
        /// 
        /// # Arguments
        /// 
        /// ClassId: Collection Id
		/// 
		/// TokenId: Identifier of NFT.
		NFTBurned(ClassId, TokenId),
		
		/// Transfer has been ended.
        /// 
        /// # Arguments
        /// 
        /// ClassId: Collection Id
		/// 
		/// TokenId: Identifier of NFT.
        /// 
		/// AccountId: Recipient.
		Transfer(ClassId, TokenId, AccountId),
		
		/// Offer has been created.
        /// 
        /// # Arguments
        /// 
        /// ClassId: Collection Id
		/// 
		/// TokenId: Identifier of NFT.
        /// 
		/// Balance: Price of NFT.
		/// 
		/// AccountId: Buyer Address
		OfferCreated(ClassId, TokenId, Balance, AccountId),
		
		/// Offer has been accepted.
        /// 
        /// # Arguments
        /// 
        /// ClassId: Collection Id
		/// 
		/// TokenId: Identifier of NFT.
        /// 
		/// AccountId: Seller Address
		/// 
		/// AccountId: Buyer Address
		AcceptOffer(ClassId, TokenId, AccountId, AccountId),
		
		/// Offer canceled.
        /// 
        /// # Arguments
        /// 
		/// ClassId: Collection Id
		/// 
		/// TokenId: Identifier of NFT.
        /// 
		/// AccountId: Seller Address
		/// 
		/// AccountId: Buyer Address
		CancelOffer(ClassId, TokenId, AccountId, AccountId),
		
		/// Appreciation has been sent.
        /// 
        /// # Arguments
        /// 
		/// ClassId: Collection Id
		/// 
		/// TokenId: Identifier of NFT.
        /// 
		/// Balance: Amount of appreciation.
		/// 
		AppreciationReceived(ClassId, TokenId, Balance),
		
		/// Display flag has been toggled.
        /// 
        /// # Arguments
        /// 
		/// bool: Display flag
		ToggleDisplay(bool),
		
		/// Report state has been set.
        /// 
        /// # Arguments
        /// 
		/// ClassId: Collection Id
		/// 
		/// TokenId: Identifier of NFT.
		/// 
		/// ReportReason: Reason of report
		ArtReported(ClassId, TokenId, ReportReason),
		
		/// Report has been accepted.
        /// 
        /// # Arguments
        /// 
		/// ClassId: Collection Id
		/// 
		/// TokenId: Identifier of NFT.
		ArtReportAccepted(ClassId, TokenId),
		
		/// Report has been cleared.
        /// 
        /// # Arguments
        /// 
		/// ClassId: Collection Id
		/// 
		/// TokenId: Identifier of NFT.
        ArtReportCleared(ClassId, TokenId),
    }
);

decl_storage! {
	trait Store for Module<T: Config> as ArtGallery {
		/// Curator address
		pub Curator get(fn curator): T::AccountId;

		/// Returns `None` if info not set or removed.
		/// Should really be refactored to store this info in T::TokenData
		pub TokenExtendedInfo get(fn token_extended_info): double_map hasher(twox_64_concat) T::ClassId, hasher(twox_64_concat) T::TokenId => Option<ExtendedInfo>;

		/// Returns `None` if a given account has no offer on a given Token.
		pub Offers get(fn offer): double_map hasher(twox_64_concat) (T::ClassId, T::TokenId), hasher(twox_64_concat) T::AccountId => Option<BalanceOf<T>>;
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		#[weight = T::BlockWeights::get().max_block / 100]
		pub fn create_collection(origin, metadata: Vec<u8>, class_data: T::ClassData) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let collection_id = nft::Module::<T>::create_class(&who, metadata, class_data)?;

			Self::deposit_event(RawEvent::CollectionCreated(collection_id));

			Ok(())
		}

		#[weight = T::BlockWeights::get().max_block / 100]
		pub fn mint(origin,
				collection_id: T::ClassId,
				metadata: Vec<u8>,
				token_data: T::TokenData
			) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// collection exists check
			let collection = nft::Module::<T>::classes(collection_id).ok_or(Error::<T>::CollectionNotFound)?;

			ensure!(collection.owner == who, Error::<T>::MustBeCollectionOwner);


			//T::Currency::set_lock(PALLET_ID, &who, T::DefaultCost::get(), WithdrawReasons::all());
			// agree there needs to be some cost but I'm not certain it should be via lock since tokens
			// are transferrable
			let token_id = nft::Module::<T>::mint(&who, collection_id, metadata, token_data)?;

			Self::deposit_event(RawEvent::NFTCreated(collection_id, token_id));

			Ok(())
		}

		#[weight = T::BlockWeights::get().max_block / 100]
		pub fn burn(origin,
				collection_id: T::ClassId,
				token_id: T::TokenId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// collection exists check
			let collection = nft::Module::<T>::classes(collection_id).ok_or(Error::<T>::CollectionNotFound)?;

			ensure!(Curator::<T>::get() == who || collection.owner == who, 
				Error::<T>::MustBeCollectionOwnerOrCurator);
			
			let info = TokenExtendedInfo::<T>::get(collection_id, token_id).unwrap_or_else(|| ExtendedInfo {
				display_flag: false,
				report: ReportReason::None,
				frozen: false
			});
			ensure!(info.frozen == false, Error::<T>::TokenFrozen);
			// doesn't make sense - the burn could be by a different person than the lock.
			//T::Currency::remove_lock(PALLET_ID, &who);
			nft::Module::<T>::burn(&who, (collection_id, token_id))?;	

			Self::deposit_event(RawEvent::NFTBurned(collection_id, token_id));

			Ok(())
		}

		#[weight = T::BlockWeights::get().max_block / 100]
		pub fn transfer(origin,
				collection_id: T::ClassId,
				token_id: T::TokenId,
				recipient: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// token exists check
			let token = nft::Module::<T>::tokens(collection_id, token_id).ok_or(Error::<T>::TokenNotFound)?;
			ensure!(token.owner == who, Error::<T>::MustBeTokenOwner);
			let info = TokenExtendedInfo::<T>::get(collection_id, token_id).unwrap_or_else(|| ExtendedInfo {
				display_flag: false,
				report: ReportReason::None,
				frozen: false
			});
			ensure!(info.frozen == false, Error::<T>::TokenFrozen);

			nft::Module::<T>::transfer(&who, &recipient, (collection_id, token_id))?;	
			Self::deposit_event(RawEvent::Transfer(collection_id, token_id, recipient));

			Ok(())
		}

		#[weight = T::BlockWeights::get().max_block / 100]
		pub fn create_offer(origin,
			collection_id: T::ClassId,
			token_id: T::TokenId,
			price: BalanceOf<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			T::Currency::reserve(&who, price)?; 
			Offers::<T>::insert((collection_id, token_id), who.clone(), price);
			Self::deposit_event(RawEvent::OfferCreated(collection_id, token_id, price, who));
			Ok(())
		}

		#[weight = T::BlockWeights::get().max_block / 100]
		pub fn accept_offer(origin,
			collection_id: T::ClassId,
			token_id: T::TokenId,
			buyer_address: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let token = nft::Module::<T>::tokens(collection_id, token_id).ok_or(Error::<T>::TokenNotFound)?;
			ensure!(token.owner == who, Error::<T>::MustBeTokenOwner);
			let info = TokenExtendedInfo::<T>::get(collection_id, token_id).unwrap_or_else(|| ExtendedInfo {
				display_flag: false,
				report: ReportReason::None,
				frozen: false
			});
			ensure!(info.frozen == false, Error::<T>::TokenFrozen);
			if let Some(offer) = Offers::<T>::get((collection_id, token_id), buyer_address.clone()){
				T::Currency::repatriate_reserved(&buyer_address, &who, offer, BalanceStatus::Free)?;
				Offers::<T>::remove((collection_id, token_id), who.clone());
				nft::Module::<T>::transfer(&who, &buyer_address, (collection_id, token_id))?;
				Self::deposit_event(RawEvent::AcceptOffer(collection_id, token_id, who, buyer_address));
			} else {
				fail!(Error::<T>::OfferNotFound);
			}
			Ok(())	
		}

		#[weight = T::BlockWeights::get().max_block / 100]
		pub fn cancel_offer(origin,
			collection_id: T::ClassId,
			token_id: T::TokenId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let token = nft::Module::<T>::tokens(collection_id, token_id).ok_or(Error::<T>::TokenNotFound)?;
			if let Some(offer) = Offers::<T>::get((collection_id, token_id), who.clone()){
				T::Currency::unreserve(&who, offer); 
				Offers::<T>::remove((collection_id, token_id), who.clone());
				Self::deposit_event(RawEvent::CancelOffer(collection_id, token_id, token.owner, who));
				Ok(())
			} else {
				fail!(Error::<T>::OfferNotFound);
			}
		}

		#[weight = T::BlockWeights::get().max_block / 100]
		pub fn appreciate(origin,
			collection_id: T::ClassId,
			token_id: T::TokenId,
			amount: BalanceOf<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// token exists check
			let token = nft::Module::<T>::tokens(collection_id, token_id).ok_or(Error::<T>::TokenNotFound)?;

			let balance = T::Currency::free_balance(&who);
			ensure!(balance >= amount, Error::<T>::BalanceNotEnough);

			T::Currency::transfer(&who, &token.owner, amount, ExistenceRequirement::AllowDeath)?;
			Self::deposit_event(RawEvent::AppreciationReceived(collection_id, token_id, amount));

			Ok(())	
		}

		#[weight = T::BlockWeights::get().max_block / 100]
		pub fn toggle_display(origin,
			collection_id: T::ClassId,
			token_id: T::TokenId,
			display: bool) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// token exists check
			let token = nft::Module::<T>::tokens(collection_id, token_id).ok_or(Error::<T>::TokenNotFound)?;
			ensure!(token.owner == who, Error::<T>::MustBeTokenOwner);

			// get token info
			let mut info = TokenExtendedInfo::<T>::get(collection_id, token_id).unwrap_or_else(|| ExtendedInfo {
				display_flag: false,
				report: ReportReason::None,
				frozen: false
			});
			info.display_flag = display;

			TokenExtendedInfo::<T>::insert(collection_id, token_id, info);
			Self::deposit_event(RawEvent::ToggleDisplay(display));

			Ok(())	
		}

		#[weight = T::BlockWeights::get().max_block / 100]
		pub fn report(origin,
			collection_id: T::ClassId,
			token_id: T::TokenId,
			reason: ReportReason) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// token exists check
			ensure!(nft::Module::<T>::tokens(collection_id, token_id).is_some(), Error::<T>::TokenNotFound);

			// get token info
			let mut info = TokenExtendedInfo::<T>::get(collection_id, token_id).unwrap_or_else(|| ExtendedInfo {
				display_flag: false,
				report: ReportReason::None,
				frozen: false
			});
			info.report = reason.clone();

			TokenExtendedInfo::<T>::insert(collection_id, token_id, info);
			Self::deposit_event(RawEvent::ArtReported(collection_id, token_id, reason));

			Ok(())	
		}

		#[weight = T::BlockWeights::get().max_block / 100]
		pub fn accept_report(origin,
			collection_id: T::ClassId,
			token_id: T::TokenId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// token exists check
			ensure!(nft::Module::<T>::tokens(collection_id, token_id).is_some(), Error::<T>::TokenNotFound);

			ensure!(Curator::<T>::get() == who, Error::<T>::MustBeCurator);

			// get token info
			let mut info = TokenExtendedInfo::<T>::get(collection_id, token_id).unwrap_or_else(|| ExtendedInfo {
				display_flag: false,
				report: ReportReason::None,
				frozen: false
			});
			info.report = ReportReason::Reported;

			Self::deposit_event(RawEvent::ArtReportAccepted(collection_id, token_id));

			Ok(())	
		}

		#[weight = T::BlockWeights::get().max_block / 100]
		pub fn clear_report(origin,
			collection_id: T::ClassId,
			token_id: T::TokenId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// token exists check
			ensure!(nft::Module::<T>::tokens(collection_id, token_id).is_some(), Error::<T>::TokenNotFound);

			ensure!(Curator::<T>::get() == who, Error::<T>::MustBeCurator);

			// get token info
			let mut info = TokenExtendedInfo::<T>::get(collection_id, token_id).unwrap_or_else(|| ExtendedInfo {
				display_flag: false,
				report: ReportReason::None,
				frozen: false
			});
			info.report = ReportReason::Reported;

			Self::deposit_event(RawEvent::ArtReportCleared(collection_id, token_id));

			Ok(())	
		}

		#[weight = T::BlockWeights::get().max_block / 100]
		pub fn set_curator(origin,
			curator: T::AccountId) -> DispatchResult {
			let _ = ensure_root(origin)?;

			Curator::<T>::put(curator);	

			Ok(())	
		}
	}
}


#[derive(Clone, Eq, PartialEq, RuntimeDebug, Encode, Decode)]
pub struct GallerySwapAction<T: Config>{
	collection_id: T::ClassId,
	token_id: T::TokenId
}

impl<T: Config> pallet_atomic_swap::SwapAction<<T as frame_system::Config>::AccountId, T> for GallerySwapAction<T> 
{
    fn reserve(&self, source: &<T as frame_system::Config>::AccountId) -> frame_support::dispatch::DispatchResult {
		if let Some(token) = nft::Module::<T>::tokens(self.collection_id, self.token_id){
			ensure!(token.owner == *source, Error::<T>::MustBeTokenOwner);
			// get token info
			let mut info = TokenExtendedInfo::<T>::get(self.collection_id, self.token_id).unwrap_or_else(|| ExtendedInfo {
				display_flag: false,
				report: ReportReason::None,
				frozen: false
			});
			// if token is already frozen, it is already being used in a swap!
			ensure!(info.frozen == false, Error::<T>::TokenFrozen);
			info.frozen = true;
			TokenExtendedInfo::<T>::insert(self.collection_id, self.token_id, info);
		} else {
			fail!(Error::<T>::TokenNotFound)
		}
		Ok(())
    }

    fn claim(&self, source: &<T as frame_system::Config>::AccountId, target: &<T as frame_system::Config>::AccountId) -> bool {
        if let Some(token) = nft::Module::<T>::tokens(self.collection_id, self.token_id){
			if token.owner == *source {
				nft::Module::<T>::transfer(source, target, (self.collection_id, self.token_id)).is_ok()
			} else {
				false
			}
		} else {
			false
		}
    }

    fn weight(&self) -> frame_support::dispatch::Weight {
		//TODO
        T::BlockWeights::get().max_block / 50
    }

    fn cancel(&self, source: &<T as frame_system::Config>::AccountId) {
        if let Some(token) = nft::Module::<T>::tokens(self.collection_id, self.token_id){
			if token.owner == *source {
				// get token info
				let mut info = TokenExtendedInfo::<T>::get(self.collection_id, self.token_id).unwrap_or_else(|| ExtendedInfo {
					display_flag: false,
					report: ReportReason::None,
					frozen: false
				});
				//unfreeze
				info.frozen = false;
				TokenExtendedInfo::<T>::insert(self.collection_id, self.token_id, info);
			}
		}
    }
}