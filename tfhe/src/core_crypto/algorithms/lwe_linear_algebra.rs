//! Module containing primitives pertaining to [`LWE ciphertext`](`LweCiphertext`) linear algebra,
//! like addition, multiplication, etc.

use crate::core_crypto::algorithms::misc::*;
use crate::core_crypto::algorithms::slice_algorithms::*;
use crate::core_crypto::commons::numeric::UnsignedInteger;
use crate::core_crypto::commons::parameters::CiphertextModulus;
use crate::core_crypto::commons::traits::*;
use crate::core_crypto::entities::*;

/// Add the right-hand side [`LWE ciphertext`](`LweCiphertext`) to the left-hand side [`LWE
/// ciphertext`](`LweCiphertext`) updating it in-place.
///
/// # Example
///
/// ```
/// use tfhe::core_crypto::prelude::*;
///
/// // DISCLAIMER: these toy example parameters are not guaranteed to be secure or yield correct
/// // computations
/// // Define parameters for LweCiphertext creation
/// let lwe_dimension = LweDimension(742);
/// let lwe_modular_std_dev = StandardDev(0.000007069849454709433);
/// let ciphertext_modulus = CiphertextModulus::new_native();
///
/// // Create the PRNG
/// let mut seeder = new_seeder();
/// let seeder = seeder.as_mut();
/// let mut encryption_generator =
///     EncryptionRandomGenerator::<ActivatedRandomGenerator>::new(seeder.seed(), seeder);
/// let mut secret_generator =
///     SecretRandomGenerator::<ActivatedRandomGenerator>::new(seeder.seed());
///
/// // Create the LweSecretKey
/// let lwe_secret_key =
///     allocate_and_generate_new_binary_lwe_secret_key(lwe_dimension, &mut secret_generator);
///
/// // Create the plaintext
/// let msg = 3u64;
/// let plaintext = Plaintext(msg << 60);
///
/// // Create a new LweCiphertext
/// let mut lwe = allocate_and_encrypt_new_lwe_ciphertext(
///     &lwe_secret_key,
///     plaintext,
///     lwe_modular_std_dev,
///     ciphertext_modulus,
///     &mut encryption_generator,
/// );
///
/// let rhs = lwe.clone();
///
/// lwe_ciphertext_add_assign(&mut lwe, &rhs);
///
/// let decrypted_plaintext = decrypt_lwe_ciphertext(&lwe_secret_key, &lwe);
///
/// // Round and remove encoding
/// // First create a decomposer working on the high 4 bits corresponding to our encoding.
/// let decomposer = SignedDecomposer::new(DecompositionBaseLog(4), DecompositionLevelCount(1));
///
/// let rounded = decomposer.closest_representable(decrypted_plaintext.0);
///
/// // Remove the encoding
/// let cleartext = rounded >> 60;
///
/// // Check we recovered the expected result
/// assert_eq!(cleartext, msg + msg);
/// ```
pub fn lwe_ciphertext_add_assign<Scalar, LhsCont, RhsCont>(
    lhs: &mut LweCiphertext<LhsCont>,
    rhs: &LweCiphertext<RhsCont>,
) where
    Scalar: UnsignedInteger,
    LhsCont: ContainerMut<Element = Scalar>,
    RhsCont: Container<Element = Scalar>,
{
    assert_eq!(
        lhs.ciphertext_modulus(),
        rhs.ciphertext_modulus(),
        "Mismatched moduli between lhs ({:?}) and rhs ({:?}) LweCiphertext",
        lhs.ciphertext_modulus(),
        rhs.ciphertext_modulus()
    );

    let ciphertext_modulus = lhs.ciphertext_modulus();

    if ciphertext_modulus.is_native_modulus() {
        slice_wrapping_add_assign(lhs.as_mut(), rhs.as_ref());
    } else {
        let mut ct_128_lhs =
            LweCiphertext::new(0u128, lhs.lwe_size(), CiphertextModulus::new_native());

        copy_from_convert(&mut ct_128_lhs, lhs);

        let mut ct_128_rhs =
            LweCiphertext::new(0u128, rhs.lwe_size(), CiphertextModulus::new_native());

        copy_from_convert(&mut ct_128_rhs, rhs);

        slice_wrapping_add_assign(ct_128_lhs.as_mut(), ct_128_rhs.as_ref());

        slice_wrapping_rem_assign(ct_128_lhs.as_mut(), ciphertext_modulus.get());

        copy_from_convert(lhs, &ct_128_lhs);
    }
}

