// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use crate::{CryptoError, CryptoResult};

pub(crate) trait EnforceSerialize {
    const ALGORITHM: &'static [u8];
    const SIZE_IN_BITS: usize;
    const SIZE_IN_BYTES: usize = Self::SIZE_IN_BITS / 8;

    /// Here we avoid uncessary allocation & enforce output has `key_size`
    fn serialize(&self, output: &[u8], data: &[u8]) -> Vec<u8> {
        assert!(output.len() <= Self::SIZE_IN_BYTES);
        let mut res = vec![0; Self::ALGORITHM.len() + 1 + Self::SIZE_IN_BYTES + data.len()];

        let (algorithm_part, others) = res.split_at_mut(Self::ALGORITHM.len());
        let (colon, others) = others.split_at_mut(1);
        // Here we enforce output has key size with zeros
        let (_zeros, others) = others.split_at_mut(Self::SIZE_IN_BYTES - output.len());
        let (output_part, data_part) = others.split_at_mut(output.len());

        algorithm_part.copy_from_slice(Self::ALGORITHM);
        colon[0] = b':';
        output_part.copy_from_slice(output);
        data_part.copy_from_slice(data);

        res
    }
}

pub(crate) trait EnforceDeserialize {
    const ALGORITHM: &'static [u8];
    const SIZE_IN_BITS: usize;
    const SIZE_IN_BYTES: usize = Self::SIZE_IN_BITS / 8;

    fn deserialize(data: &[u8]) -> CryptoResult<(&[u8], &[u8])> {
        let index = data
            .iter()
            .position(|x| *x == b':')
            .ok_or(CryptoError::Decryption)?;
        let (algo, output_and_data) = data.split_at(index + 1);

        if &algo[..index] != Self::ALGORITHM {
            return Err(CryptoError::Algorithm(
                String::from_utf8_lossy(&algo[..index]).into(),
            ));
        }

        Ok(output_and_data.split_at(Self::SIZE_IN_BYTES))
    }
}

#[test]
fn test_key_equality() {
    use hex_literal::hex;

    let pub_key = SequesterPublicKeyDer::try_from(
        &hex!(
            "30819f300d06092a864886f70d010101050003818d0030818902818100a0b7653b73670b364cd397"
            "27a3843502d1ba0402e38f197d4b6ab2ea31da8b23cfef82fdc7bf723d0ae6093767aa92274dde3a"
            "b0f959f0624d4d5f4c7601a561a646e2f323e60f69e52f53158654f0a592ba0f19cebb3f7b2cbc6b"
            "7920670c6fe1b23c13a77597dc50d34dc1c454c25ee3348cfae7e089bcf50a266b4033f5f5020301"
            "0001"
        )[..],
    )
    .unwrap();

    let pub_key2 = pub_key.clone();

    assert_eq!(pub_key, pub_key2);
}

#[cfg(test)]
mod test {
    pub use crate::{
        SequesterPrivateKeyDer, SequesterPublicKeyDer, SequesterSigningKeyDer,
        SequesterVerifyKeyDer,
    };
    pub use hex_literal::hex;

