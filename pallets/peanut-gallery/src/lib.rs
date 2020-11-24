#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_event, dispatch};
use frame_support::traits::EnsureOrigin;

pub trait Trait: frame_system::Trait {
	type Event: From<Event> + Into<<Self as frame_system::Trait>::Event>;
	type DumboOrigin: frame_support::traits::EnsureOrigin<Self::Origin>;
}

decl_event!(
	pub enum Event {
		/// Dumbo wants peanuts.
		DumboWantsPeanuts,
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// Dumbo wants peanuts.
		/// The dispatch origin for this call must be Dumbo.
		#[weight = 1]
		pub fn dumbo_wants_peanuts(origin) -> dispatch::DispatchResult {
			T::DumboOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::DumboWantsPeanuts);
			Ok(())
		}
	}
}
