#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use sp_runtime::traits::Bounded;

use crate::Module as Gallery;

fn last_event() -> crate::mock::Event {
    frame_system::Module::<crate::mock::Test>::events()
        .pop()
        .expect("Event expected")
        .event
}

benchmarks! {
    set_curator {
        let curator: T::AccountId = whitelisted_caller();
    }: set_curator(RawOrigin::Root, curator.clone())
    verify {
        assert_eq!(curator, Gallery::<T>::curator());
    }

    create_collection {
        let caller: T::AccountId = whitelisted_caller();
    }: create_collection(RawOrigin::Signed(caller.clone()), Vec::<u8>::default(), ClassData::default())
    verify {
        assert_eq!(
            last_event(),
            crate::mock::Event::pallet_gallery(crate::RawEvent::CollectionCreated(0)),
        );
    }

    mint {
        let caller: T::AccountId = whitelisted_caller();
        Gallery::<T>::create_collection(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone())), Vec::<u8>::default(), ClassData::default());
    }: mint(RawOrigin::Signed(caller.clone()), Default::default(), Vec::<u8>::default(), TokenData::default())
    verify {
        assert_eq!(
            last_event(),
            crate::mock::Event::pallet_gallery(crate::RawEvent::NFTCreated(0, 0)),
        );
    }

    // TODO: use non-default balance to appreciate
    appreciate {
        let caller: T::AccountId = whitelisted_caller();
        Gallery::<T>::create_collection(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone())), Vec::<u8>::default(), ClassData::default());
        Gallery::<T>::mint(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone())), Default::default(), Vec::<u8>::default(), TokenData::default());
    }: appreciate(RawOrigin::Signed(caller.clone()), Default::default(), Default::default(), Default::default())
    verify {
        assert_eq!(
          last_event(),
          crate::mock::Event::pallet_gallery(crate::RawEvent::AppreciationReceived(0, 0, 0)),
        );
    }

    toggle_display {
        let caller: T::AccountId = whitelisted_caller();
        Gallery::<T>::create_collection(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone())), Vec::<u8>::default(), ClassData::default());
        Gallery::<T>::mint(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone())), Default::default(), Vec::<u8>::default(), TokenData::default());
    }: appreciate(RawOrigin::Signed(caller.clone()), Default::default(), Default::default(), Default::default())
    verify {
        assert_eq!(
          last_event(),
          crate::mock::Event::pallet_gallery(crate::RawEvent::AppreciationReceived(0, 0, 0)),
        );
    }

    burn {
        let caller: T::AccountId = whitelisted_caller();
        Gallery::<T>::create_collection(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone())), Vec::<u8>::default(), ClassData::default());
        Gallery::<T>::mint(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone())), Default::default(), Vec::<u8>::default(), TokenData::default());
    }: burn(RawOrigin::Signed(caller.clone()), Default::default(), Default::default())
    verify {
        assert_eq!(
          last_event(),
          crate::mock::Event::pallet_gallery(crate::RawEvent::NFTBurned(0, 0)),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::*;
    use frame_support::assert_ok;

    #[test]
    fn set_curator() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_set_curator::<Test>());
        });
    }

    #[test]
    fn create_collection() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_create_collection::<Test>());
        });
    }

    #[test]
    fn appreciate() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_appreciate::<Test>());
        });
    }

    #[test]
    fn mint() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_mint::<Test>());
        });
    }

    #[test]
    fn burn() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_burn::<Test>());
        });
    }
}
