use super::*;

#[allow(unused)]
use crate::Module as Bca;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::{EventRecord, RawOrigin};

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	let events = frame_system::Pallet::<T>::events();
	let system_event: <T as frame_system::Config>::Event = generic_event.into();
	let EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}

benchmarks! {
    create_collection {
        let s in 0 .. 100;
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), Vec::<u8>::default(), ClassData::default())
    verify {
        assert_last_event::<T>(crate::Event::CollectionCreated(Default::default(), whitelisted_caller()).into())
    }
}

impl_benchmark_test_suite!(Bca, crate::mock::new_test_ext(), crate::mock::Test);
