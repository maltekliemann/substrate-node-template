use crate::{mock::*, Error, Pallet};
use frame_support::{
	assert_noop, assert_ok,
	pallet_prelude::*,
	storage::storage_prefix,
	traits::{GetStorageVersion, StorageVersion},
};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::something(), Some(42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(TemplateModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
	});
}

#[test]
fn storage_version_is_correct() {
	new_test_ext().execute_with(|| {
		println!(
			"{:?}",
			storage_prefix(<Pallet<Test>>::name().as_bytes(), b":__STORAGE_VERSION__:")
		);
		assert_eq!(<Pallet<Test> as GetStorageVersion>::current_storage_version(), 123);
		assert_eq!(<Pallet<Test> as GetStorageVersion>::on_chain_storage_version(), 123);
		assert_eq!(StorageVersion::get::<Pallet<Test>>(), 123);
	});
}
