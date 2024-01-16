#![allow(deprecated)]

mod tests_common;

use fair_launch::{
    common::CommonModule, initial_launch::InitialLaunchModule, transfer::TransferModule,
};
use multiversx_sc::types::MultiValueEncoded;
use multiversx_sc_scenario::{managed_address, managed_biguint, managed_token_id, rust_biguint};
use tests_common::*;

#[test]
fn init_test() {
    let _ = FairLaunchSetup::new(fair_launch::contract_obj);
}

#[test]
fn percentage_test() {
    let mut fl_setup = FairLaunchSetup::new(fair_launch::contract_obj);
    fl_setup.b_mock.set_block_nonce(10);
    fl_setup
        .b_mock
        .execute_query(&fl_setup.fl_wrapper, |sc| {
            let percentage =
                sc.get_fee_percentage(BUY_FEE_PERCENTAGE_START, BUY_FEE_PERCENTAGE_END);
            let expected_percentage = BUY_FEE_PERCENTAGE_START - 808; // (BUY_FEE_PERCENTAGE_END - BUY_FEE_PERCENTAGE_START) * 10 blocks / (100 blocks - 1) ~= 808
            assert_eq!(percentage, expected_percentage);
        })
        .assert_ok();
}

#[test]
fn calculate_fee_test() {
    let mut fl_setup = FairLaunchSetup::new(fair_launch::contract_obj);
    fl_setup
        .b_mock
        .execute_query(&fl_setup.fl_wrapper, |sc| {
            let fee = sc.calculate_fee_rounded_up(&managed_biguint!(1_000), 4_000);
            let expected_fee = managed_biguint!(400);
            assert_eq!(fee, expected_fee);

            let fee = sc.calculate_fee_rounded_up(&managed_biguint!(1), 4_000);
            let expected_fee = managed_biguint!(1);
            assert_eq!(fee, expected_fee);

            let fee = sc.calculate_fee_rounded_up(&managed_biguint!(1_001), 4_000);
            let expected_fee = managed_biguint!(401);
            assert_eq!(fee, expected_fee);
        })
        .assert_ok();
}

#[test]
fn transfer_test() {
    let mut fl_setup = FairLaunchSetup::new(fair_launch::contract_obj);
    fl_setup
        .b_mock
        .execute_tx(
            &fl_setup.owner_address,
            &fl_setup.fl_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_token_fees(managed_token_id!(TOKEN_ID), 4_000);
            },
        )
        .assert_ok();

    fl_setup.b_mock.set_esdt_balance(
        &fl_setup.first_user_address,
        TOKEN_ID,
        &rust_biguint!(1_000),
    );

    fl_setup
        .b_mock
        .execute_esdt_transfer(
            &fl_setup.first_user_address,
            &fl_setup.fl_wrapper,
            TOKEN_ID,
            0,
            &rust_biguint!(1_000),
            |sc| {
                sc.forward_transfer(
                    managed_address!(&fl_setup.second_user_address),
                    MultiValueEncoded::new(),
                );
            },
        )
        .assert_ok();

    fl_setup
        .b_mock
        .check_esdt_balance(&fl_setup.first_user_address, TOKEN_ID, &rust_biguint!(0));

    fl_setup.b_mock.check_esdt_balance(
        &fl_setup.second_user_address,
        TOKEN_ID,
        &rust_biguint!(600),
    );

    fl_setup
        .b_mock
        .check_esdt_balance(&fl_setup.owner_address, TOKEN_ID, &rust_biguint!(400));
}
