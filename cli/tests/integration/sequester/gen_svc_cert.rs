use libparsec::{tmp_path, TmpPath};
use predicates::boolean::PredicateBooleanExt;

// Generated with `openssl genrsa 512`
const AUTHORITY_PRIVATE_KEY: &str = r"
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
