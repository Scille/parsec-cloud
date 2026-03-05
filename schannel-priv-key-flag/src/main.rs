use windows_sys::Win32::Security::Cryptography::{
    AT_KEYEXCHANGE, AT_SIGNATURE, CERT_NCRYPT_KEY_SPEC,
};

fn main() {
    if AT_KEYEXCHANGE & CERT_NCRYPT_KEY_SPEC != 0 {
        println!("AT_KEYEXCHANGE is included in CERT_NCRYPT_KEY_SPEC");
    }
    if AT_SIGNATURE & CERT_NCRYPT_KEY_SPEC != 0 {
        println!("AT_SIGNATURE is included in CERT_NCRYPT_KEY_SPEC");
    }
}