/// Add the right-hand side [`LWE ciphertext`](`LweCiphertext`) to the left-hand side [`LWE
/// ciphertext`](`LweCiphertext`) writing the result in the output [`LWE
/// ciphertext`](`LweCiphertext`).
///
/// # Example
///
/// ```
/// use tfhe::core_crypto::prelude::*;
///
/// // DISCLAIMER: these toy example parameters are not guaranteed to be secure or yield correct
/// // computations
/// // Define parameters for LweCiphertext creation
/// let lwe_dimension = LweDimension(742);
/// let lwe_modular_std_dev = StandardDev(0.000007069849454709433);
/// let ciphertext_modulus = CiphertextModulus::new_native();
///
/// // Create the PRNG
/// let mut seeder = new_seeder();
/// let seeder = seeder.as_mut();
/// let mut encryption_generator =
///     EncryptionRandomGenerator::<ActivatedRandomGenerator>::new(seeder.seed(), seeder);
/// let mut secret_generator =
///     SecretRandomGenerator::<ActivatedRandomGenerator>::new(seeder.seed());
///
/// // Create the LweSecretKey
/// let lwe_secret_key =
///     allocate_and_generate_new_binary_lwe_secret_key(lwe_dimension, &mut secret_generator);
///
/// // Create the plaintext
/// let msg = 3u64;
/// let plaintext = Plaintext(msg << 60);
///
/// // Create a new LweCiphertext
/// let lwe = allocate_and_encrypt_new_lwe_ciphertext(
///     &lwe_secret_key,
///     plaintext,
///     lwe_modular_std_dev,
///     ciphertext_modulus,
///     &mut encryption_generator,
/// );
///
/// let rhs = lwe.clone();
///
/// let mut output = lwe.clone();
///
/// lwe_ciphertext_add(&mut output, &lwe, &rhs);
///
/// let decrypted_plaintext = decrypt_lwe_ciphertext(&lwe_secret_key, &output);
///
/// // Round and remove encoding
/// // First create a decomposer working on the high 4 bits corresponding to our encoding.
/// let decomposer = SignedDecomposer::new(DecompositionBaseLog(4), DecompositionLevelCount(1));
///
/// let rounded = decomposer.closest_representable(decrypted_plaintext.0);
///
/// // Remove the encoding
/// let cleartext = rounded >> 60;
///
/// // Check we recovered the expected result
/// assert_eq!(cleartext, msg + msg);
/// ```
pub fn lwe_ciphertext_add<Scalar, OutputCont, LhsCont, RhsCont>(
    output: &mut LweCiphertext<OutputCont>,
    lhs: &LweCiphertext<LhsCont>,
    rhs: &LweCiphertext<RhsCont>,
) where
    Scalar: UnsignedInteger,
    OutputCont: ContainerMut<Element = Scalar>,
    LhsCont: Container<Element = Scalar>,
    RhsCont: Container<Element = Scalar>,
{
    assert_eq!(
        lhs.ciphertext_modulus(),
        rhs.ciphertext_modulus(),
        "Mismatched moduli between lhs ({:?}) and rhs ({:?}) LweCiphertext",
        lhs.ciphertext_modulus(),
        rhs.ciphertext_modulus()
    );

    assert_eq!(
        output.ciphertext_modulus(),
        rhs.ciphertext_modulus(),
        "Mismatched moduli between output ({:?}) and rhs ({:?}) LweCiphertext",
        output.ciphertext_modulus(),
        rhs.ciphertext_modulus()
    );

    let ciphertext_modulus = output.ciphertext_modulus();

    if ciphertext_modulus.is_native_modulus() {
        slice_wrapping_add(output.as_mut(), lhs.as_ref(), rhs.as_ref());
    } else {
        let mut ct_128_output =
            LweCiphertext::new(0u128, output.lwe_size(), CiphertextModulus::new_native());

        let mut ct_128_lhs =
            LweCiphertext::new(0u128, lhs.lwe_size(), CiphertextModulus::new_native());

        copy_from_convert(&mut ct_128_lhs, lhs);

        let mut ct_128_rhs =
            LweCiphertext::new(0u128, rhs.lwe_size(), CiphertextModulus::new_native());

        copy_from_convert(&mut ct_128_rhs, rhs);

        slice_wrapping_add(
            ct_128_output.as_mut(),
            ct_128_lhs.as_ref(),
            ct_128_rhs.as_ref(),
        );

        slice_wrapping_rem_assign(ct_128_output.as_mut(), ciphertext_modulus.get());

        copy_from_convert(output, &ct_128_output);
    }
}

