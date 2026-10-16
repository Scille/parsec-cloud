use libparsec::{TmpPath, tmp_path};
use predicates::boolean::PredicateBooleanExt;

// Generated with `openssl genrsa 512`
const SMALL_AUTHORITY_PRIVATE_KEY: &str = r"
-----BEGIN PRIVATE KEY-----
MIIBVQIBADANBgkqhkiG9w0BAQEFAASCAT8wggE7AgEAAkEAshrD1K0SNdPKftGp
yEpOAB9hKrw5wK2ABJ79k4O0O4TzCsO9Ltsvii8iJiVHmWU5KkEG1WXUAui9Cppp
JN4pkwIDAQABAkAU65PpHVMwQ2pbryD0R9YVqZnuPSHDgh2xbUN32laHTwofYwN2
iRAszdfKU5QBChZ2SwtEzuQxAJM+6qO0FbvBAiEA6zfpP2qx+04h+7P+LKsVRkmR
WaSWUpG7coViGlOuZUMCIQDB1xKIFyYe9J5BxrV+aRaRBxRSaIqRNWy78CxQuMu9
cQIhAKzhIyXhHX8/JesBl8bs124ZlCL2vSVixwqczkXbS0pRAiB0W1I4dwzfEB/f
hBR2aUauj/1W6oIjYFqBBk7Ttdo3QQIhAOTIiNTbThZAWAGiFGKys7+nQqFiuQdr
tUX1mIwegVn/
-----END PRIVATE KEY-----
";

// Generated with `openssl genrsa 1024`
const AUTHORITY_PRIVATE_KEY: &str = r"
-----BEGIN PRIVATE KEY-----
MIICdgIBADANBgkqhkiG9w0BAQEFAASCAmAwggJcAgEAAoGBANfWLORzbNy+FCP1
E9u8NqdYvGZ8sRTspj5FRFLMqyRYWwpyoOaHbJms5SKRLe1MvCU08OV0u0tBXxZ8
ZXTGmHok0k3h84NYJdMsw5IJnPi3KLGZScoKMaJ8MLpYpe8prP28ZR5p+7T/9bHc
Dtf/oj1gMVkkcLXtxU8nMhyH8AmzAgMBAAECgYEAvV4wbQ/Ih7RNv26bScNMwG7e
fZ0RUSaAhUoItAzWa0EKP5RrD1fqNLq5Hyb5gKY+vPZCFU/htBla8EOcUGSeCY7Q
wBTl8wF4/n5zGu2k158U4ADG36mh5A5fV1vVnl/n1dePW46p9KaqYEMCS1vGjNqX
82nN5RTDoy8MIppPvwECQQD+ogmCfVlzy+jXHaiKhJPODhdNBw2w/N197DVVRPLe
eBF+OQ4Va2S9T76lUZg5RBezPAS/djZHNSBfllFpGF9TAkEA2P7RNuuBbJ+VSF+d
czM567OnnIeL7KMtB8xJDhPYokyZwXPWNihU40l5afs2fMGEUuDbyUDiMS7SpLTg
6E1AIQJAM53bOaV5TZznLTwo3zNy1tFKbeXhI6COsakBQrDAXtKOET01mqfzeuDA
b7izl+AywbWH7NvkstiVIGtTQ2+aWwJAPnZzYNy/37W8Mh4b45peVOPdHqjB9Xcv
Mixy5xN9a5KjO9WNVv+bF+YBhiljENxfq7/AZJlBwyiYSAMtLpF2IQJAeBf01rgq
vbWjXx97YxLoctYYb77ydh0NtjH/fUi5TMZhS1tCIv66BO39YLXEsTQYG9fcfq6I
GpEHjWmyAcWmeQ==
-----END PRIVATE KEY-----
";

// Generated with `openssl genrsa 512 | openssl rsa -pubout`
const SERVICE_PUBLIC_KEY: &str = r"
-----BEGIN PUBLIC KEY-----
MFwwDQYJKoZIhvcNAQEBBQADSwAwSAJBAMqLD2wRpKjpey5UX2WXB4W9en0vQeYf
nVR9y18Mppflc75cyfsQjpFTNPu/TPWMr84lStCxs1EkAIEy6222XbcCAwEAAQ==
-----END PUBLIC KEY-----
";

#[rstest::rstest]
#[tokio::test]
async fn test_create_sequester_service_certificate(tmp_path: TmpPath) {
    let pkey_path = tmp_path.join("pkey.pem");
    let pubkey_path = tmp_path.join("pubkey.pem");

    tokio::try_join!(
        tokio::fs::write(&pkey_path, AUTHORITY_PRIVATE_KEY),
        tokio::fs::write(&pubkey_path, SERVICE_PUBLIC_KEY)
    )
    .unwrap();

    crate::assert_cmd_success!(
        "sequester",
        "generate-service-certificate",
        "--service-label=foobar",
        "--service-public-key",
        &pubkey_path,
        "--authority-private-key",
        &pkey_path,
        "-"
    )
    .stdout(
        predicates::str::contains("BEGIN PARSEC SEQUESTER SERVICE CERTIFICATE").and(
            predicates::str::contains("END PARSEC SEQUESTER SERVICE CERTIFICATE"),
        ),
    );
}

#[rstest::rstest]
#[tokio::test]
async fn test_too_small_authority_key(tmp_path: TmpPath) {
    let pkey_path = tmp_path.join("pkey.pem");
    let pubkey_path = tmp_path.join("pubkey.pem");

    tokio::try_join!(
        tokio::fs::write(&pkey_path, SMALL_AUTHORITY_PRIVATE_KEY),
        tokio::fs::write(&pubkey_path, SERVICE_PUBLIC_KEY)
    )
    .unwrap();

    crate::assert_cmd_failure!(
        "sequester",
        "generate-service-certificate",
        "--service-label=foobar",
        "--service-public-key",
        &pubkey_path,
        "--authority-private-key",
        &pkey_path,
        "-"
    )
    .stderr(predicates::str::contains(
        "The authority key is too small (need at least 66, actual: 64)",
    ));
}
