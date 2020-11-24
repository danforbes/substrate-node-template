#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_event, dispatch};
use frame_system::ensure_signed;

use frame_support::dispatch::Dispatchable;
use frame_support::traits::{Box, Get};

pub trait Trait: frame_system::Trait {
	type Event: From<Event> + Into<<Self as frame_system::Trait>::Event>;
	type Call: frame_support::dispatch::Dispatchable<Origin=<Self as Trait>::Origin> + frame_support::Parameter;
	type DumboAccountId: frame_support::traits::Get<Self::AccountId>;
	type Origin: From<Origin>;
}

// The pallet MUST publish an enum or struct named "Origin".
#[derive(PartialEq, Eq, Clone, sp_runtime::RuntimeDebug, codec::Encode, codec::Decode)]
pub enum Origin {
	Dumbo,
}

/// Helper that other pallets may use to check that a dispatch is from Dumbo.
pub struct EnsureDumbo;
impl<O: Into<Result<Origin, O>> + From<Origin>> frame_support::traits::EnsureOrigin<O> for EnsureDumbo {
	type Success = ();
	fn try_origin(o: O) -> Result<Self::Success, O> {
		o.into().and_then(|o| match o {
			Origin::Dumbo => Ok(()),
		})
	}
}

decl_event!(
	pub enum Event {
		/// A call was proxied as Dumbo. \[result\]
		Dumdid(dispatch::DispatchResult),
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: <T as frame_system::Trait>::Origin {
		fn deposit_event() = default;

		/// Proxy the provided dispatch as the custom Dumbo origin.
		/// The dispatch origin for this call must be signed by T::DumboAccountId.
		#[weight = 1]
		pub fn proxy_dumbo(origin, call: Box<<T as Trait>::Call>) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			frame_support::ensure!(who == T::DumboAccountId::get(), sp_runtime::DispatchError::BadOrigin);
			let res = call.dispatch(Origin::Dumbo.into());
			Self::deposit_event(Event::Dumdid(res.map(|_| ()).map_err(|e| e.error)));
			Ok(())
		}
	}
}
