use bincode;
use wasm_bindgen::prelude::*;

use super::js_wasm_seeder;

use std::panic::set_hook;

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) enum ShortintCiphertextInner {
    Big(crate::shortint::ciphertext::CiphertextBig),
    Small(crate::shortint::ciphertext::CiphertextSmall),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) enum ShortintCompressedCiphertextInner {
    Big(crate::shortint::ciphertext::CompressedCiphertextBig),
    Small(crate::shortint::ciphertext::CompressedCiphertextSmall),
}

#[wasm_bindgen]
pub struct ShortintCiphertext(pub(crate) ShortintCiphertextInner);

#[wasm_bindgen]
pub struct ShortintCompressedCiphertext(pub(crate) ShortintCompressedCiphertextInner);

#[wasm_bindgen]
pub struct ShortintClientKey(pub(crate) crate::shortint::ClientKey);

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) enum ShortintPublicKeyInner {
    Big(crate::shortint::PublicKeyBig),
    Small(crate::shortint::PublicKeySmall),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) enum ShortintCompressedPublicKeyInner {
    Big(crate::shortint::CompressedPublicKeyBig),
    Small(crate::shortint::CompressedPublicKeySmall),
}

#[wasm_bindgen]
pub struct ShortintPublicKey(pub(crate) ShortintPublicKeyInner);

#[wasm_bindgen]
pub struct ShortintCompressedPublicKey(pub(crate) ShortintCompressedPublicKeyInner);

#[wasm_bindgen]
pub struct ShortintCompressedServerKey(pub(crate) crate::shortint::CompressedServerKey);

#[wasm_bindgen]
pub struct Shortint {}

#[wasm_bindgen]
pub struct ShortintParameters(pub(crate) crate::shortint::Parameters);

pub const SHORTINT_NATIVE_MODULUS: u64 = 0;

#[wasm_bindgen]
impl Shortint {
    #[wasm_bindgen]
    pub fn get_parameters(
        message_bits: usize,
        carry_bits: usize,
    ) -> Result<ShortintParameters, JsError> {
        set_hook(Box::new(console_error_panic_hook::hook));
        match (message_bits, carry_bits) {
            (1, 0) => Ok(crate::shortint::parameters::PARAM_MESSAGE_1_CARRY_0),
            (1, 1) => Ok(crate::shortint::parameters::PARAM_MESSAGE_1_CARRY_1),
            (2, 0) => Ok(crate::shortint::parameters::PARAM_MESSAGE_2_CARRY_0),
            (1, 2) => Ok(crate::shortint::parameters::PARAM_MESSAGE_1_CARRY_2),
            (2, 1) => Ok(crate::shortint::parameters::PARAM_MESSAGE_2_CARRY_1),
            (3, 0) => Ok(crate::shortint::parameters::PARAM_MESSAGE_3_CARRY_0),
            (1, 3) => Ok(crate::shortint::parameters::PARAM_MESSAGE_1_CARRY_3),
            (2, 2) => Ok(crate::shortint::parameters::PARAM_MESSAGE_2_CARRY_2),
            (3, 1) => Ok(crate::shortint::parameters::PARAM_MESSAGE_3_CARRY_1),
            (4, 0) => Ok(crate::shortint::parameters::PARAM_MESSAGE_4_CARRY_0),
            (1, 4) => Ok(crate::shortint::parameters::PARAM_MESSAGE_1_CARRY_4),
            (2, 3) => Ok(crate::shortint::parameters::PARAM_MESSAGE_2_CARRY_3),
            (3, 2) => Ok(crate::shortint::parameters::PARAM_MESSAGE_3_CARRY_2),
            (4, 1) => Ok(crate::shortint::parameters::PARAM_MESSAGE_4_CARRY_1),
            (5, 0) => Ok(crate::shortint::parameters::PARAM_MESSAGE_5_CARRY_0),
            (1, 5) => Ok(crate::shortint::parameters::PARAM_MESSAGE_1_CARRY_5),
            (2, 4) => Ok(crate::shortint::parameters::PARAM_MESSAGE_2_CARRY_4),
            (3, 3) => Ok(crate::shortint::parameters::PARAM_MESSAGE_3_CARRY_3),
            (4, 2) => Ok(crate::shortint::parameters::PARAM_MESSAGE_4_CARRY_2),
            (5, 1) => Ok(crate::shortint::parameters::PARAM_MESSAGE_5_CARRY_1),
            (6, 0) => Ok(crate::shortint::parameters::PARAM_MESSAGE_6_CARRY_0),
            (1, 6) => Ok(crate::shortint::parameters::PARAM_MESSAGE_1_CARRY_6),
            (2, 5) => Ok(crate::shortint::parameters::PARAM_MESSAGE_2_CARRY_5),
            (3, 4) => Ok(crate::shortint::parameters::PARAM_MESSAGE_3_CARRY_4),
            (4, 3) => Ok(crate::shortint::parameters::PARAM_MESSAGE_4_CARRY_3),
            (5, 2) => Ok(crate::shortint::parameters::PARAM_MESSAGE_5_CARRY_2),
            (6, 1) => Ok(crate::shortint::parameters::PARAM_MESSAGE_6_CARRY_1),
            (7, 0) => Ok(crate::shortint::parameters::PARAM_MESSAGE_7_CARRY_0),
            (1, 7) => Ok(crate::shortint::parameters::PARAM_MESSAGE_1_CARRY_7),
            (2, 6) => Ok(crate::shortint::parameters::PARAM_MESSAGE_2_CARRY_6),
            (3, 5) => Ok(crate::shortint::parameters::PARAM_MESSAGE_3_CARRY_5),
            (4, 4) => Ok(crate::shortint::parameters::PARAM_MESSAGE_4_CARRY_4),
            (5, 3) => Ok(crate::shortint::parameters::PARAM_MESSAGE_5_CARRY_3),
            (6, 2) => Ok(crate::shortint::parameters::PARAM_MESSAGE_6_CARRY_2),
            (7, 1) => Ok(crate::shortint::parameters::PARAM_MESSAGE_7_CARRY_1),
            (8, 0) => Ok(crate::shortint::parameters::PARAM_MESSAGE_8_CARRY_0),
            _ => Err(wasm_bindgen::JsError::new(
                format!(
                "No parameters for {message_bits} bits of message and {carry_bits} bits of carry"
            )
                .as_str(),
            )),
        }
        .map(ShortintParameters)
    }

