use crate::c_api::high_level_api::keys::{ClientKey, PublicKey};
use crate::high_level_api::prelude::*;
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, MulAssign,
    Neg, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

use crate::c_api::high_level_api::u256::U256;
use crate::c_api::utils::*;
use std::os::raw::c_int;

/// Implement C functions for all the operations supported by a integer type,
/// which should also be accessible from C API
macro_rules! impl_operations_for_integer_type {
    (
        name: $name:ident,
        clear_scalar_type: $clear_scalar_type:ty
    ) => {
        impl_binary_fn_on_type_mut!($name => add, sub, mul, bitand, bitor, bitxor, eq, ge, gt, le, lt, min, max);
        impl_binary_assign_fn_on_type_mut!($name => add_assign, sub_assign, mul_assign, bitand_assign, bitor_assign, bitxor_assign);
        impl_scalar_binary_fn_on_type_mut!($name, $clear_scalar_type => add, sub, mul, shl, shr);
        impl_scalar_binary_assign_fn_on_type_mut!($name, $clear_scalar_type => add_assign, sub_assign, mul_assign, shl_assign, shr_assign);

        impl_unary_fn_on_type_mut!($name => neg);
    };
}

/// Creates a type that will act as an opaque wrapper
/// aroung a tfhe integer.
///
/// It also implements binary operations for this wrapper type
macro_rules! create_integer_wrapper_type {
    (
        name: $name:ident,
        clear_scalar_type: $clear_scalar_type:ty
    ) => {
        pub struct $name($crate::high_level_api::$name);

        impl_destroy_on_type!($name);

        impl_operations_for_integer_type!(name: $name, clear_scalar_type: $clear_scalar_type);

        impl_serialize_deserialize_on_type!($name);

        impl_clone_on_type!($name);

        // The compressed version of the ciphertext type
        ::paste::paste! {
            pub struct [<Compressed $name>]($crate::high_level_api::[<Compressed $name>]);

            impl_destroy_on_type!([<Compressed $name>]);

            impl_clone_on_type!([<Compressed $name>]);

            impl_serialize_deserialize_on_type!([<Compressed $name>]);

            #[no_mangle]
            pub unsafe extern "C" fn [<compressed_ $name:snake _decompress>](
                sself: *const [<Compressed $name>],
                result: *mut *mut $name,
            ) -> ::std::os::raw::c_int {
                $crate::c_api::utils::catch_panic(|| {
                    let compressed = $crate::c_api::utils::get_ref_checked(sself).unwrap();

                    let decompressed_inner = compressed.0.clone().into();
                    *result = Box::into_raw(Box::new($name(decompressed_inner)));
                })
            }
        }
    };
}

create_integer_wrapper_type!(name: FheUint8, clear_scalar_type: u8);
create_integer_wrapper_type!(name: FheUint10, clear_scalar_type: u16);
create_integer_wrapper_type!(name: FheUint12, clear_scalar_type: u16);
create_integer_wrapper_type!(name: FheUint14, clear_scalar_type: u16);
create_integer_wrapper_type!(name: FheUint16, clear_scalar_type: u16);
create_integer_wrapper_type!(name: FheUint32, clear_scalar_type: u32);
create_integer_wrapper_type!(name: FheUint64, clear_scalar_type: u64);
create_integer_wrapper_type!(name: FheUint128, clear_scalar_type: u64);
create_integer_wrapper_type!(name: FheUint256, clear_scalar_type: u64);

impl_decrypt_on_type!(FheUint8, u8);
impl_try_encrypt_with_client_key_on_type!(FheUint8{crate::high_level_api::FheUint8}, u8);
impl_try_encrypt_with_public_key_on_type!(FheUint8{crate::high_level_api::FheUint8}, u8);
impl_try_encrypt_with_client_key_on_type!(CompressedFheUint8{crate::high_level_api::CompressedFheUint8}, u8);

impl_decrypt_on_type!(FheUint10, u16);
impl_try_encrypt_with_client_key_on_type!(FheUint10{crate::high_level_api::FheUint10}, u16);
impl_try_encrypt_with_public_key_on_type!(FheUint10{crate::high_level_api::FheUint10}, u16);
impl_try_encrypt_with_client_key_on_type!(CompressedFheUint10{crate::high_level_api::CompressedFheUint10}, u16);

impl_decrypt_on_type!(FheUint12, u16);
impl_try_encrypt_with_client_key_on_type!(FheUint12{crate::high_level_api::FheUint12}, u16);
impl_try_encrypt_with_public_key_on_type!(FheUint12{crate::high_level_api::FheUint12}, u16);
impl_try_encrypt_with_client_key_on_type!(CompressedFheUint12{crate::high_level_api::CompressedFheUint12}, u16);

impl_decrypt_on_type!(FheUint14, u16);
impl_try_encrypt_with_client_key_on_type!(FheUint14{crate::high_level_api::FheUint14}, u16);
impl_try_encrypt_with_public_key_on_type!(FheUint14{crate::high_level_api::FheUint14}, u16);
impl_try_encrypt_with_client_key_on_type!(CompressedFheUint14{crate::high_level_api::CompressedFheUint14}, u16);

impl_decrypt_on_type!(FheUint16, u16);
impl_try_encrypt_with_client_key_on_type!(FheUint16{crate::high_level_api::FheUint16}, u16);
impl_try_encrypt_with_public_key_on_type!(FheUint16{crate::high_level_api::FheUint16}, u16);
impl_try_encrypt_with_client_key_on_type!(CompressedFheUint16{crate::high_level_api::CompressedFheUint16}, u16);

