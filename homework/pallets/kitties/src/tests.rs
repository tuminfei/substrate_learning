use crate::{mock::*, *};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_create() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 0;

		// set balance
		assert_ok!(Balances::force_set_balance(RuntimeOrigin::root(), account_id, 100000000000));

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id);
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), *b"1234"));

		// assert event
		System::assert_last_event(
			Event::KittyCreated {
				who: account_id,
				kitty_id,
				kitty: KittiesModule::kitties(kitty_id).unwrap(),
			}
			.into(),
		);

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1);
		assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(kitty_id), None);

		crate::NextKittyId::<Test>::set(crate::KittyId::max_value());
		assert_noop!(
			KittiesModule::create(RuntimeOrigin::signed(account_id), *b"1234"),
			Error::<Test>::InvalidKittyId
		);
	});
}

#[test]
fn it_works_for_bred() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;

		// set balance
		assert_ok!(Balances::force_set_balance(RuntimeOrigin::root(), account_id, 100000000000));

		assert_noop!(
			KittiesModule::bred(RuntimeOrigin::signed(account_id), kitty_id, kitty_id, *b"1234"),
			Error::<Test>::SameKittyId
		);

		assert_noop!(
			KittiesModule::bred(RuntimeOrigin::signed(account_id), kitty_id, kitty_id, *b"1234"),
			Error::<Test>::SameKittyId
		);

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), *b"1234"));
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), *b"5678"));

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 2);

		assert_ok!(KittiesModule::bred(
			RuntimeOrigin::signed(account_id),
			kitty_id,
			kitty_id + 1,
			*b"1234"
		));

		// assert event
		System::assert_last_event(
			Event::KittyBred {
				who: account_id,
				kitty_id: kitty_id + 2,
				kitty: KittiesModule::kitties(kitty_id + 2).unwrap(),
			}
			.into(),
		);

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
		let kitty_id = 0;
		let account_id = 1;
		let recipient = 2;

		// set balance
		assert_ok!(Balances::force_set_balance(RuntimeOrigin::root(), account_id, 100000000000));

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), *b"1234"));
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

		assert_noop!(
			KittiesModule::transfer(RuntimeOrigin::signed(recipient), kitty_id, account_id),
			Error::<Test>::NotOwner
		);

		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(account_id), kitty_id, recipient));

		// assert event
		System::assert_last_event(
			Event::KittyTransfer { who: account_id, kitty_id, recipient }.into(),
		);

		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(recipient));

		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(recipient), kitty_id, account_id));

		// assert event
		System::assert_last_event(
			Event::KittyTransfer { who: recipient, kitty_id, recipient: account_id }.into(),
		);

		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
	});
}