/// Add the right-hand side encoded [`Plaintext`] to the left-hand side [`LWE
/// ciphertext`](`LweCiphertext`) updating it in-place.
///
/// # Example
///
/// ```
/// use tfhe::core_crypto::prelude::*;
///
/// // DISCLAIMER: these toy example parameters are not guaranteed to be secure or yield correct
/// // computations
/// // Define parameters for LweCiphertext creation
/// let lwe_dimension = LweDimension(742);
/// let lwe_modular_std_dev = StandardDev(0.000007069849454709433);
/// let ciphertext_modulus = CiphertextModulus::new_native();
///
/// // Create the PRNG
/// let mut seeder = new_seeder();
/// let seeder = seeder.as_mut();
/// let mut encryption_generator =
///     EncryptionRandomGenerator::<ActivatedRandomGenerator>::new(seeder.seed(), seeder);
/// let mut secret_generator =
///     SecretRandomGenerator::<ActivatedRandomGenerator>::new(seeder.seed());
///
/// // Create the LweSecretKey
/// let lwe_secret_key =
///     allocate_and_generate_new_binary_lwe_secret_key(lwe_dimension, &mut secret_generator);
///
/// // Create the plaintext
/// let msg = 3u64;
/// let plaintext = Plaintext(msg << 60);
///
/// // Create a new LweCiphertext
/// let mut lwe = allocate_and_encrypt_new_lwe_ciphertext(
///     &lwe_secret_key,
///     plaintext,
///     lwe_modular_std_dev,
///     ciphertext_modulus,
///     &mut encryption_generator,
/// );
///
/// lwe_ciphertext_plaintext_add_assign(&mut lwe, plaintext);
///
/// let decrypted_plaintext = decrypt_lwe_ciphertext(&lwe_secret_key, &lwe);
///
/// // Round and remove encoding
/// // First create a decomposer working on the high 4 bits corresponding to our encoding.
/// let decomposer = SignedDecomposer::new(DecompositionBaseLog(4), DecompositionLevelCount(1));
///
/// let rounded = decomposer.closest_representable(decrypted_plaintext.0);
///
/// // Remove the encoding
/// let cleartext = rounded >> 60;
///
/// // Check we recovered the expected result
/// assert_eq!(cleartext, msg + msg);
/// ```
pub fn lwe_ciphertext_plaintext_add_assign<Scalar, InCont>(
    lhs: &mut LweCiphertext<InCont>,
    rhs: Plaintext<Scalar>,
) where
    Scalar: UnsignedInteger,
    InCont: ContainerMut<Element = Scalar>,
{
    let ciphertext_modulus = lhs.ciphertext_modulus();
    let body = lhs.get_mut_body();

    if ciphertext_modulus.is_native_modulus() {
        *body.data = (*body.data).wrapping_add(rhs.0);
    } else {
        let body_128: u128 = (*body.data).cast_into();
        (*body.data) = body_128
            .wrapping_add(rhs.0.cast_into())
            .wrapping_rem(ciphertext_modulus.get())
            .cast_into()
    }
}