impl_decrypt_on_type!(FheUint32, u32);
impl_try_encrypt_with_client_key_on_type!(FheUint32{crate::high_level_api::FheUint32}, u32);
impl_try_encrypt_with_public_key_on_type!(FheUint32{crate::high_level_api::FheUint32}, u32);
impl_try_encrypt_with_client_key_on_type!(CompressedFheUint32{crate::high_level_api::CompressedFheUint32}, u32);

impl_decrypt_on_type!(FheUint64, u64);
impl_try_encrypt_with_client_key_on_type!(FheUint64{crate::high_level_api::FheUint64}, u64);
impl_try_encrypt_with_public_key_on_type!(FheUint64{crate::high_level_api::FheUint64}, u64);
impl_try_encrypt_with_client_key_on_type!(CompressedFheUint64{crate::high_level_api::CompressedFheUint64}, u64);

#[no_mangle]
pub unsafe extern "C" fn fhe_uint128_try_encrypt_with_client_key_u128(
    low_word: u64,
    high_word: u64,
    client_key: *const ClientKey,
    result: *mut *mut FheUint128,
) -> c_int {
    catch_panic(|| {
        let client_key = get_ref_checked(client_key).unwrap();

        let value = ((high_word as u128) << 64u128) | low_word as u128;

        let inner = <crate::high_level_api::FheUint128>::try_encrypt(value, &client_key.0).unwrap();

        *result = Box::into_raw(Box::new(FheUint128(inner)));
    })
}

#[no_mangle]
pub unsafe extern "C" fn compressed_fhe_uint128_try_encrypt_with_client_key_u128(
    low_word: u64,
    high_word: u64,
    client_key: *const ClientKey,
    result: *mut *mut CompressedFheUint128,
) -> c_int {
    catch_panic(|| {
        let client_key = get_ref_checked(client_key).unwrap();

        let value = ((high_word as u128) << 64u128) | low_word as u128;

        let inner =
            <crate::high_level_api::CompressedFheUint128>::try_encrypt(value, &client_key.0)
                .unwrap();

        *result = Box::into_raw(Box::new(CompressedFheUint128(inner)));
    })
}

#[no_mangle]
pub unsafe extern "C" fn fhe_uint128_try_encrypt_with_public_key_u128(
    low_word: u64,
    high_word: u64,
    public_key: *const PublicKey,
    result: *mut *mut FheUint128,
) -> c_int {
    catch_panic(|| {
        let public_key = get_ref_checked(public_key).unwrap();

        let value = ((high_word as u128) << 64u128) | low_word as u128;

        let inner = <crate::high_level_api::FheUint128>::try_encrypt(value, &public_key.0).unwrap();

        *result = Box::into_raw(Box::new(FheUint128(inner)));
    })
}

#[no_mangle]
pub unsafe extern "C" fn fhe_uint128_decrypt(
    encrypted_value: *const FheUint128,
    client_key: *const ClientKey,
    low_word: *mut u64,
    high_word: *mut u64,
) -> c_int {
    catch_panic(|| {
        let client_key = get_ref_checked(client_key).unwrap();
        let encrypted_value = get_ref_checked(encrypted_value).unwrap();

        let inner: u128 = encrypted_value.0.decrypt(&client_key.0);

        *low_word = (inner & (u64::MAX as u128)) as u64;
        *high_word = (inner >> 64) as u64;
    })
}

#[no_mangle]
pub unsafe extern "C" fn fhe_uint256_try_encrypt_with_client_key_u256(
    value: *const U256,
    client_key: *const ClientKey,
    result: *mut *mut FheUint256,
) -> c_int {
    catch_panic(|| {
        let client_key = get_ref_checked(client_key).unwrap();

        let inner =
            <crate::high_level_api::FheUint256>::try_encrypt((*value).0, &client_key.0).unwrap();

        *result = Box::into_raw(Box::new(FheUint256(inner)));
    })
}

#[no_mangle]
pub unsafe extern "C" fn compressed_fhe_uint256_try_encrypt_with_client_key_u256(
    value: *const U256,
    client_key: *const ClientKey,
    result: *mut *mut CompressedFheUint256,
) -> c_int {
    catch_panic(|| {
        let client_key = get_ref_checked(client_key).unwrap();

        let inner =
            <crate::high_level_api::CompressedFheUint256>::try_encrypt((*value).0, &client_key.0)
                .unwrap();

        *result = Box::into_raw(Box::new(CompressedFheUint256(inner)));
    })
}

#[no_mangle]
pub unsafe extern "C" fn fhe_uint256_try_encrypt_with_public_key_u256(
    value: *const U256,
    public_key: *const PublicKey,
    result: *mut *mut FheUint256,
) -> c_int {
    catch_panic(|| {
        let public_key = get_ref_checked(public_key).unwrap();

        let inner =
            <crate::high_level_api::FheUint256>::try_encrypt((*value).0, &public_key.0).unwrap();

        *result = Box::into_raw(Box::new(FheUint256(inner)));
    })
}

#[no_mangle]
pub unsafe extern "C" fn fhe_uint256_decrypt(
    encrypted_value: *const FheUint256,
    client_key: *const ClientKey,
    result: *mut *mut U256,
) -> c_int {
    catch_panic(|| {
        let client_key = get_ref_checked(client_key).unwrap();
        let encrypted_value = get_ref_checked(encrypted_value).unwrap();

        let inner: crate::integer::U256 = encrypted_value.0.decrypt(&client_key.0);
        *result = Box::into_raw(Box::new(U256(inner)));
    })
}
