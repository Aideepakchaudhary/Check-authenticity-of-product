use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::{BlakeTwo256, Hash};

pub type HashType = <Test as frame_system::Config>::Hash;
pub type Hashing = <Test as frame_system::Config>::Hashing;

#[test]
fn add_manufacture_successfully() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		// Dispatch a signed extrinsic.

		assert_ok!(TemplateModule::add_manufacturer(RuntimeOrigin::root(), TEST_ACCOUNT));
	})
}

#[test]
fn add_duplicate_manufacture_fail() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		// Dispatch a signed extrinsic.

		assert_ok!(TemplateModule::add_manufacturer(RuntimeOrigin::root(), TEST_ACCOUNT));
		assert_noop!(
			TemplateModule::add_manufacturer(RuntimeOrigin::root(), TEST_ACCOUNT),
			Error::<Test>::ManufacturerAlreadyPresent
		);

	})
}

#[test]
fn add_product_successfully() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;

		assert_ok!(TemplateModule::add_manufacturer(RuntimeOrigin::root(), TEST_ACCOUNT));
		let hash = HashType::from(Hashing::hash_of(&42));

		assert_ok!(TemplateModule::add_product(RuntimeOrigin::signed(1), hash));

	})
}