    #[wasm_bindgen]
    pub fn get_parameters_small(
        message_bits: usize,
        carry_bits: usize,
    ) -> Result<ShortintParameters, JsError> {
        set_hook(Box::new(console_error_panic_hook::hook));
        match (message_bits, carry_bits) {
            (1, 1) => Ok(crate::shortint::parameters::PARAM_SMALL_MESSAGE_1_CARRY_1),
            (2, 2) => Ok(crate::shortint::parameters::PARAM_SMALL_MESSAGE_2_CARRY_2),
            (3, 3) => Ok(crate::shortint::parameters::PARAM_SMALL_MESSAGE_3_CARRY_3),
            (4, 4) => Ok(crate::shortint::parameters::PARAM_SMALL_MESSAGE_4_CARRY_4),
            _ => Err(wasm_bindgen::JsError::new(
                format!(
                "No parameters for {message_bits} bits of message and {carry_bits} bits of carry"
            )
                .as_str(),
            )),
        }
        .map(ShortintParameters)
    }

    #[wasm_bindgen]
    #[allow(clippy::too_many_arguments)]
    pub fn new_parameters(
        lwe_dimension: usize,
        glwe_dimension: usize,
        polynomial_size: usize,
        lwe_modular_std_dev: f64,
        glwe_modular_std_dev: f64,
        pbs_base_log: usize,
        pbs_level: usize,
        ks_base_log: usize,
        ks_level: usize,
        pfks_level: usize,
        pfks_base_log: usize,
        pfks_modular_std_dev: f64,
        cbs_level: usize,
        cbs_base_log: usize,
        message_modulus: usize,
        carry_modulus: usize,
        modulus_power_of_2_exponent: usize,
    ) -> ShortintParameters {
        set_hook(Box::new(console_error_panic_hook::hook));
        use crate::core_crypto::prelude::*;
        ShortintParameters(crate::shortint::Parameters {
            lwe_dimension: LweDimension(lwe_dimension),
            glwe_dimension: GlweDimension(glwe_dimension),
            polynomial_size: PolynomialSize(polynomial_size),
            lwe_modular_std_dev: StandardDev(lwe_modular_std_dev),
            glwe_modular_std_dev: StandardDev(glwe_modular_std_dev),
            pbs_base_log: DecompositionBaseLog(pbs_base_log),
            pbs_level: DecompositionLevelCount(pbs_level),
            ks_base_log: DecompositionBaseLog(ks_base_log),
            ks_level: DecompositionLevelCount(ks_level),
            pfks_level: DecompositionLevelCount(pfks_level),
            pfks_base_log: DecompositionBaseLog(pfks_base_log),
            pfks_modular_std_dev: StandardDev(pfks_modular_std_dev),
            cbs_level: DecompositionLevelCount(cbs_level),
            cbs_base_log: DecompositionBaseLog(cbs_base_log),
            message_modulus: crate::shortint::parameters::MessageModulus(message_modulus),
            carry_modulus: crate::shortint::parameters::CarryModulus(carry_modulus),
            ciphertext_modulus: crate::shortint::parameters::CiphertextModulus::try_new_power_of_2(
                modulus_power_of_2_exponent,
            )
            .unwrap(),
        })
    }

