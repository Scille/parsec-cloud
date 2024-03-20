// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use hex_literal::hex;
use rstest::rstest;

use libparsec_crypto::{
    CryptoError, SequesterKeySize, SequesterPrivateKeyDer, SequesterPublicKeyDer,
    SequesterSigningKeyDer, SequesterVerifyKeyDer,
};

/// Generated with `openssl genrsa -out sequester-privkey-1024.pem 1024`
const PRIVATE_KEY_PEM_1024: &str = std::include_str!("./files/sequester-privkey-1024.pem");

/// Generated with `openssl rsa -in sequester-privkey-1024.pem -outform der -out sequester-privkey-1024.der`.
const PRIVATE_KEY_DER_1024: &[u8] = std::include_bytes!("./files/sequester-privkey-1024.der");

/// Generated with `openssl rsa -in sequester-privkey-1024.pem -pubout -out sequester-pubkey-1024.pem`
const PUBLIC_KEY_PEM_1024: &str = std::include_str!("./files/sequester-pubkey-1024.pem");

/// Generated with `openssl rsa -in sequester-privkey-1024.pem -pubout -outform der -out sequester-pubkey-1024.der`
const PUBLIC_KEY_DER_1024: &[u8] = std::include_bytes!("./files/sequester-pubkey-1024.der");

/// Generated with `openssl genrsa -out sequester-privkey-2048.pem 2048`
const PRIVATE_KEY_PEM_2048: &str = std::include_str!("./files/sequester-privkey-2048.pem");

/// Generated with `openssl rsa -in sequester-privkey-2048.pem -outform der -out sequester-privkey-2048.der`
const PRIVATE_KEY_DER_2048: &[u8] = std::include_bytes!("./files/sequester-privkey-2048.der");

/// Generated with `openssl rsa -in sequester-privkey-2048.pem -pubout -out sequester-pubkey-2048.pem`
const PUBLIC_KEY_PEM_2048: &str = std::include_str!("./files/sequester-pubkey-2048.pem");

/// Generated with `openssl rsa -in sequester-privkey-2048.pem -pubout -outform der -out sequester-pubkey-2048.der`
const PUBLIC_KEY_DER_2048: &[u8] = std::include_bytes!("./files/sequester-pubkey-2048.der");

#[test]
fn only_rsa_is_supported() {
    let unsupported_key: &[u8] = &hex!(
        "308201b73082012b06072a8648ce3804013082011e02818100f8df308877a3bcf66db78734"
        "882f854f934cb5e3bc3cd284b62751d2b6490fd0793dfd43e68393c5fe02f897236fe40949"
        "892aa7718939be9aec22e813c7ab55a245b0b3d2cef69123c862c2449af305ce30784929a3"
        "ba4ac3e5f14cf50508558ee9600c74b5ff1020fe3728ff92c303292dcf13c54aac726ba462"
        "bfee65d3b302150086d038ac40999b7fb3cb7d6253f9ff71a932ca57028180517dea95129a"
        "3e46e7aee51e00fd4eb0bc90eec8c340fe27dc8e72287ebddbdb7c748b67d7416a54b12a1b"
        "d09248d1b6e9291a7d266c02fcf76c887a710065e0fdc767e74dfa13edb5d1a8bca331dd32"
        "dccc199cc2055b446b35a30bd6edde35d08cfbbfd2fbbda2fecce75d4ab5eab2c772cbf914"
        "1325e0e6e05348b035b3ff03818500028181008fb6c7096b0f95aa2baa71e32ba1c1f54088"
        "59f573a4786c55fe050e29180a60b6487885c18a294d1f4655032c1fc3c88eb113dae0cddb"
        "cd5faa40a312dc339256e019aeffd2410c71ef00bf2c519444f3d43024adfc55be6e5db2b7"
        "8e7d6c54c6cc882ffa606e76e53fc73b3443347507e91227944aeed42004a305de09918b");

    assert!(SequesterPublicKeyDer::try_from(unsupported_key).is_err());
    assert!(SequesterVerifyKeyDer::try_from(unsupported_key).is_err());
}

#[rstest]
#[case(PUBLIC_KEY_DER_1024)]
#[case(PUBLIC_KEY_DER_2048)]
fn key_equality(#[case] raw_pub_key: &[u8]) {
    let pub_key = SequesterPublicKeyDer::try_from(raw_pub_key).unwrap();
    let pub_key2 = SequesterPublicKeyDer::try_from(raw_pub_key).unwrap();

    assert_eq!(pub_key, pub_key2);
}

