use core::ops::Index;

use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, pallet_prelude::DispatchError, traits::ExtrinsicCall};
use sp_core::{offchain::OffchainStorage, ByteArray, Decode, Encode};
use sp_externalities::Externalities;
use sp_keystore::Keystore;
use sp_runtime::{offchain::storage::StorageValueRef, testing::TestXt};
use sp_std::collections::btree_map::BTreeMap;

fn events() -> Vec<Event<Test>> {
	let result =
		System::events()
			.into_iter()
			.map(|r| r.event)
			.filter_map(|e| {
				if let RuntimeEvent::TemplateModule(inner) = e {
					Some(inner)
				} else {
					None
				}
			})
			.collect();

	System::reset_events();

	result
}

#[test]
fn enroll_as_provider_works() {
	new_mock_runtime(|mut ext, _, _, _| {
		ext.execute_with(|| {
			let account_id = sp_core::sr25519::Public::from_raw([1u8; 32]);

			assert_ok!(TemplateModule::enroll_as_provider(RuntimeOrigin::signed(account_id)));

			System::assert_last_event(Event::<Test>::ProviderEnrolled(account_id).into());
		});
	});
}

#[test]
fn enroll_as_provider_fails_with_non_signed_origin() {
	new_mock_runtime(|mut ext, _, _, _| {
		ext.execute_with(|| {
			assert_noop!(
				TemplateModule::enroll_as_provider(RuntimeOrigin::none()),
				DispatchError::BadOrigin
			);
		});
	});
}

#[test]
fn enroll_as_provider_fails_when_already_enrolled() {
	new_mock_runtime(|mut ext, _, _, _| {
		ext.execute_with(|| {
			let account_id = sp_core::sr25519::Public::from_raw([1u8; 32]);

			assert_ok!(TemplateModule::enroll_as_provider(RuntimeOrigin::signed(account_id)));

			assert_noop!(
				TemplateModule::enroll_as_provider(RuntimeOrigin::signed(account_id)),
				Error::<Test>::AlreadyAProvider
			);
		});
	});
}

#[test]
fn create_enclave_request_works() {
	new_mock_runtime(|mut ext, _, _, _| {
		ext.execute_with(|| {
			let account_id = sp_core::sr25519::Public::from_raw([1u8; 32]);

			assert_ok!(TemplateModule::create_enclave_request(
				RuntimeOrigin::signed(account_id),
				None,
				crate::EnclaveRequestParam::new(crate::EnclaveAction::CreateEnclave {}, None)
			));

			System::assert_last_event(Event::<Test>::EnclaveRequestCreated(1).into());
		});
	});
}

#[test]
fn create_enclave_request_fails_with_non_signed_origin() {
	new_mock_runtime(|mut ext, _, _, _| {
		ext.execute_with(|| {
			assert_noop!(
				TemplateModule::create_enclave_request(
					RuntimeOrigin::none(),
					None,
					crate::EnclaveRequestParam::new(crate::EnclaveAction::CreateEnclave {}, None)
				),
				DispatchError::BadOrigin
			);
		});
	});
}

#[test]
fn acknowledge_enclave_request_works() {
	new_mock_runtime(|mut ext, _, _, _| {
		ext.execute_with(|| {
			let account_id = sp_core::sr25519::Public::from_raw([1u8; 32]);
			let handler_id = sp_core::sr25519::Public::from_raw([2u8; 32]);

			assert_ok!(TemplateModule::enroll_as_provider(RuntimeOrigin::signed(handler_id)));

			assert_ok!(TemplateModule::create_enclave_request(
				RuntimeOrigin::signed(account_id),
				Some(handler_id),
				crate::EnclaveRequestParam::new(crate::EnclaveAction::CreateEnclave {}, None)
			));

			assert_ok!(TemplateModule::acknowledge_enclave_request(
				RuntimeOrigin::signed(handler_id),
				1
			));

			System::assert_last_event(Event::EnclaveRequestAcknowledged(1).into());
		});
	});
}

#[test]
fn acknowledge_enclave_request_fails_for_nonexistent_request() {
	new_mock_runtime(|mut ext, _, _, _| {
		ext.execute_with(|| {
			let account_id = sp_core::sr25519::Public::from_raw([1u8; 32]);

			assert_noop!(
				TemplateModule::acknowledge_enclave_request(RuntimeOrigin::signed(account_id), 999),
				Error::<Test>::RequestNotFound
			);
		});
	});
}