/// Compute the opposite of the input [`LWE ciphertext`](`LweCiphertext`) and update it in place.
///
/// # Example
///
/// ```
/// use tfhe::core_crypto::prelude::*;
///
/// // DISCLAIMER: these toy example parameters are not guaranteed to be secure or yield correct
/// // computations
/// // Define parameters for LweCiphertext creation
/// let lwe_dimension = LweDimension(742);
/// let lwe_modular_std_dev = StandardDev(0.000007069849454709433);
/// let ciphertext_modulus = CiphertextModulus::new_native();
///
/// // Create the PRNG
/// let mut seeder = new_seeder();
/// let seeder = seeder.as_mut();
/// let mut encryption_generator =
///     EncryptionRandomGenerator::<ActivatedRandomGenerator>::new(seeder.seed(), seeder);
/// let mut secret_generator =
///     SecretRandomGenerator::<ActivatedRandomGenerator>::new(seeder.seed());
///
/// // Create the LweSecretKey
/// let lwe_secret_key =
///     allocate_and_generate_new_binary_lwe_secret_key(lwe_dimension, &mut secret_generator);
///
/// // Create the plaintext
/// let msg = 3u64;
/// let plaintext = Plaintext(msg << 60);
///
/// // Create a new LweCiphertext
/// let mut lwe = allocate_and_encrypt_new_lwe_ciphertext(
///     &lwe_secret_key,
///     plaintext,
///     lwe_modular_std_dev,
///     ciphertext_modulus,
///     &mut encryption_generator,
/// );
///
/// lwe_ciphertext_opposite_assign(&mut lwe);
///
/// let decrypted_plaintext = decrypt_lwe_ciphertext(&lwe_secret_key, &lwe);
///
/// // Round and remove encoding
/// // First create a decomposer working on the high 4 bits corresponding to our encoding.
/// let decomposer = SignedDecomposer::new(DecompositionBaseLog(4), DecompositionLevelCount(1));
///
/// let rounded = decomposer.closest_representable(decrypted_plaintext.0);
///
/// // Remove the encoding
/// let cleartext = rounded >> 60;
///
/// // Check we recovered the expected result
/// assert_eq!(cleartext, msg.wrapping_neg() % (1 << 4));
/// ```
pub fn lwe_ciphertext_opposite_assign<Scalar, InCont>(ct: &mut LweCiphertext<InCont>)
where
    Scalar: UnsignedInteger,
    InCont: ContainerMut<Element = Scalar>,
{
    assert!(
        ct.ciphertext_modulus().is_native_modulus(),
        "This operation only supports native moduli"
    );
    slice_wrapping_opposite_assign(ct.as_mut());
}

