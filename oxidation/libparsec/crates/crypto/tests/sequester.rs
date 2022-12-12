// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use hex_literal::hex;
use rstest::rstest;

use libparsec_crypto::{
    SequesterPrivateKeyDer, SequesterPublicKeyDer, SequesterSigningKeyDer, SequesterVerifyKeyDer,
};

const PRIVATE_KEY_PEM_1024: &str = r#"-----BEGIN PRIVATE KEY-----
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

const PRIVATE_KEY_DER_1024: [u8; 634] = hex!(
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

const PUBLIC_KEY_PEM_1024: &str = r#"-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCy3ACjw7XGibBp8/QMSU0qW+MT
sQNPvx3+Du7g82z7z2JEACVsxmDVCEeCc4owRddbWEwZQ7wExxI9aKwM7yU7TujX
m9CdoZFi3MCDZiJpt7Yss4WC+KMCGQR7CHwRtgGEsEk+DByLHRD51+ai61r/Zvfu
GDAxlfO8xyq1cgfr/QIDAQAB
-----END PUBLIC KEY-----
"#;

const PUBLIC_KEY_DER_1024: [u8; 162] = hex!(
    "30819f300d06092a864886f70d010101050003818d0030818902818100b2dc00a3c3b5c689"
    "b069f3f40c494d2a5be313b1034fbf1dfe0eeee0f36cfbcf624400256cc660d5084782738a"
    "3045d75b584c1943bc04c7123d68ac0cef253b4ee8d79bd09da19162dcc083662269b7b62c"
    "b38582f8a30219047b087c11b60184b0493e0c1c8b1d10f9d7e6a2eb5aff66f7ee18303195"
    "f3bcc72ab57207ebfd0203010001"
);

const PRIVATE_KEY_PEM_2048: &str = r#"-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCnDVsZGa7iu45E
2d++JupEgmdlo8EGFjfXJzVgXduWT42rmxGEKCARzHt5RK0EjSsfmLg4uQqvVlEc
lUtN4HR466nU6tqnf+PLjYJNVQQVh1rRM27N97gfRxuOVwkb6tAMIFUytWRyQd9C
FEzMpLJebFeZc/dWQ76gOXhBY71L5/7O+m8xugwIpex8qvao+aVgHaQqGNHJN+CB
Zys2/GzMO/6ypbbhy6nr/OSknvc00I7XiQDynq77fciZNuqPIa7Za+N14rIrUsyK
bCEc8RH0sn2wcdieVXHPdvF3ZasFbD5tNJOHo//SplOLEwhdYAGs7NEcjEOmXksk
8t0gCJcxAgMBAAECggEAfJIqF6qeTnd8XP13qo3MbnAr+JBHdWmGFIwpcoYrJIiI
ZaqKQlnFqGAqx0CeIOHAgZxZ6/qY7C1a6QyvjhBN6ooaKCtGCFgHH5iTzWUTWZaI
863999u9Up2s/9UJducAY7qMrfa0Q7u5Z8ZJiuVaGa2DGWxvycVU1phkg6aD16Cg
nZ3+9oWbaBfELPEFdktYwa5fwflOMN6+Fw7isIVlOC4tvGpiR1scXKEwjr2P5a0P
D0s6ca4D9xlCrlJ6CpdyKbLwz5ya+gBwjW3OJYI8+NzIw5SwmnnZoP3MqI96ME70
RGBfyqPs4DZjEaqWSSiaDMu10nndEvdakFwgCBzCAQKBgQDeIAouUgnG53RCjukN
mLTHLDpuQk7yycmVcjqWgq2Dkg2FxGbl7f9p6vLDicbMieXYbSTbt5rYTnU6c5j0
o2uAiGsUK8E7JsD8qid7XV7M6r0mTCAqP16SEcf9oSureYFXIWq6pkgnQLS6LVw6
rWwG+uaaCw3qjQvlPk17pvKGYQKBgQDAhzf5EJG3LICmgSKsz6qS/G82N8gxqCwC
Gf8egXGweQSSYrUaemtsAHHbVNIxEqPoedpj+ErofvYo2exNbQVyp33rC1kGNqqT
YTzsV83jp0vv9cn6LwgeK+5mfuNTs13H/YfKpdfJRgoo78Ot7c//FpcM4iwtcFEm
CDhD7DAi0QKBgCSBmYrBWvAAzD/AIxdj+Jofb779UOJgPaw9KNnhwki0cVqBy/OQ
KQEsZxeyBiVJqKfoUD14MI/KXUPtCb9zOFDYmtim1Ew3c1JTWMfSsaj2D35C1qp6
3b0eBQNvQLEe+B3s1RC2EXe6D7nliJnwNKf1Bn9cv73DzqevlKZ06rkhAoGBAJAF
VuCw/WAaIU7s8QR3AHGd9o+HYPGMjQcxbT/jsylBDiN3RWS+SRdsqb9aDzZ4bDae
2LjJ5zdOqlPa2UL+GAIxuycVwQ293RIMvmdKM5Fl949UvTey8Q6o7Fit+N4HIlNV
m7wDHY3KIBWJbkbwzBRVUL9pfYSsLXgW1AgzabrRAoGAUW82mYl9HAwQOBPb40OK
j2qKisC4VA8vR/Uco9yda5XT7iqG3htvUa+Ugj5r4XFRJvOyQG2WUiOWwGYlzfz2
1Ogu8O8RHcaJafAIMTJJtjsO8pIf6Y7aWHA3Ze3to+01fTkGB2I8tDKc3fn+iKlX
z0mxbH0U7CoeRzfgJ6UmIio=
-----END PRIVATE KEY-----
"#;

const PRIVATE_KEY_DER_2048: [u8; 1217] = hex!(
    "308204bd020100300d06092a864886f70d0101010500048204a7308204a302010002820101"
    "00a70d5b1919aee2bb8e44d9dfbe26ea44826765a3c1061637d72735605ddb964f8dab9b11"
    "84282011cc7b7944ad048d2b1f98b838b90aaf56511c954b4de07478eba9d4eadaa77fe3cb"
    "8d824d550415875ad1336ecdf7b81f471b8e57091bead00c205532b5647241df42144ccca4"
    "b25e6c579973f75643bea039784163bd4be7fecefa6f31ba0c08a5ec7caaf6a8f9a5601da4"
    "2a18d1c937e081672b36fc6ccc3bfeb2a5b6e1cba9ebfce4a49ef734d08ed78900f29eaefb"
    "7dc89936ea8f21aed96be375e2b22b52cc8a6c211cf111f4b27db071d89e5571cf76f17765"
    "ab056c3e6d349387a3ffd2a6538b13085d6001acecd11c8c43a65e4b24f2dd200897310203"
    "010001028201007c922a17aa9e4e777c5cfd77aa8dcc6e702bf89047756986148c2972862b"
    "24888865aa8a4259c5a8602ac7409e20e1c0819c59ebfa98ec2d5ae90caf8e104dea8a1a28"
    "2b460858071f9893cd6513599688f3adfdf7dbbd529dacffd50976e70063ba8cadf6b443bb"
    "b967c6498ae55a19ad83196c6fc9c554d6986483a683d7a0a09d9dfef6859b6817c42cf105"
    "764b58c1ae5fc1f94e30debe170ee2b08565382e2dbc6a62475b1c5ca1308ebd8fe5ad0f0f"
    "4b3a71ae03f71942ae527a0a977229b2f0cf9c9afa00708d6dce25823cf8dcc8c394b09a79"
    "d9a0fdcca88f7a304ef444605fcaa3ece0366311aa9649289a0ccbb5d279dd12f75a905c20"
    "081cc20102818100de200a2e5209c6e774428ee90d98b4c72c3a6e424ef2c9c995723a9682"
    "ad83920d85c466e5edff69eaf2c389c6cc89e5d86d24dbb79ad84e753a7398f4a36b80886b"
    "142bc13b26c0fcaa277b5d5ecceabd264c202a3f5e9211c7fda12bab798157216abaa64827"
    "40b4ba2d5c3aad6c06fae69a0b0dea8d0be53e4d7ba6f2866102818100c08737f91091b72c"
    "80a68122accfaa92fc6f3637c831a82c0219ff1e8171b079049262b51a7a6b6c0071db54d2"
    "3112a3e879da63f84ae87ef628d9ec4d6d0572a77deb0b590636aa93613cec57cde3a74bef"
    "f5c9fa2f081e2bee667ee353b35dc7fd87caa5d7c9460a28efc3adedcfff16970ce22c2d70"
    "5126083843ec3022d10281802481998ac15af000cc3fc0231763f89a1f6fbefd50e2603dac"
    "3d28d9e1c248b4715a81cbf39029012c6717b2062549a8a7e8503d78308fca5d43ed09bf73"
    "3850d89ad8a6d44c3773525358c7d2b1a8f60f7e42d6aa7addbd1e05036f40b11ef81decd5"
    "10b61177ba0fb9e58899f034a7f5067f5cbfbdc3cea7af94a674eab92102818100900556e0"
    "b0fd601a214eecf1047700719df68f8760f18c8d07316d3fe3b329410e23774564be49176c"
    "a9bf5a0f36786c369ed8b8c9e7374eaa53dad942fe180231bb2715c10dbddd120cbe674a33"
    "9165f78f54bd37b2f10ea8ec58adf8de072253559bbc031d8dca2015896e46f0cc145550bf"
    "697d84ac2d7816d4083369bad1028180516f3699897d1c0c103813dbe3438a8f6a8a8ac0b8"
    "540f2f47f51ca3dc9d6b95d3ee2a86de1b6f51af94823e6be1715126f3b2406d96522396c0"
    "6625cdfcf6d4e82ef0ef111dc68969f008313249b63b0ef2921fe98eda58703765ededa3ed"
    "357d390607623cb4329cddf9fe88a957cf49b16c7d14ec2a1e4737e027a526222a");

const PUBLIC_KEY_PEM_2048: &str = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEApw1bGRmu4ruORNnfvibq
RIJnZaPBBhY31yc1YF3blk+Nq5sRhCggEcx7eUStBI0rH5i4OLkKr1ZRHJVLTeB0
eOup1Orap3/jy42CTVUEFYda0TNuzfe4H0cbjlcJG+rQDCBVMrVkckHfQhRMzKSy
XmxXmXP3VkO+oDl4QWO9S+f+zvpvMboMCKXsfKr2qPmlYB2kKhjRyTfggWcrNvxs
zDv+sqW24cup6/zkpJ73NNCO14kA8p6u+33ImTbqjyGu2WvjdeKyK1LMimwhHPER
9LJ9sHHYnlVxz3bxd2WrBWw+bTSTh6P/0qZTixMIXWABrOzRHIxDpl5LJPLdIAiX
MQIDAQAB
-----END PUBLIC KEY-----
"#;

const PUBLIC_KEY_DER_2048: [u8; 294] = hex!(
    "30820122300d06092a864886f70d01010105000382010f003082010a0282010100a70d5b19"
    "19aee2bb8e44d9dfbe26ea44826765a3c1061637d72735605ddb964f8dab9b1184282011cc"
    "7b7944ad048d2b1f98b838b90aaf56511c954b4de07478eba9d4eadaa77fe3cb8d824d5504"
    "15875ad1336ecdf7b81f471b8e57091bead00c205532b5647241df42144ccca4b25e6c5799"
    "73f75643bea039784163bd4be7fecefa6f31ba0c08a5ec7caaf6a8f9a5601da42a18d1c937"
    "e081672b36fc6ccc3bfeb2a5b6e1cba9ebfce4a49ef734d08ed78900f29eaefb7dc89936ea"
    "8f21aed96be375e2b22b52cc8a6c211cf111f4b27db071d89e5571cf76f17765ab056c3e6d"
    "349387a3ffd2a6538b13085d6001acecd11c8c43a65e4b24f2dd200897310203010001");

#[test]
fn only_rsa_is_supported() {
    let unsupported_key = hex!(
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

    assert!(SequesterPublicKeyDer::try_from(&unsupported_key[..]).is_err());
    assert!(SequesterVerifyKeyDer::try_from(&unsupported_key[..]).is_err());
}

#[rstest]
#[case(&PUBLIC_KEY_DER_1024[..])]
#[case(&PUBLIC_KEY_DER_2048[..])]
fn test_key_equality(#[case] pub_key: &[u8]) {
    let pub_key = SequesterPublicKeyDer::try_from(pub_key).unwrap();

    let pub_key2 = pub_key.clone();

    assert_eq!(pub_key, pub_key2);
}

#[rstest]
#[case(1024)]
#[case(2048)]
fn test_size(#[case] size_in_bits: usize) {
    let (priv_key, pub_key) = SequesterPrivateKeyDer::generate_pair(size_in_bits);
    let (signing_key, verify_key) = SequesterSigningKeyDer::generate_pair(size_in_bits);
    assert_eq!(priv_key.size_in_bytes(), size_in_bits / 8);
    assert_eq!(pub_key.size_in_bytes(), size_in_bits / 8);
    assert_eq!(signing_key.size_in_bytes(), size_in_bits / 8);
    assert_eq!(verify_key.size_in_bytes(), size_in_bits / 8);
}

#[rstest]
#[case(1024)]
#[case(2048)]
fn test_encrypt_decrypt(#[case] size_in_bits: usize) {
    let (priv_key, pub_key) = SequesterPrivateKeyDer::generate_pair(size_in_bits);

    let encrypted = pub_key.encrypt(b"foo");

    assert_eq!(priv_key.decrypt(&encrypted).unwrap(), b"foo");
}

#[rstest]
#[case(1024)]
#[case(2048)]
fn test_sign_verify(#[case] size_in_bits: usize) {
    let (signing_key, verify_key) = SequesterSigningKeyDer::generate_pair(size_in_bits);

    let signed = signing_key.sign(b"foo");

    assert_eq!(verify_key.verify(&signed).unwrap(), b"foo");
}

#[rstest]
#[case(PRIVATE_KEY_PEM_1024, &PRIVATE_KEY_DER_1024[..], PUBLIC_KEY_PEM_1024, &PUBLIC_KEY_DER_1024[..])]
#[case(PRIVATE_KEY_PEM_2048, &PRIVATE_KEY_DER_2048[..], PUBLIC_KEY_PEM_2048, &PUBLIC_KEY_DER_2048[..])]
fn test_import_export(
    #[case] private_key_pem: &str,
    #[case] private_key_der: &[u8],
    #[case] public_key_pem: &str,
    #[case] public_key_der: &[u8],
) {
    println!("{private_key_der:02x?}");
    println!("\n---\n");
    println!("{public_key_der:02x?}");
    let priv_key_pem = SequesterPrivateKeyDer::load_pem(private_key_pem).unwrap();
    let priv_key_der = SequesterPrivateKeyDer::try_from(private_key_der).unwrap();

    let pub_key_pem = SequesterPublicKeyDer::load_pem(public_key_pem).unwrap();
    let pub_key_der = SequesterPublicKeyDer::try_from(public_key_der).unwrap();

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

#[test]
fn test_sign_compat() {
    let verify_key = SequesterVerifyKeyDer::try_from(&PUBLIC_KEY_DER_1024[..]).unwrap();

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
fn test_encrypt_compat() {
    let priv_key = SequesterPrivateKeyDer::try_from(&PRIVATE_KEY_DER_1024[..]).unwrap();

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