    #[wasm_bindgen]
    pub fn new_client_key_from_seed_and_parameters(
        seed_high_bytes: u64,
        seed_low_bytes: u64,
        parameters: &ShortintParameters,
    ) -> Result<ShortintClientKey, JsError> {
        set_hook(Box::new(console_error_panic_hook::hook));
        let seed_high_bytes: u128 = seed_high_bytes.into();
        let seed_low_bytes: u128 = seed_low_bytes.into();
        let seed: u128 = (seed_high_bytes << 64) | seed_low_bytes;

        let mut constant_seeder = Box::new(js_wasm_seeder::ConstantSeeder::new(
            crate::core_crypto::commons::math::random::Seed(seed),
        ));

        let mut tmp_shortint_engine =
            crate::shortint::engine::ShortintEngine::new_from_seeder(constant_seeder.as_mut());

        tmp_shortint_engine
            .new_client_key(parameters.0.to_owned())
            .map_err(|e| wasm_bindgen::JsError::new(format!("{e:?}").as_str()))
            .map(ShortintClientKey)
    }

    #[wasm_bindgen]
    pub fn new_client_key(parameters: &ShortintParameters) -> ShortintClientKey {
        set_hook(Box::new(console_error_panic_hook::hook));

        ShortintClientKey(crate::shortint::client_key::ClientKey::new(
            parameters.0.to_owned(),
        ))
    }

    #[wasm_bindgen]
    pub fn new_public_key(client_key: &ShortintClientKey) -> ShortintPublicKey {
        set_hook(Box::new(console_error_panic_hook::hook));

        ShortintPublicKey(ShortintPublicKeyInner::Big(
            crate::shortint::public_key::PublicKeyBig::new(&client_key.0),
        ))
    }

    #[wasm_bindgen]
    pub fn new_public_key_small(client_key: &ShortintClientKey) -> ShortintPublicKey {
        set_hook(Box::new(console_error_panic_hook::hook));

        ShortintPublicKey(ShortintPublicKeyInner::Small(
            crate::shortint::public_key::PublicKeySmall::new(&client_key.0),
        ))
    }

    #[wasm_bindgen]
    pub fn new_compressed_public_key(
        client_key: &ShortintClientKey,
    ) -> ShortintCompressedPublicKey {
        set_hook(Box::new(console_error_panic_hook::hook));

        ShortintCompressedPublicKey(ShortintCompressedPublicKeyInner::Big(
            crate::shortint::public_key::CompressedPublicKeyBig::new(&client_key.0),
        ))
    }

    #[wasm_bindgen]
    pub fn new_compressed_public_key_small(
        client_key: &ShortintClientKey,
    ) -> ShortintCompressedPublicKey {
        set_hook(Box::new(console_error_panic_hook::hook));

        ShortintCompressedPublicKey(ShortintCompressedPublicKeyInner::Small(
            crate::shortint::public_key::CompressedPublicKeySmall::new(&client_key.0),
        ))
    }

    #[wasm_bindgen]
    pub fn new_compressed_server_key(
        client_key: &ShortintClientKey,
    ) -> ShortintCompressedServerKey {
        set_hook(Box::new(console_error_panic_hook::hook));

        ShortintCompressedServerKey(crate::shortint::server_key::CompressedServerKey::new(
            &client_key.0,
        ))
    }

