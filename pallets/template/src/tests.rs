use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

pub type HashType = <Test as frame_system::Config>::Hash;
pub type Hashing = <Test as frame_system::Config>::Hashing;

#[test]
fn add_manufacture_successfully() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_support::Config>::AccountId = 1;
		// Dispatch a signed extrinsic.

		assert_ok!(TemplateModule::add_manufacturer(RuntimeOrigin::root(), TEST_ACCOUNT));
	})
}
