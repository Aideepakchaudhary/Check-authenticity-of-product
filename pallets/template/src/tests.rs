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
fn add_manufacture_fail() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		// Dispatch a signed extrinsic.

		assert_noop!(TemplateModule::add_manufacturer(RuntimeOrigin::signed(1),TEST_ACCOUNT),
			sp_runtime::DispatchError::BadOrigin
		);
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

#[test]
fn add_product_from_sudo_fail() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;

		assert_ok!(TemplateModule::add_manufacturer(RuntimeOrigin::root(), TEST_ACCOUNT));
		let hash = HashType::from(Hashing::hash_of(&42));

		assert_noop!(TemplateModule::add_product(RuntimeOrigin::root(),hash),
			sp_runtime::DispatchError::BadOrigin
		);

	})
}

#[test]
fn add_duplicate_product_fail() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;

		assert_ok!(TemplateModule::add_manufacturer(RuntimeOrigin::root(), TEST_ACCOUNT));
		let hash = HashType::from(Hashing::hash_of(&42));

		assert_ok!(TemplateModule::add_product(RuntimeOrigin::signed(1), hash));

		assert_noop!(TemplateModule::add_product(RuntimeOrigin::signed(1), hash),
			Error::<Test>::ProductAlreadyPresent
		);

	})
}

#[test]
fn add_product_by_unauthorised_person_fail() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;

		assert_ok!(TemplateModule::add_manufacturer(RuntimeOrigin::root(), TEST_ACCOUNT));
		let hash = HashType::from(Hashing::hash_of(&42));

		assert_noop!(TemplateModule::add_product(RuntimeOrigin::signed(2), hash),
			Error::<Test>::UnAuthorisedPerson
		);

	})
}

#[test]
fn check_authenticity_successfully() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;

		assert_ok!(TemplateModule::add_manufacturer(RuntimeOrigin::root(), TEST_ACCOUNT));
		let hash = HashType::from(Hashing::hash_of(&42));

		assert_ok!(TemplateModule::add_product(RuntimeOrigin::signed(1), hash));

		assert_ok!(TemplateModule::check_authenticity(RuntimeOrigin::signed(1), hash));

	})
}

#[test]
fn check_authenticity_fail() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;

		assert_ok!(TemplateModule::add_manufacturer(RuntimeOrigin::root(), TEST_ACCOUNT));
		let hash = HashType::from(Hashing::hash_of(&42));

		assert_ok!(TemplateModule::add_product(RuntimeOrigin::signed(1), hash));

		let check_hash = HashType::from(Hashing::hash_of(&43));
		assert_noop!(TemplateModule::check_authenticity(RuntimeOrigin::signed(1), check_hash),
			Error::<Test>::UnAuthenticProduct
		);
	})
}

#[test]
fn check_authenticity_from_invalid_origin_fail() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;

		assert_ok!(TemplateModule::add_manufacturer(RuntimeOrigin::root(), TEST_ACCOUNT));
		let hash = HashType::from(Hashing::hash_of(&42));

		assert_ok!(TemplateModule::add_product(RuntimeOrigin::signed(1), hash));

		let check_hash = HashType::from(Hashing::hash_of(&43));
		assert_noop!(TemplateModule::check_authenticity(RuntimeOrigin::root(), check_hash),
			sp_runtime::DispatchError::BadOrigin
		);
	})
}

#[test]
fn refund_products_successfully() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;

		assert_ok!(TemplateModule::add_manufacturer(RuntimeOrigin::root(), TEST_ACCOUNT));
		let hash = HashType::from(Hashing::hash_of(&42));

		assert_ok!(TemplateModule::add_product(RuntimeOrigin::signed(1), hash));

		assert_ok!(TemplateModule::check_authenticity(RuntimeOrigin::signed(1), hash));

		assert_ok!(TemplateModule::refund_products(RuntimeOrigin::signed(1), hash));
	})
}

#[test]
fn duplicate_refund_products_fail() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;

		assert_ok!(TemplateModule::add_manufacturer(RuntimeOrigin::root(), TEST_ACCOUNT));
		let hash = HashType::from(Hashing::hash_of(&42));

		assert_ok!(TemplateModule::add_product(RuntimeOrigin::signed(1), hash));

		assert_ok!(TemplateModule::check_authenticity(RuntimeOrigin::signed(1), hash));

		assert_ok!(TemplateModule::refund_products(RuntimeOrigin::signed(1), hash));

		assert_noop!(TemplateModule::refund_products(RuntimeOrigin::signed(1), hash),
			Error::<Test>::UnsoldProduct
		);
	})
}

#[test]
fn refund_products_from_incorrect_origin_fail() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;

		assert_ok!(TemplateModule::add_manufacturer(RuntimeOrigin::root(), TEST_ACCOUNT));
		let hash = HashType::from(Hashing::hash_of(&42));

		assert_ok!(TemplateModule::add_product(RuntimeOrigin::signed(1), hash));

		assert_ok!(TemplateModule::check_authenticity(RuntimeOrigin::signed(1), hash));

		assert_noop!(TemplateModule::refund_products(RuntimeOrigin::root(), hash),
			sp_runtime::DispatchError::BadOrigin
		);
	})
}
