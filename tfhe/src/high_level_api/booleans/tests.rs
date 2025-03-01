// Without this, clippy will conplain about equal expressions to `ffalse & ffalse`
// However since we overloaded these operators, we want to test them to see
// if they are correct
#![allow(clippy::eq_op)]
#![allow(clippy::bool_assert_comparison)]
use std::ops::{BitAnd, BitOr, BitXor, Not};

use crate::high_level_api::prelude::*;
use crate::high_level_api::{
    generate_keys, set_server_key, ClientKey, CompressedFheBool, ConfigBuilder, FheBool,
    FheBoolParameters,
};

fn setup_static_default() -> ClientKey {
    let config = ConfigBuilder::all_disabled().enable_default_bool().build();

    let (my_keys, server_keys) = generate_keys(config);

    set_server_key(server_keys);
    my_keys
}

fn setup_static_tfhe() -> ClientKey {
    let config = ConfigBuilder::all_disabled()
        .enable_custom_bool(FheBoolParameters::tfhe_lib())
        .build();

    let (my_keys, server_keys) = generate_keys(config);

    set_server_key(server_keys);
    my_keys
}

#[test]
fn test_xor_truth_table_static_default() {
    let keys = setup_static_default();

    let ttrue = FheBool::encrypt(true, &keys);
    let ffalse = FheBool::encrypt(false, &keys);

    xor_truth_table(&ttrue, &ffalse, &keys);
}

#[test]
fn test_and_truth_table_static_default() {
    let keys = setup_static_default();

    let ttrue = FheBool::encrypt(true, &keys);
    let ffalse = FheBool::encrypt(false, &keys);

    and_truth_table(&ttrue, &ffalse, &keys);
}

#[test]
fn test_or_truth_table_static_default() {
    let keys = setup_static_default();

    let ttrue = FheBool::encrypt(true, &keys);
    let ffalse = FheBool::encrypt(false, &keys);

    or_truth_table(&ttrue, &ffalse, &keys);
}

#[test]
fn test_not_truth_table_static_default() {
    let keys = setup_static_default();

    let ttrue = FheBool::encrypt(true, &keys);
    let ffalse = FheBool::encrypt(false, &keys);

    not_truth_table(&ttrue, &ffalse, &keys);
}

#[test]
fn test_xor_truth_table_static_tfhe() {
    let keys = setup_static_tfhe();

    let ttrue = FheBool::encrypt(true, &keys);
    let ffalse = FheBool::encrypt(false, &keys);

    xor_truth_table(&ttrue, &ffalse, &keys);
}

#[test]
fn test_and_truth_table_static_tfhe() {
    let keys = setup_static_tfhe();

    let ttrue = FheBool::encrypt(true, &keys);
    let ffalse = FheBool::encrypt(false, &keys);

    and_truth_table(&ttrue, &ffalse, &keys);
}

#[test]
fn test_or_truth_table_static_tfhe() {
    let keys = setup_static_tfhe();

    let ttrue = FheBool::encrypt(true, &keys);
    let ffalse = FheBool::encrypt(false, &keys);

    or_truth_table(&ttrue, &ffalse, &keys);
}

#[test]
fn test_not_truth_table_static_tfhe() {
    let keys = setup_static_tfhe();

    let ttrue = FheBool::encrypt(true, &keys);
    let ffalse = FheBool::encrypt(false, &keys);

    not_truth_table(&ttrue, &ffalse, &keys);
}

fn xor_truth_table<'a, BoolType>(ttrue: &'a BoolType, ffalse: &'a BoolType, key: &ClientKey)
where
    &'a BoolType: BitXor<&'a BoolType, Output = BoolType>,
    BoolType: FheDecrypt<bool>,
{
    let r = ffalse ^ ffalse;
    assert_eq!(r.decrypt(key), false);

    let r = ffalse ^ ttrue;
    assert_eq!(r.decrypt(key), true);

    let r = ttrue ^ ffalse;
    assert_eq!(r.decrypt(key), true);

    let r = ttrue ^ ttrue;
    assert_eq!(r.decrypt(key), false);
}

fn and_truth_table<'a, BoolType>(ttrue: &'a BoolType, ffalse: &'a BoolType, key: &ClientKey)
where
    &'a BoolType: BitAnd<&'a BoolType, Output = BoolType>,
    BoolType: FheDecrypt<bool>,
{
    let r = ffalse & ffalse;
    assert_eq!(r.decrypt(key), false);

    let r = ffalse & ttrue;
    assert_eq!(r.decrypt(key), false);

    let r = ttrue & ffalse;
    assert_eq!(r.decrypt(key), false);

    let r = ttrue & ttrue;
    assert_eq!(r.decrypt(key), true);
}

fn or_truth_table<'a, BoolType>(ttrue: &'a BoolType, ffalse: &'a BoolType, key: &ClientKey)
where
    &'a BoolType: BitOr<&'a BoolType, Output = BoolType>,
    BoolType: FheDecrypt<bool>,
{
    let r = ffalse | ffalse;
    assert_eq!(r.decrypt(key), false);

    let r = ffalse | ttrue;
    assert_eq!(r.decrypt(key), true);

    let r = ttrue | ffalse;
    assert_eq!(r.decrypt(key), true);

    let r = ttrue | ttrue;
    assert_eq!(r.decrypt(key), true);
}

fn not_truth_table<'a, BoolType>(ttrue: &'a BoolType, ffalse: &'a BoolType, key: &ClientKey)
where
    &'a BoolType: Not<Output = BoolType>,
    BoolType: FheDecrypt<bool>,
{
    let r = !ffalse;
    assert_eq!(r.decrypt(key), true);

    let r = !ttrue;
    assert_eq!(r.decrypt(key), false);
}

#[test]
fn test_compressed_bool() {
    let keys = setup_static_default();

    let cttrue = CompressedFheBool::encrypt(true, &keys);
    let cffalse = CompressedFheBool::encrypt(false, &keys);

    let a = FheBool::from(cttrue);
    let b = FheBool::from(cffalse);

    assert_eq!(a.decrypt(&keys), true);
    assert_eq!(b.decrypt(&keys), false);
}