#[rstest]
#[case(SequesterKeySize::_1024Bits)]
#[case(SequesterKeySize::_2048Bits)]
fn size(#[case] size_in_bits: SequesterKeySize) {
    let (priv_key, pub_key) = SequesterPrivateKeyDer::generate_pair(size_in_bits);
    let (signing_key, verify_key) = SequesterSigningKeyDer::generate_pair(size_in_bits);
    assert_eq!(priv_key.size_in_bytes(), size_in_bits as usize / 8);
    assert_eq!(pub_key.size_in_bytes(), size_in_bits as usize / 8);
    assert_eq!(signing_key.size_in_bytes(), size_in_bits as usize / 8);
    assert_eq!(verify_key.size_in_bytes(), size_in_bits as usize / 8);
}

#[rstest]
#[case(SequesterKeySize::_1024Bits)]
#[case(SequesterKeySize::_2048Bits)]
fn encrypt_decrypt(#[case] size_in_bits: SequesterKeySize) {
    let (priv_key, pub_key) = SequesterPrivateKeyDer::generate_pair(size_in_bits);

    let encrypted = pub_key.encrypt(b"foo");

    assert_eq!(priv_key.decrypt(&encrypted).unwrap(), b"foo");
}

#[rstest]
#[case(SequesterKeySize::_1024Bits)]
#[case(SequesterKeySize::_2048Bits)]
fn sign_verify(#[case] size_in_bits: SequesterKeySize) {
    let (signing_key, verify_key) = SequesterSigningKeyDer::generate_pair(size_in_bits);

    let signed = signing_key.sign(b"foo");

    assert_eq!(verify_key.verify(&signed).unwrap(), b"foo");
}

#[rstest]
#[case(
    PRIVATE_KEY_PEM_1024,
    PRIVATE_KEY_DER_1024,
    PUBLIC_KEY_PEM_1024,
    PUBLIC_KEY_DER_1024
)]
#[case(
    PRIVATE_KEY_PEM_2048,
    PRIVATE_KEY_DER_2048,
    PUBLIC_KEY_PEM_2048,
    PUBLIC_KEY_DER_2048
)]
fn import_export(
    #[case] private_key_pem: &str,
    #[case] private_key_der: &[u8],
    #[case] public_key_pem: &str,
    #[case] public_key_der: &[u8],
) {
    let priv_key_pem = SequesterPrivateKeyDer::load_pem(private_key_pem).unwrap();
    let priv_key_der = SequesterPrivateKeyDer::try_from(private_key_der).unwrap();

    let pub_key_pem = SequesterPublicKeyDer::load_pem(public_key_pem).unwrap();
    let pub_key_der = SequesterPublicKeyDer::try_from(public_key_der).unwrap();

    let signing_key_pem = SequesterSigningKeyDer::load_pem(private_key_pem).unwrap();
    let signing_key_der = SequesterSigningKeyDer::try_from(private_key_der).unwrap();

    let verify_key_pem = SequesterVerifyKeyDer::load_pem(public_key_pem).unwrap();
    let verify_key_der = SequesterVerifyKeyDer::try_from(public_key_der).unwrap();

    assert_eq!(priv_key_pem, priv_key_der);
    assert_eq!(pub_key_pem, pub_key_der);

    // Also test roundtrip
    assert_eq!(
        SequesterPrivateKeyDer::load_pem(&priv_key_pem.dump_pem()).unwrap(),
        priv_key_pem
    );
    assert_eq!(
        SequesterPrivateKeyDer::try_from(&priv_key_der.dump()[..]).unwrap(),
        priv_key_der
    );

    assert_eq!(
        SequesterPublicKeyDer::load_pem(&pub_key_pem.dump_pem()).unwrap(),
        pub_key_pem
    );
    assert_eq!(
        SequesterPublicKeyDer::try_from(&pub_key_der.dump()[..]).unwrap(),
        pub_key_der
    );

    assert_eq!(
        SequesterSigningKeyDer::load_pem(&signing_key_pem.dump_pem()).unwrap(),
        signing_key_pem
    );
    assert_eq!(
        SequesterSigningKeyDer::try_from(&signing_key_der.dump()[..]).unwrap(),
        signing_key_der
    );

    assert_eq!(
        SequesterVerifyKeyDer::load_pem(&verify_key_pem.dump_pem()).unwrap(),
        verify_key_pem
    );
    assert_eq!(
        SequesterVerifyKeyDer::try_from(&verify_key_der.dump()[..]).unwrap(),
        verify_key_der
    );
}