/// Mulitply the left-hand side [`LWE ciphertext`](`LweCiphertext`) by the right-hand side cleartext
/// updating it in-place.
///
/// # Example
///
/// ```
/// use tfhe::core_crypto::prelude::*;
///
/// // DISCLAIMER: these toy example parameters are not guaranteed to be secure or yield correct
/// // computations
/// // Define parameters for LweCiphertext creation
/// let lwe_dimension = LweDimension(742);
/// let lwe_modular_std_dev = StandardDev(0.000007069849454709433);
/// let ciphertext_modulus = CiphertextModulus::new_native();
///
/// // Create the PRNG
/// let mut seeder = new_seeder();
/// let seeder = seeder.as_mut();
/// let mut encryption_generator =
///     EncryptionRandomGenerator::<ActivatedRandomGenerator>::new(seeder.seed(), seeder);
/// let mut secret_generator =
///     SecretRandomGenerator::<ActivatedRandomGenerator>::new(seeder.seed());
///
/// // Create the LweSecretKey
/// let lwe_secret_key =
///     allocate_and_generate_new_binary_lwe_secret_key(lwe_dimension, &mut secret_generator);
///
/// // Create the plaintext
/// let msg = 3u64;
/// let plaintext = Plaintext(msg << 60);
/// let mul_cleartext = 2;
///
/// // Create a new LweCiphertext
/// let mut lwe = allocate_and_encrypt_new_lwe_ciphertext(
///     &lwe_secret_key,
///     plaintext,
///     lwe_modular_std_dev,
///     ciphertext_modulus,
///     &mut encryption_generator,
/// );
///
/// lwe_ciphertext_cleartext_mul_assign(&mut lwe, Cleartext(mul_cleartext));
///
/// let decrypted_plaintext = decrypt_lwe_ciphertext(&lwe_secret_key, &lwe);
///
/// // Round and remove encoding
/// // First create a decomposer working on the high 4 bits corresponding to our encoding.
/// let decomposer = SignedDecomposer::new(DecompositionBaseLog(4), DecompositionLevelCount(1));
///
/// let rounded = decomposer.closest_representable(decrypted_plaintext.0);
///
/// // Remove the encoding
/// let cleartext = rounded >> 60;
///
/// // Check we recovered the expected result
/// assert_eq!(cleartext, msg * mul_cleartext);
/// ```
pub fn lwe_ciphertext_cleartext_mul_assign<Scalar, InCont>(
    lhs: &mut LweCiphertext<InCont>,
    rhs: Cleartext<Scalar>,
) where
    Scalar: UnsignedInteger,
    InCont: ContainerMut<Element = Scalar>,
{
    assert!(
        lhs.ciphertext_modulus().is_native_modulus(),
        "This operation only supports native moduli"
    );
    slice_wrapping_scalar_mul_assign(lhs.as_mut(), rhs.0);
}

/// Subtract the right-hand side [`LWE ciphertext`](`LweCiphertext`) to the left-hand side [`LWE
/// ciphertext`](`LweCiphertext`) updating it in-place.
///
/// # Example
///
/// ```
/// use tfhe::core_crypto::prelude::*;
///
/// // DISCLAIMER: these toy example parameters are not guaranteed to be secure or yield correct
/// // computations
/// // Define parameters for LweCiphertext creation
/// let lwe_dimension = LweDimension(742);
/// let lwe_modular_std_dev = StandardDev(0.000007069849454709433);
/// let ciphertext_modulus = CiphertextModulus::new_native();
///
/// // Create the PRNG
/// let mut seeder = new_seeder();
/// let seeder = seeder.as_mut();
/// let mut encryption_generator =
///     EncryptionRandomGenerator::<ActivatedRandomGenerator>::new(seeder.seed(), seeder);
/// let mut secret_generator =
///     SecretRandomGenerator::<ActivatedRandomGenerator>::new(seeder.seed());
///
/// // Create the LweSecretKey
/// let lwe_secret_key =
///     allocate_and_generate_new_binary_lwe_secret_key(lwe_dimension, &mut secret_generator);
///
/// // Create the plaintext
/// let msg = 3u64;
/// let plaintext = Plaintext(msg << 60);
///
/// // Create a new LweCiphertext
/// let mut lwe = allocate_and_encrypt_new_lwe_ciphertext(
///     &lwe_secret_key,
///     plaintext,
///     lwe_modular_std_dev,
///     ciphertext_modulus,
///     &mut encryption_generator,
/// );
///
/// let rhs = lwe.clone();
///
/// lwe_ciphertext_sub_assign(&mut lwe, &rhs);
///
/// let decrypted_plaintext = decrypt_lwe_ciphertext(&lwe_secret_key, &lwe);
///
/// // Round and remove encoding
/// // First create a decomposer working on the high 4 bits corresponding to our encoding.
/// let decomposer = SignedDecomposer::new(DecompositionBaseLog(4), DecompositionLevelCount(1));
///
/// let rounded = decomposer.closest_representable(decrypted_plaintext.0);
///
/// // Remove the encoding
/// let cleartext = rounded >> 60;
///
/// // Check we recovered the expected result
/// assert_eq!(cleartext, msg - msg);
/// ```
pub fn lwe_ciphertext_sub_assign<Scalar, LhsCont, RhsCont>(
    lhs: &mut LweCiphertext<LhsCont>,
    rhs: &LweCiphertext<RhsCont>,
) where
    Scalar: UnsignedInteger,
    LhsCont: ContainerMut<Element = Scalar>,
    RhsCont: Container<Element = Scalar>,
{
    assert!(
        lhs.ciphertext_modulus().is_native_modulus(),
        "This operation only supports native moduli"
    );
    slice_wrapping_sub_assign(lhs.as_mut(), rhs.as_ref());
}

