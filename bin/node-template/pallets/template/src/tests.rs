use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn pairing_computation_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::pairing_arkworks(RuntimeOrigin::signed(1), 42));
	});
}

#[test]
fn groth16_verification_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::verify_groth16(RuntimeOrigin::signed(1), 42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			TemplateModule::cause_error(RuntimeOrigin::signed(1)),
			Error::<Test>::NoneValue
		);
	});
}
