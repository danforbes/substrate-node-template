use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn create_collection() {
    new_test_ext().execute_with(|| {
        assert_ok!(create_default_collection());

        assert_eq!(
            last_event(),
            crate::mock::Event::pallet_bca(crate::Event::CollectionCreated(0, ALICE)),
        );
    });
}