#[test]
fn acknowledge_enclave_request_fails_for_unauthorized_handler() {
	new_mock_runtime(|mut ext, _, _, _| {
		ext.execute_with(|| {
			let account_id = sp_core::sr25519::Public::from_raw([1u8; 32]);
			let handler = sp_core::sr25519::Public::from_raw([2u8; 32]);

			let unauthorized_handler = sp_core::sr25519::Public::from_raw([3u8; 32]);

			assert_ok!(TemplateModule::create_enclave_request(
				RuntimeOrigin::signed(account_id),
				Some(handler),
				crate::EnclaveRequestParam::new(crate::EnclaveAction::CreateEnclave {}, None)
			));

			assert_noop!(
				TemplateModule::acknowledge_enclave_request(
					RuntimeOrigin::signed(unauthorized_handler),
					1
				),
				Error::<Test>::NotAuthorizedHandler
			);
		});
	});
}

#[test]
fn acknowledge_enclave_request_fails_if_already_acknowledged() {
	new_mock_runtime(|mut ext, _, _, _| {
		ext.execute_with(|| {
			let account_id = sp_core::sr25519::Public::from_raw([1u8; 32]);
			let handler = sp_core::sr25519::Public::from_raw([2u8; 32]);

			assert_ok!(TemplateModule::create_enclave_request(
				RuntimeOrigin::signed(account_id),
				None,
				crate::EnclaveRequestParam::new(crate::EnclaveAction::CreateEnclave {}, None)
			));

			assert_ok!(TemplateModule::enroll_as_provider(RuntimeOrigin::signed(handler)));

			assert_ok!(TemplateModule::acknowledge_enclave_request(
				RuntimeOrigin::signed(handler),
				1
			));

			assert_noop!(
				TemplateModule::acknowledge_enclave_request(RuntimeOrigin::signed(handler), 1),
				Error::<Test>::RequestNotFound
			);
		});
	});
}