    #[wasm_bindgen]
    pub fn encrypt(client_key: &ShortintClientKey, message: u64) -> ShortintCiphertext {
        set_hook(Box::new(console_error_panic_hook::hook));

        ShortintCiphertext(ShortintCiphertextInner::Big(client_key.0.encrypt(message)))
    }

    #[wasm_bindgen]
    pub fn encrypt_small(client_key: &ShortintClientKey, message: u64) -> ShortintCiphertext {
        set_hook(Box::new(console_error_panic_hook::hook));

        ShortintCiphertext(ShortintCiphertextInner::Small(
            client_key.0.encrypt_small(message),
        ))
    }

    #[wasm_bindgen]
    pub fn encrypt_compressed(
        client_key: &ShortintClientKey,
        message: u64,
    ) -> ShortintCompressedCiphertext {
        set_hook(Box::new(console_error_panic_hook::hook));

        ShortintCompressedCiphertext(ShortintCompressedCiphertextInner::Big(
            client_key.0.encrypt_compressed(message),
        ))
    }

    #[wasm_bindgen]
    pub fn encrypt_compressed_small(
        client_key: &ShortintClientKey,
        message: u64,
    ) -> ShortintCompressedCiphertext {
        set_hook(Box::new(console_error_panic_hook::hook));

        ShortintCompressedCiphertext(ShortintCompressedCiphertextInner::Small(
            client_key.0.encrypt_compressed_small(message),
        ))
    }

    #[wasm_bindgen]
    pub fn decompress_ciphertext(
        compressed_ciphertext: &ShortintCompressedCiphertext,
    ) -> ShortintCiphertext {
        set_hook(Box::new(console_error_panic_hook::hook));
        match &compressed_ciphertext.0 {
            ShortintCompressedCiphertextInner::Big(inner) => {
                ShortintCiphertext(ShortintCiphertextInner::Big(inner.clone().into()))
            }
            ShortintCompressedCiphertextInner::Small(inner) => {
                ShortintCiphertext(ShortintCiphertextInner::Small(inner.clone().into()))
            }
        }
    }

    #[wasm_bindgen]
    pub fn encrypt_with_public_key(
        public_key: &ShortintPublicKey,
        message: u64,
    ) -> ShortintCiphertext {
        set_hook(Box::new(console_error_panic_hook::hook));

        match &public_key.0 {
            ShortintPublicKeyInner::Big(inner) => {
                ShortintCiphertext(ShortintCiphertextInner::Big(inner.encrypt(message)))
            }
            ShortintPublicKeyInner::Small(inner) => {
                ShortintCiphertext(ShortintCiphertextInner::Small(inner.encrypt(message)))
            }
        }
    }

    #[wasm_bindgen]
    pub fn encrypt_with_compressed_public_key(
        public_key: &ShortintCompressedPublicKey,
        message: u64,
    ) -> ShortintCiphertext {
        set_hook(Box::new(console_error_panic_hook::hook));

        match &public_key.0 {
            ShortintCompressedPublicKeyInner::Big(inner) => {
                ShortintCiphertext(ShortintCiphertextInner::Big(inner.encrypt(message)))
            }
            ShortintCompressedPublicKeyInner::Small(inner) => {
                ShortintCiphertext(ShortintCiphertextInner::Small(inner.encrypt(message)))
            }
        }
    }

    #[wasm_bindgen]
    pub fn decrypt(client_key: &ShortintClientKey, ct: &ShortintCiphertext) -> u64 {
        set_hook(Box::new(console_error_panic_hook::hook));
        match &ct.0 {
            ShortintCiphertextInner::Big(inner) => client_key.0.decrypt(inner),
            ShortintCiphertextInner::Small(inner) => client_key.0.decrypt(inner),
        }
    }

    #[wasm_bindgen]
    pub fn serialize_ciphertext(ciphertext: &ShortintCiphertext) -> Result<Vec<u8>, JsError> {
        set_hook(Box::new(console_error_panic_hook::hook));
        bincode::serialize(&ciphertext.0)
            .map_err(|e| wasm_bindgen::JsError::new(format!("{e:?}").as_str()))
    }