/// Subtract the right-hand side [`LWE ciphertext`](`LweCiphertext`) to the left-hand side [`LWE
/// ciphertext`](`LweCiphertext`) writing the result in the output [`LWE
/// ciphertext`](`LweCiphertext`).
///
/// # Example
///
/// ```
/// use tfhe::core_crypto::prelude::*;
///
/// // DISCLAIMER: these toy example parameters are not guaranteed to be secure or yield correct
/// // computations
/// // Define parameters for LweCiphertext creation
/// let lwe_dimension = LweDimension(742);
/// let lwe_modular_std_dev = StandardDev(0.000007069849454709433);
/// let ciphertext_modulus = CiphertextModulus::new_native();
///
/// // Create the PRNG
/// let mut seeder = new_seeder();
/// let seeder = seeder.as_mut();
/// let mut encryption_generator =
///     EncryptionRandomGenerator::<ActivatedRandomGenerator>::new(seeder.seed(), seeder);
/// let mut secret_generator =
///     SecretRandomGenerator::<ActivatedRandomGenerator>::new(seeder.seed());
///
/// // Create the LweSecretKey
/// let lwe_secret_key =
///     allocate_and_generate_new_binary_lwe_secret_key(lwe_dimension, &mut secret_generator);
///
/// // Create the plaintext
/// let msg1 = 3u64;
/// let msg2 = 2u64;
/// let plaintext1 = Plaintext(msg1 << 60);
/// let plaintext2 = Plaintext(msg2 << 60);
///
/// // Create a new LweCiphertext
/// let lwe1 = allocate_and_encrypt_new_lwe_ciphertext(
///     &lwe_secret_key,
///     plaintext1,
///     lwe_modular_std_dev,
///     ciphertext_modulus,
///     &mut encryption_generator,
/// );
/// let lwe2 = allocate_and_encrypt_new_lwe_ciphertext(
///     &lwe_secret_key,
///     plaintext2,
///     lwe_modular_std_dev,
///     ciphertext_modulus,
///     &mut encryption_generator,
/// );
///
/// let mut output = lwe1.clone();
///
/// lwe_ciphertext_sub(&mut output, &lwe1, &lwe2);
///
/// let decrypted_plaintext = decrypt_lwe_ciphertext(&lwe_secret_key, &output);
///
/// // Round and remove encoding
/// // First create a decomposer working on the high 4 bits corresponding to our encoding.
/// let decomposer = SignedDecomposer::new(DecompositionBaseLog(4), DecompositionLevelCount(1));
///
/// let rounded = decomposer.closest_representable(decrypted_plaintext.0);
///
/// // Remove the encoding
/// let cleartext = rounded >> 60;
///
/// // Check we recovered the expected result
/// assert_eq!(cleartext, msg1 - msg2);
/// ```
pub fn lwe_ciphertext_sub<Scalar, OutputCont, LhsCont, RhsCont>(
    output: &mut LweCiphertext<OutputCont>,
    lhs: &LweCiphertext<LhsCont>,
    rhs: &LweCiphertext<RhsCont>,
) where
    Scalar: UnsignedInteger,
    OutputCont: ContainerMut<Element = Scalar>,
    LhsCont: Container<Element = Scalar>,
    RhsCont: Container<Element = Scalar>,
{
    slice_wrapping_sub(output.as_mut(), lhs.as_ref(), rhs.as_ref());
}

