use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_create() {
	new_test_ext().execute_with(|| {
		let ketty_id = 0;
		let account_id = 0;

		assert_eq!(KittiesModule::next_kitty_id(), ketty_id);
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

		assert_eq!(KittiesModule::next_kitty_id(), ketty_id + 1);
		assert_eq!(KittiesModule::kitties(ketty_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(ketty_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(ketty_id), None);

		crate::NextKittyId::<Test>::set(crate::KittyId::max_value());
		assert_noop!(
			KittiesModule::create(Origin::signed(account_id)),
			Error::<Test>::InvalidKittyId
		)
	});
}

#[test]
fn it_works_for_bred() {
	new_test_ext().execute_with(|| {
		let ketty_id = 0;
		let account_id = 1;

		assert_noop!(
			KittiesModule::bred(Origin::signed(account_id), kitty_id, kitty_id),
			Error::<Test>::SameKittyId
		)

		assert_noop!(
			KittiesModule::bred(Origin::signed(account_id), kitty_id, kitty_id),
			Error::<Test>::SameKittyId
		)

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

		assert_eq!(KittiesModule::next_kitty_id(), ketty_id + 2);

		assert_ok!(KittiesModule::bred(RuntimeOrigin::signed(account_id), ketty_id, ketty_id + 1));

		let bred_kitty_id = 2;
		assert_eq!(KittiesModule::next_kitty_id(), bred_kitty_id + 1);
		assert_eq!(KittiesModule::kitties(bred_kitty_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(bred_kitty_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(bred_kitty_id), Some((kitty_id, kitty_id + 1)));
	});
}

#[test]
fn it_works_for_transfer() {
	new_test_ext().execute_with(|| {
		let ketty_id = 0;
		let account_id = 1;
		let recipient = 2

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_eq!(KittiesModule::kitty_owner(bred_kitty_id), Some(account_id));

		assert_noop!(
			KittiesModule::transfer(Origin::signed(recipient), kitty_id, kitty_id),
			Error::<Test>::NotOwner
		)

		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(account_id)), kitty_id, recipient);

		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(recipient));

		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(recipient)), kitty_id, account_id);

		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
	});
}