    pub const PRIVATE_KEY_PEM: &str = r#"-----BEGIN PRIVATE KEY-----
MIICdgIBADANBgkqhkiG9w0BAQEFAASCAmAwggJcAgEAAoGBALLcAKPDtcaJsGnz
9AxJTSpb4xOxA0+/Hf4O7uDzbPvPYkQAJWzGYNUIR4JzijBF11tYTBlDvATHEj1o
rAzvJTtO6Neb0J2hkWLcwINmImm3tiyzhYL4owIZBHsIfBG2AYSwST4MHIsdEPnX
5qLrWv9m9+4YMDGV87zHKrVyB+v9AgMBAAECgYBBfQRO8g3QkAGkCcrF5OD4LYTL
ZPjNbjDRIS6d9wNkf95+/360gT5bQhjMzvk+C5R6wa27Ym2pYipvia/VXIrIuwoP
GDKyUg/B2SrDIBgLlleosVmOZwERnY939ShaxXA/TAqT4eXr5uwXm8z/YkledzTV
iZ2G0ty962SJI9KbEQJBAOsrXVrVvCjK4WVFBcFqKw2Cz6I3+PEPcOXnozMwKKre
XeT54rPoqfkkoFOPcRngiPPBHxJYMZxZ2gPWN6MkxmMCQQDCs8SRRd/XkwrH9Wga
+kO3sY9pcWNGnHq5psqOEhaOqxozzE5ztT9FCcSAUqMf8onnNXpU+I4o7lQwQACZ
dmIfAkAmtgiz/yLuBBd+OBJueC+GFdZf+Z6877HB5pNyxaasGdaS7p9mxhHUtTa/
ComvnMpudYfL2UCxYAkHQKf/7vnJAkEAk1Zqd+zCmWXikLK7Fz8vo4CwoAB4OeUM
UhVPzvcNLuV4LJ58976+pEXh96GRZAmsJdUoP8jf+0VvXBvy2C7nzQJAGLjFefMr
vXTMHxDHhuHg4ZJSbJ5BNMZb/HZ5m4KQCt9Gelx/sxZOt3VlCrquCFALxmkeYMhX
W1pav08hVpEcTg==
-----END PRIVATE KEY-----
"#;
    pub const PUBLIC_KEY_PEM: &str = r#"-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCy3ACjw7XGibBp8/QMSU0qW+MT
sQNPvx3+Du7g82z7z2JEACVsxmDVCEeCc4owRddbWEwZQ7wExxI9aKwM7yU7TujX
m9CdoZFi3MCDZiJpt7Yss4WC+KMCGQR7CHwRtgGEsEk+DByLHRD51+ai61r/Zvfu
GDAxlfO8xyq1cgfr/QIDAQAB
-----END PUBLIC KEY-----
"#;

    pub const PRIVATE_KEY_DER: [u8; 634] = hex!(
        "30820276020100300d06092a864886f70d0101010500048202603082025c02010002818100"
        "b2dc00a3c3b5c689b069f3f40c494d2a5be313b1034fbf1dfe0eeee0f36cfbcf624400256c"
        "c660d5084782738a3045d75b584c1943bc04c7123d68ac0cef253b4ee8d79bd09da19162dc"
        "c083662269b7b62cb38582f8a30219047b087c11b60184b0493e0c1c8b1d10f9d7e6a2eb5a"
        "ff66f7ee18303195f3bcc72ab57207ebfd0203010001028180417d044ef20dd09001a409ca"
        "c5e4e0f82d84cb64f8cd6e30d1212e9df703647fde7eff7eb4813e5b4218cccef93e0b947a"
        "c1adbb626da9622a6f89afd55c8ac8bb0a0f1832b2520fc1d92ac320180b9657a8b1598e67"
        "01119d8f77f5285ac5703f4c0a93e1e5ebe6ec179bccff62495e7734d5899d86d2dcbdeb64"
        "8923d29b11024100eb2b5d5ad5bc28cae1654505c16a2b0d82cfa237f8f10f70e5e7a33330"
        "28aade5de4f9e2b3e8a9f924a0538f7119e088f3c11f1258319c59da03d637a324c6630241"
        "00c2b3c49145dfd7930ac7f5681afa43b7b18f697163469c7ab9a6ca8e12168eab1a33cc4e"
        "73b53f4509c48052a31ff289e7357a54f88e28ee543040009976621f024026b608b3ff22ee"
        "04177e38126e782f8615d65ff99ebcefb1c1e69372c5a6ac19d692ee9f66c611d4b536bf0a"
        "89af9cca6e7587cbd940b160090740a7ffeef9c902410093566a77ecc29965e290b2bb173f"
        "2fa380b0a0007839e50c52154fcef70d2ee5782c9e7cf7bebea445e1f7a1916409ac25d528"
        "3fc8dffb456f5c1bf2d82ee7cd024018b8c579f32bbd74cc1f10c786e1e0e192526c9e4134"
        "c65bfc76799b82900adf467a5c7fb3164eb775650abaae08500bc6691e60c8575b5a5abf4f"
        "2156911c4e");