#[test]
fn sign_compat() {
    let verify_key = SequesterVerifyKeyDer::try_from(PUBLIC_KEY_DER_1024).unwrap();

    let signed = hex!(
        "5253415353412d5053532d5348413235363a0afc141b03789ac3f2c69bd1e577e279d4570b"
        "f3fe387f389fe52c2b4ac08a9cacbd3c5c1b080cb39969cf3ff7a375619b4b5adc4aef2aec"
        "2800f6ead1d78019f8e37d036880b71e6ba4e89562e14ab2d6b35d0db4db48d818f8d4395f"
        "8d692be38fcdfa8d526a352bb811393dd987ed5a8257b7583d145099037178456baf3c4865"
        "6c6c6f20776f726c64"
    );

    assert_eq!(verify_key.verify(&signed).unwrap(), b"Hello world");
}

#[test]
fn encrypt_compat() {
    let priv_key = SequesterPrivateKeyDer::try_from(PRIVATE_KEY_DER_1024).unwrap();

    let encrypted = hex!(
        "52534145532d4f4145502d5853414c534132302d504f4c59313330353a278b9346743cc609"
        "258a4a82023059411d78c29aed9cf9893dc36b1f1f0055e3db8fa3624b4e3ced4fa1d3683b"
        "97cff2694ddbebeb9a59d1533e9b4dba005958f70a1b7b4b54fef420fb200146a73ac0f457"
        "168e71decb50d98af9332da36b5143e3470f7858a1a43f0f7ffff6e98e2487579d96a3791d"
        "69d48ba307e9984dad42781a567fa27e74e9ee88fd945736968855588eec48f43faa396464"
        "d9a5e8cfd5326ea0193ff5732b51423146d683b74870ed"
    );

    assert_eq!(priv_key.decrypt(&encrypted).unwrap(), b"Hello world");
}

#[test]
fn verify_with_different_salt_len() {
    let verify_key = SequesterVerifyKeyDer::try_from(PUBLIC_KEY_DER_1024).unwrap();
    let ciphered_salt32 = hex!(
        "5253415353412d5053532d5348413235363a" // `RSASSA-PSS-SHA256:` in hex
        "5533ef66fbd8249dcf2f813da9e381bde5d5d730231584e194e43b5a2cd2802a"
        "3a9f713939cf5398f7dd445a97ac9ae7aa3b871b296ff5defa1afe4ee8b1083d"
        "f0c7c2631f0efb400202d5800c0540e87c4c9b94ecc50d99095212baf35e0ea7"
        "9d3a56739867e31126b60d6f11b6448719b69562207a376ca5c5ccc0154318c9"
        "48656c6c6f20576f726c640a" // `Hello world\n` in hex
    );
    let ciphered_salt94 = hex!(
        "5253415353412d5053532d5348413235363a" // `RSASSA-PSS-SHA256:` in hex
        "320bb6b943bdf45fb39bc65b60f6353aa29f11c5990f664b555723cc910a1b07"
        "e95fe82f1ac7c519088d03347461fa7d168686e431f0e4666752ecbcd3d9c576"
        "98cc41b3280784c762effd2771ebc78e55351db499f0242a1e7067e275493d51"
        "3ac8c04fa8f26ca54633c75ec01a9a60eac2eedc74bbf2979a0140e24976e287"
        "48656c6c6f20576f726c640a" // `Hello world\n` in hex
    );
    let data = b"Hello World\n";

    assert_eq!(
        verify_key
            .verify(&ciphered_salt32)
            .expect("Cannot verify salt32"),
        data
    );
    assert_eq!(
        verify_key
            .verify(&ciphered_salt94)
            .expect("Cannot verify salt94"),
        data
    );
}

#[rstest]
#[case::empty(b"", CryptoError::Decryption)]
#[case::only_separator(b":", CryptoError::Algorithm("".into()))]
#[case::no_signature(b"RSASSA-PSS-SHA256:", CryptoError::DataSize)]
#[case::signature_too_small(b"RSASSA-PSS-SHA256:\0", CryptoError::DataSize)]
#[case::missing_separator(b"RSASSA-PSS-SHA256", CryptoError::Decryption)]
#[case::unknown_algorithm(b"ALGORITHM:", CryptoError::Algorithm("ALGORITHM".into()))]
fn invalid_signature(#[case] signed: &[u8], #[case] err: CryptoError) {
    let verify_key = SequesterVerifyKeyDer::try_from(PUBLIC_KEY_DER_1024).unwrap();

    let e = verify_key.verify(signed).unwrap_err();

    assert_eq!(e, err);
}