#[test]
fn test_create_enclave_and_setup_enclave_works() {
	new_mock_runtime(|mut ext, _, keystore, pool_state| {
		ext.execute_with(|| {
			assert_ok!(keystore.insert(crate::KEY_TYPE, "//provider", &[1u8; 32]));
			assert_ok!(keystore.insert(crate::KEY_TYPE, "//conduit", &[2u8; 32]));
			assert_ok!(keystore.insert(crate::KEY_TYPE, "//user", &[3u8; 32]));

			let provider = keystore.sr25519_public_keys(crate::KEY_TYPE)[0];
			let conduit_node = keystore.sr25519_public_keys(crate::KEY_TYPE)[1];
			let user = keystore.sr25519_public_keys(crate::KEY_TYPE)[2];
			let request_id = 1;

			assert_ok!(TemplateModule::enroll_as_provider(RuntimeOrigin::signed(provider)));

			assert_ok!(TemplateModule::create_enclave_request(
				RuntimeOrigin::signed(user),
				Some(provider),
				crate::EnclaveRequestParam::new(crate::EnclaveAction::CreateEnclave {}, None)
			));

			System::assert_last_event(Event::<Test>::EnclaveRequestCreated(request_id).into());

			TemplateModule::handle_enclave_request_created(request_id);

			let tx = pool_state.write().transactions.pop().unwrap();
			let tx = TestXt::<RuntimeCall, ()>::decode(&mut &*tx).unwrap();

			match tx.call {
				RuntimeCall::TemplateModule(crate::Call::acknowledge_enclave_request {
					request_id,
				}) => {
					assert_ok!(TemplateModule::acknowledge_enclave_request(
						RuntimeOrigin::signed(provider),
						request_id
					));
				},
				_ => assert!(false, "Test failed due to unexpected call encountered"),
			}

			System::assert_last_event(Event::EnclaveRequestAcknowledged(request_id).into());
			System::reset_events();

			TemplateModule::handle_enclave_request_acknowledged(request_id);

			let mut pending_authorized_nodes = BTreeMap::new();
			pending_authorized_nodes.insert(request_id, conduit_node);

			let storage_ref =
				StorageValueRef::persistent(crate::PENDING_AUTHORIZED_CONDUIT_NODES_STORAGE);
			storage_ref.set(&pending_authorized_nodes);

			TemplateModule::process_pending_authorized_conduit_nodes();

			let tx = pool_state.write().transactions.pop().unwrap();
			let tx = TestXt::<RuntimeCall, ()>::decode(&mut &*tx).unwrap();

			match tx.call {
				RuntimeCall::TemplateModule(crate::Call::process_enclave_request {
					request_id,
					outcome,
				}) => {
					assert_ok!(TemplateModule::process_enclave_request(
						RuntimeOrigin::signed(provider),
						request_id,
						outcome
					));
				},
				_ => assert!(false, "Test failed due to unexpected call encountered"),
			}

			assert_eq!(
				crate::Enclaves::<Test>::get(&conduit_node),
				Some(crate::EnclaveInfo::new(provider, user, crate::EnclaveStatus::Pending)),
			);

			assert_eq!(
				events(),
				vec![
					Event::EnclaveRequestCreated(2),
					Event::EnclaveRequestProcessed {
						request_id,
						handle: provider,
						outcome: crate::Outcome::EnclaveCreated { handle: conduit_node },
					}
				]
			);

			TemplateModule::handle_enclave_request_created(2);

			let tx = pool_state.write().transactions.pop().unwrap();
			let tx = TestXt::<RuntimeCall, ()>::decode(&mut &*tx).unwrap();

			match tx.call {
				RuntimeCall::TemplateModule(crate::Call::acknowledge_enclave_request {
					request_id,
				}) => {
					assert_ok!(TemplateModule::acknowledge_enclave_request(
						RuntimeOrigin::signed(conduit_node),
						request_id
					));
				},
				_ => assert!(false, "Test failed due to unexpected call encountered"),
			}

			System::assert_last_event(Event::EnclaveRequestAcknowledged(2).into());

			TemplateModule::handle_enclave_request_acknowledged(2);

			let tx = pool_state.write().transactions.pop().unwrap();
			let tx = TestXt::<RuntimeCall, ()>::decode(&mut &*tx).unwrap();

			match tx.call {
				RuntimeCall::TemplateModule(crate::Call::process_enclave_request {
					request_id,
					outcome,
				}) => {
					assert_ok!(TemplateModule::process_enclave_request(
						RuntimeOrigin::signed(conduit_node),
						request_id,
						outcome
					));
				},
				_ => assert!(false, "Test failed due to unexpected call encountered"),
			}
            
			System::assert_last_event(
				Event::EnclaveRequestProcessed {
					request_id: 2,
					handle: conduit_node,
					outcome: crate::Outcome::EnclaveSetupCompleted {},
				}
				.into(),
			);

			assert_eq!(
				crate::Enclaves::<Test>::get(&conduit_node),
				Some(crate::EnclaveInfo::new(provider, user, crate::EnclaveStatus::Active)),
			);
		});
	});
}

#[test]
fn set_enclave_status_works() {
	new_mock_runtime(|mut ext, _, keystore, pool_state| {
		ext.execute_with(|| {
			assert_ok!(keystore.insert(crate::KEY_TYPE, "//provider", &[1u8; 32]));
			assert_ok!(keystore.insert(crate::KEY_TYPE, "//conduit", &[2u8; 32]));
			assert_ok!(keystore.insert(crate::KEY_TYPE, "//user", &[3u8; 32]));

			let provider = keystore.sr25519_public_keys(crate::KEY_TYPE)[0];
			let conduit = keystore.sr25519_public_keys(crate::KEY_TYPE)[1];
			let user = keystore.sr25519_public_keys(crate::KEY_TYPE)[2];

			crate::Enclaves::<Test>::insert(
				conduit,
				crate::EnclaveInfo::new(provider, user, crate::EnclaveStatus::Pending),
			);

			assert_ok!(TemplateModule::set_enclave_status(
				RuntimeOrigin::signed(conduit),
				crate::EnclaveStatus::Active
			));

			assert_eq!(
				crate::Enclaves::<Test>::get(conduit).unwrap().status(),
				&crate::EnclaveStatus::Active
			);
		});
	});
}