    pub const PUBLIC_KEY_DER: [u8; 162] = hex!(
        "30819f300d06092a864886f70d010101050003818d0030818902818100b2dc00a3c3b5c689"
        "b069f3f40c494d2a5be313b1034fbf1dfe0eeee0f36cfbcf624400256cc660d5084782738a"
        "3045d75b584c1943bc04c7123d68ac0cef253b4ee8d79bd09da19162dcc083662269b7b62c"
        "b38582f8a30219047b087c11b60184b0493e0c1c8b1d10f9d7e6a2eb5aff66f7ee18303195"
        "f3bcc72ab57207ebfd0203010001"
    );
}
#[cfg(test)]
use test::*;

#[test]
fn test_keys() {
    assert_eq!(SequesterPrivateKeyDer::SIZE_IN_BITS, 1024);

    let priv_key = SequesterPrivateKeyDer::try_from(&PRIVATE_KEY_DER[..]).unwrap();

    let pub_key = SequesterPublicKeyDer::from(priv_key.clone());

    assert_eq!(pub_key.dump(), PUBLIC_KEY_DER);

    let data = b"Hello world";

    let encrypted = pub_key.encrypt(data);

    assert_eq!(priv_key.decrypt(&encrypted).unwrap(), data);

    let signing_key = SequesterSigningKeyDer::from(priv_key);
    let verify_key = SequesterVerifyKeyDer::from(pub_key);

    let signed = signing_key.sign(data);

    assert_eq!(verify_key.verify(&signed).unwrap(), data);
}

#[test]
fn test_pub_signature_verification() {
    let pub_key_verifier = SequesterVerifyKeyDer::try_from(&PUBLIC_KEY_DER[..]).unwrap();

    let signed_data = hex!(
        "5253415353412d5053532d5348413235363a0afc141b03789ac3f2c69bd1e577e279d4570b"
        "f3fe387f389fe52c2b4ac08a9cacbd3c5c1b080cb39969cf3ff7a375619b4b5adc4aef2aec"
        "2800f6ead1d78019f8e37d036880b71e6ba4e89562e14ab2d6b35d0db4db48d818f8d4395f"
        "8d692be38fcdfa8d526a352bb811393dd987ed5a8257b7583d145099037178456baf3c4865"
        "6c6c6f20776f726c64"
    );

    let verification = pub_key_verifier.verify(&signed_data);

    assert!(verification.is_ok());
}

#[test]
fn test_priv_decipher_verification() {
    let priv_key = SequesterPrivateKeyDer::try_from(&PRIVATE_KEY_DER[..]).unwrap();

    let data = priv_key
        .decrypt(&hex!(
            "52534145532d4f4145502d5853414c534132302d504f4c59313330353a278b9346743cc609"
            "258a4a82023059411d78c29aed9cf9893dc36b1f1f0055e3db8fa3624b4e3ced4fa1d3683b"
            "97cff2694ddbebeb9a59d1533e9b4dba005958f70a1b7b4b54fef420fb200146a73ac0f457"
            "168e71decb50d98af9332da36b5143e3470f7858a1a43f0f7ffff6e98e2487579d96a3791d"
            "69d48ba307e9984dad42781a567fa27e74e9ee88fd945736968855588eec48f43faa396464"
            "d9a5e8cfd5326ea0193ff5732b51423146d683b74870ed"
        ))
        .unwrap();

    assert_eq!(data, b"Hello world");
}

#[test]
fn test_import_export() {
    let priv_key_pem = SequesterPrivateKeyDer::load_pem(PRIVATE_KEY_PEM).unwrap();
    let priv_key_der = SequesterPrivateKeyDer::try_from(&PRIVATE_KEY_DER[..]).unwrap();

    let pub_key_pem = SequesterPublicKeyDer::load_pem(PUBLIC_KEY_PEM).unwrap();
    let pub_key_der = SequesterPublicKeyDer::try_from(&PUBLIC_KEY_DER[..]).unwrap();

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
}