/// Mulitply the left-hand side [`LWE ciphertext`](`LweCiphertext`) by the right-hand side cleartext
/// writing the result in the output [`LWE ciphertext`](`LweCiphertext`).
///
/// # Example
///
/// ```
/// use tfhe::core_crypto::prelude::*;
///
/// // DISCLAIMER: these toy example parameters are not guaranteed to be secure or yield correct
/// // computations
/// // Define parameters for LweCiphertext creation
/// let lwe_dimension = LweDimension(742);
/// let lwe_modular_std_dev = StandardDev(0.000007069849454709433);
/// let ciphertext_modulus = CiphertextModulus::new_native();
///
/// // Create the PRNG
/// let mut seeder = new_seeder();
/// let seeder = seeder.as_mut();
/// let mut encryption_generator =
///     EncryptionRandomGenerator::<ActivatedRandomGenerator>::new(seeder.seed(), seeder);
/// let mut secret_generator =
///     SecretRandomGenerator::<ActivatedRandomGenerator>::new(seeder.seed());
///
/// // Create the LweSecretKey
/// let lwe_secret_key =
///     allocate_and_generate_new_binary_lwe_secret_key(lwe_dimension, &mut secret_generator);
///
/// // Create the plaintext
/// let msg = 3u64;
/// let plaintext = Plaintext(msg << 60);
/// let mul_cleartext = 2;
///
/// // Create a new LweCiphertext
/// let lwe = allocate_and_encrypt_new_lwe_ciphertext(
///     &lwe_secret_key,
///     plaintext,
///     lwe_modular_std_dev,
///     ciphertext_modulus,
///     &mut encryption_generator,
/// );
///
/// let mut output = lwe.clone();
///
/// lwe_ciphertext_cleartext_mul(&mut output, &lwe, Cleartext(mul_cleartext));
///
/// let decrypted_plaintext = decrypt_lwe_ciphertext(&lwe_secret_key, &output);
///
/// // Round and remove encoding
/// // First create a decomposer working on the high 4 bits corresponding to our encoding.
/// let decomposer = SignedDecomposer::new(DecompositionBaseLog(4), DecompositionLevelCount(1));
///
/// let rounded = decomposer.closest_representable(decrypted_plaintext.0);
///
/// // Remove the encoding
/// let cleartext = rounded >> 60;
///
/// // Check we recovered the expected result
/// assert_eq!(cleartext, msg * mul_cleartext);
/// ```
pub fn lwe_ciphertext_cleartext_mul<Scalar, InputCont, OutputCont>(
    output: &mut LweCiphertext<OutputCont>,
    lhs: &LweCiphertext<InputCont>,
    rhs: Cleartext<Scalar>,
) where
    Scalar: UnsignedInteger,
    InputCont: Container<Element = Scalar>,
    OutputCont: ContainerMut<Element = Scalar>,
{
    assert!(
        output.ciphertext_modulus().is_native_modulus(),
        "This operation only supports native moduli"
    );
    output.as_mut().copy_from_slice(lhs.as_ref());
    lwe_ciphertext_cleartext_mul_assign(output, rhs);
}