    #[wasm_bindgen]
    pub fn deserialize_ciphertext(buffer: &[u8]) -> Result<ShortintCiphertext, JsError> {
        set_hook(Box::new(console_error_panic_hook::hook));
        bincode::deserialize(buffer)
            .map_err(|e| wasm_bindgen::JsError::new(format!("{e:?}").as_str()))
            .map(ShortintCiphertext)
    }

    #[wasm_bindgen]
    pub fn serialize_compressed_ciphertext(
        ciphertext: &ShortintCompressedCiphertext,
    ) -> Result<Vec<u8>, JsError> {
        set_hook(Box::new(console_error_panic_hook::hook));
        bincode::serialize(&ciphertext.0)
            .map_err(|e| wasm_bindgen::JsError::new(format!("{e:?}").as_str()))
    }

    #[wasm_bindgen]
    pub fn deserialize_compressed_ciphertext(
        buffer: &[u8],
    ) -> Result<ShortintCompressedCiphertext, JsError> {
        set_hook(Box::new(console_error_panic_hook::hook));
        bincode::deserialize(buffer)
            .map_err(|e| wasm_bindgen::JsError::new(format!("{e:?}").as_str()))
            .map(ShortintCompressedCiphertext)
    }

    #[wasm_bindgen]
    pub fn serialize_client_key(client_key: &ShortintClientKey) -> Result<Vec<u8>, JsError> {
        set_hook(Box::new(console_error_panic_hook::hook));
        bincode::serialize(&client_key.0)
            .map_err(|e| wasm_bindgen::JsError::new(format!("{e:?}").as_str()))
    }

    #[wasm_bindgen]
    pub fn deserialize_client_key(buffer: &[u8]) -> Result<ShortintClientKey, JsError> {
        set_hook(Box::new(console_error_panic_hook::hook));
        bincode::deserialize(buffer)
            .map_err(|e| wasm_bindgen::JsError::new(format!("{e:?}").as_str()))
            .map(ShortintClientKey)
    }

    #[wasm_bindgen]
    pub fn serialize_public_key(public_key: &ShortintPublicKey) -> Result<Vec<u8>, JsError> {
        set_hook(Box::new(console_error_panic_hook::hook));
        bincode::serialize(&public_key.0)
            .map_err(|e| wasm_bindgen::JsError::new(format!("{e:?}").as_str()))
    }

    #[wasm_bindgen]
    pub fn deserialize_public_key(buffer: &[u8]) -> Result<ShortintPublicKey, JsError> {
        set_hook(Box::new(console_error_panic_hook::hook));
        bincode::deserialize(buffer)
            .map_err(|e| wasm_bindgen::JsError::new(format!("{e:?}").as_str()))
            .map(ShortintPublicKey)
    }

    #[wasm_bindgen]
    pub fn serialize_compressed_public_key(
        public_key: &ShortintCompressedPublicKey,
    ) -> Result<Vec<u8>, JsError> {
        set_hook(Box::new(console_error_panic_hook::hook));
        bincode::serialize(&public_key.0)
            .map_err(|e| wasm_bindgen::JsError::new(format!("{e:?}").as_str()))
    }

    #[wasm_bindgen]
    pub fn deserialize_compressed_public_key(
        buffer: &[u8],
    ) -> Result<ShortintCompressedPublicKey, JsError> {
        set_hook(Box::new(console_error_panic_hook::hook));
        bincode::deserialize(buffer)
            .map_err(|e| wasm_bindgen::JsError::new(format!("{e:?}").as_str()))
            .map(ShortintCompressedPublicKey)
    }

    #[wasm_bindgen]
    pub fn serialize_compressed_server_key(
        server_key: &ShortintCompressedServerKey,
    ) -> Result<Vec<u8>, JsError> {
        set_hook(Box::new(console_error_panic_hook::hook));
        bincode::serialize(&server_key.0)
            .map_err(|e| wasm_bindgen::JsError::new(format!("{e:?}").as_str()))
    }

    #[wasm_bindgen]
    pub fn deserialize_compressed_server_key(
        buffer: &[u8],
    ) -> Result<ShortintCompressedServerKey, JsError> {
        set_hook(Box::new(console_error_panic_hook::hook));
        bincode::deserialize(buffer)
            .map_err(|e| wasm_bindgen::JsError::new(format!("{e:?}").as_str()))
            .map(ShortintCompressedServerKey)
    }
}
