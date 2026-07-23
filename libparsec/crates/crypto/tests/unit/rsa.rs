// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use hex_literal::hex;

use crate::RsaPrivateKey;

// Pkcs8 rsa private key
const RSA_PKEY_PEM_512: &str = r#"-----BEGIN PRIVATE KEY-----
MIIBVAIBADANBgkqhkiG9w0BAQEFAASCAT4wggE6AgEAAkEAw8w02ltll115P8ZO
NTQ9a6431UQanFclMCSZljTajzbfx9DbKe03jQVWXTxiSxYE7/vhcSNTfSYCO5rP
8UHaSQIDAQABAkEAugyWxE5IsJYPmrwSoJetLV70iuAV8S0VlzOQBZzfeWjcWcV4
VNdIxUKWg0IFZTfmh3jNM6F1E8BalDUzPGJoKQIhAP2qMqM29sNeNRwI2p2InE8O
U8alrvzLPIIJQD/5M3BXAiEAxZmipk7d5apIIFMt6VVCyDoLAmH1bu+V7O9bFSeL
5l8CICikECDeOYLroQ6wzHXY4VI5NWrqOLL/zN34pXaacodZAiBWlmItmjWmJB4C
/DAMJS5kavrUCnTwLdB2yNQSyweE5QIgF57vYlm4l4AFy/uUeEBbOyIJ11UCR2dk
pSjq5trHDe4=
-----END PRIVATE KEY-----"#;

#[test]
fn test_sign_pkcs1v15_unprefixed() {
    let key = RsaPrivateKey::load_pkcs8_pem(RSA_PKEY_PEM_512).unwrap();
    let data = hex!("fb6fdfe5fa570e544ee91335613c30a7eb54a24e970d7c7108bdee392b4abe5f");

    let signature = key.sign_pkcs1v15_unprefixed(&data).unwrap();
    let expected_signature = hex!("222193e6b850b46b18abffb745cb856e284e15bbe9e28487f2b6f041274ccf847e18b4c749795cb0b71cb23f4972984e2a43f5bd8d6bef58db683eb406854d8b");
    assert_eq!(signature, expected_signature);
}
